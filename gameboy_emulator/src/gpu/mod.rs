pub const SCREEN_W: usize = 160;
pub const SCREEN_H: usize = 144;

#[derive(Debug, PartialEq)]
pub enum GPUMode {
    HBlank,
    VBlank,
    OAM,
    Transfer,
}

pub struct Scanline {
    pub data: Vec<u8>,
    pub width: usize,
}

pub struct FrameBuffer {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Debug)]
pub struct Palette(pub [Color; 4]);

pub struct GPURegisters {
    pub lcdc: u8,
    pub stat: u8,
    pub scy: u8,
    pub scx: u8,
    pub ly: u8,
    pub lyc: u8,
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
    pub wy: u8,
    pub wx: u8,
}

pub struct GPU {
    pub framebuffer: FrameBuffer,
    pub vram: [u8; 0x2000],
    pub oam: [u8; 0xA0],
    pub current_line: u8,
    pub mode: GPUMode,
    pub mode_clock: u32,
    pub registers: GPURegisters,
    pub palette: Palette,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        FrameBuffer {
            data: vec![0; width * height * 3],
            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = (y * self.width + x) * 3;
        if index + 2 < self.data.len() {
            self.data[index] = color.0;
            self.data[index + 1] = color.1;
            self.data[index + 2] = color.2;
        }
    }
}

impl Scanline {
    pub fn new(width: usize) -> Self {
        Scanline {
            data: vec![0; width * 3],
            width,
        }
    }
}

impl GPU {
    pub fn new() -> Self {
        GPU {
            framebuffer: FrameBuffer::new(160, 144),
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            current_line: 0,
            mode: GPUMode::OAM,
            mode_clock: 0,
            registers: GPURegisters {
                lcdc: 0x91,
                stat: 0x00,
                scy: 0,
                scx: 0,
                ly: 0,
                lyc: 0,
                bgp: 0xFC,
                obp0: 0xFF,
                obp1: 0xFF,
                wy: 0,
                wx: 0,
            },
            palette: Palette([
                Color(255, 255, 255), // White
                Color(192, 192, 192), // Light gray
                Color(96, 96, 96),    // Dark gray
                Color(0, 0, 0),       // Black
            ]),
        }
    }

    pub fn step(&mut self, cycles: u32) -> u8 {
        self.mode_clock += cycles;

        match self.mode {
            GPUMode::OAM => {
                if self.mode_clock >= 80 {
                    self.mode_clock = 0;
                    self.mode = GPUMode::Transfer;
                }
            }
            GPUMode::Transfer => {
                if self.mode_clock >= 172 {
                    self.mode_clock = 0;
                    self.mode = GPUMode::HBlank;
                    self.render_scanline();
                }
            }
            GPUMode::HBlank => {
                if self.mode_clock >= 204 {
                    self.mode_clock = 0;
                    self.current_line += 1;
                    self.registers.ly = self.current_line;

                    if self.current_line == 144 {
                        self.mode = GPUMode::VBlank;
                    } else {
                        self.mode = GPUMode::OAM;
                    }
                }
            }
            GPUMode::VBlank => {
                if self.mode_clock >= 456 {
                    self.mode_clock = 0;
                    self.current_line += 1;
                    self.registers.ly = self.current_line;

                    if self.current_line > 153 {
                        self.mode = GPUMode::OAM;
                        self.current_line = 0;
                        self.registers.ly = 0;
                    }
                }
            }
        }
        self.update_stat();
        self.check_interrupts()
    }

    pub fn set_palette(&mut self, colors: [Color; 4]) {
        self.palette = Palette(colors);
    }

    pub fn write_oam(&mut self, address: usize, value: u8) {
        if address < 0xA0 && self.mode != GPUMode::Transfer {
            self.oam[address] = value;
        }
    }

    pub fn read_oam(&self, address: usize) -> Option<u8> {
        if address < 0xA0 && self.mode != GPUMode::Transfer {
            Some(self.oam[address])
        } else {
            None
        }
    }

    fn render_scanline(&mut self) {
        let ly = self.registers.ly as usize;
        let scy = self.registers.scy as usize;
        let scx = self.registers.scx as usize;

        let lcdc = self.registers.lcdc;
        if lcdc & 0x80 == 0 {
            return; // LCD disabled
        }

        if lcdc & 0x01 != 0 { // Render background only if enabled
            let tile_map_base = if lcdc & 0x08 != 0 { 0x1C00 } else { 0x1800 };
            let signed = lcdc & 0x10 == 0;

            for tile_x in 0..(SCREEN_W / 8 + 1) {
                let pixel_x = (tile_x * 8 + scx) % 256;
                let pixel_y = (ly + scy) % 256;
                let tile_y = pixel_y / 8;
                let tile_index_addr = tile_map_base + tile_y * 32 + (pixel_x / 8);
                if tile_index_addr >= self.vram.len() {
                    println!("Warning: Invalid tile map address 0x{:04X}", tile_index_addr);
                    continue;
                }
                let tile_index = self.vram[tile_index_addr];
                self.render_tile(tile_index, tile_x, tile_y, signed);
            }
        }

        self.render_window();
        self.render_sprites();
    }

    fn render_tile(&mut self, tile_index: u8, tile_x: usize, tile_y: usize, signed: bool) {
        let line = self.registers.ly as usize % 8;
        let (low, high) = self.read_tile_line(tile_index, line as u8, signed);
        for bit in 0..8 {
            let x = tile_x * 8 + bit;
            if x < SCREEN_W {
                let color_index = ((high >> (7 - bit)) & 1) << 1 | ((low >> (7 - bit)) & 1);
                let color = get_color_from_palette(color_index, self.registers.bgp, &self.palette);
                self.framebuffer.set_pixel(x, self.registers.ly as usize, color);
            }
        }
    }

    fn render_window(&mut self) {
        if self.registers.lcdc & 0x20 == 0 {
            return; // Window disabled
        }
        let wy = self.registers.wy as usize;
        let wx = self.registers.wx as usize;
        if self.registers.ly < wy || wx >= SCREEN_W + 7 {
            return; // Window not visible on this scanline
        }

        let tile_map_base = if self.registers.lcdc & 0x40 != 0 { 0x1C00 } else { 0x1800 };
        let signed = self.registers.lcdc & 0x10 == 0;
        let line = self.registers.ly - wy;

        for x in 0..SCREEN_W {
            if x + 7 < wx {
                continue; // Skip pixels before window start
            }
            let pixel_x = x + 7 - wx;
            let pixel_y = line;

            let tile_x = pixel_x / 8;
            let tile_y = pixel_y / 8;
            let tile_index_addr = tile_map_base + tile_y * 32 + tile_x;

            if tile_index_addr >= self.vram.len() {
                println!("Warning: Invalid window tile map address 0x{:04X}", tile_index_addr);
                continue;
            }
            let tile_index = self.vram[tile_index_addr];
            let line_offset = pixel_y % 8;

            let (low, high) = self.read_tile_line(tile_index, line_offset as u8, signed);
            let bit = 7 - (pixel_x % 8);
            let color_index = ((high >> bit) & 1) << 1 | ((low >> bit) & 1);

            let color = get_color_from_palette(color_index, self.registers.bgp, &self.palette);
            self.framebuffer.set_pixel(x, self.registers.ly as usize, color);
        }
    }

    fn render_sprites(&mut self) {
        if self.registers.lcdc & 0x02 == 0 {
            return; // Sprites disabled
        }
        let sprite_height = if self.registers.lcdc & 0x04 != 0 { 16 } else { 8 };
        let mut sprites_drawn = 0;

        for i in (0..160).step_by(4) {
            let y_pos = self.oam[i] as i32 - 16;
            let x_pos = self.oam[i + 1] as i32 - 8;
            let tile_index = self.oam[i + 2];
            let attributes = self.oam[i + 3];

            if self.registers.ly as i32 >= y_pos && self.registers.ly as i32 < y_pos + sprite_height as i32 {
                let line = if attributes & 0x40 != 0 {
                    sprite_height - 1 - (self.registers.ly as i32 - y_pos) as usize // Vertical flip
                } else {
                    (self.registers.ly as i32 - y_pos) as usize // Normal
                } as u8;
                let (low, high) = self.read_tile_line(tile_index, line, false); // Sprites use unsigned indexing
                let palette = if attributes & 0x10 != 0 { self.registers.obp1 } else { self.registers.obp0 };

                for bit in 0..8 {
                    let x = x_pos + (if attributes & 0x20 != 0 { 7 - bit } else { bit }) as i32;
                    if x >= 0 && x < SCREEN_W as i32 && sprites_drawn < 10 {
                        let color_index = ((high >> (7 - bit)) & 1) << 1 | ((low >> (7 - bit)) & 1);
                        if color_index != 0 { // Skip transparent pixels
                            let color = get_color_from_palette(color_index, palette, &self.palette);
                            self.framebuffer.set_pixel(x as usize, self.registers.ly as usize, color);
                        }
                    }
                }
                sprites_drawn += 1;
            }
            if sprites_drawn >= 10 {
                break; // Max 10 sprites per scanline
            }
        }
    }

    fn read_tile_line(&self, tile_index: u8, y: u8, signed_index: bool) -> (u8, u8) {
        if self.mode == GPUMode::Transfer {
            println!("Warning: Attempted VRAM access during Transfer mode");
            return (0, 0);
        }
        let base_address = if signed_index {
            let signed = tile_index as i8 as i16;
            (0x1000i16 + signed * 16) as usize
        } else {
            (tile_index as usize) * 16
        };

        let line_offset = (y as usize) * 2;
        let address = base_address + line_offset;

        if address + 1 >= self.vram.len() {
            println!("Warning: Invalid VRAM address 0x{:04X}", address);
            return (0, 0);
        }

        let byte1 = self.vram[address];
        let byte2 = self.vram[address + 1];
        (byte1, byte2)
    }

    fn update_stat(&mut self) {
        self.registers.stat = (self.registers.stat & 0xFC) | match self.mode {
            GPUMode::HBlank => 0,
            GPUMode::VBlank => 1,
            GPUMode::OAM => 2,
            GPUMode::Transfer => 3,
        };
        if self.registers.ly == self.registers.lyc {
            self.registers.stat |= 0x04;
        } else {
            self.registers.stat &= !0x04;
        }
    }

    pub fn check_interrupts(&self) -> u8 {
        let mut interrupts = 0;
        if self.mode == GPUMode::VBlank && self.registers.stat & 0x10 != 0 {
            interrupts |= 0x01; // VBlank interrupt
        }
        if self.registers.ly == self.registers.lyc && self.registers.stat & 0x40 != 0 {
            interrupts |= 0x02; // LY=LYC interrupt
        }
        if self.mode == GPUMode::HBlank && self.registers.stat & 0x08 != 0 {
            interrupts |= 0x02; // HBlank interrupt
        }
        if self.mode == GPUMode::OAM && self.registers.stat & 0x20 != 0 {
            interrupts |= 0x02; // OAM interrupt
        }
        interrupts
    }
}

fn get_color_from_palette(index: u8, palette: u8, gpu_palette: &Palette) -> Color {
    let shade = (palette >> (index * 2)) & 0b11;
    gpu_palette.0[shade as usize]
}