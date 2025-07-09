/// GPU Module para Emulador Game Boy
#[derive(Debug, Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8);

pub const SCREEN_W: usize = 160;
pub const SCREEN_H: usize = 144;

/// Paleta com 4 cores
pub struct Palette(pub [Color; 4]);

impl Palette {
    pub fn from_u8(val: u8) -> Self {
        let mut colors = [Color(0, 0, 0); 4];
        for i in 0..4 {
            colors[i] = match (val >> (i * 2)) & 0x03 {
                0 => Color(255, 255, 255),
                1 => Color(192, 192, 192),
                2 => Color(96, 96, 96),
                3 => Color(0, 0, 0),
                _ => unreachable!(),
            };
        }
        Palette(colors)
    }
}

/// Framebuffer da tela (160x144), com 3 bytes por pixel (RGB).
pub struct FrameBuffer {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
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
        if x < self.width && y < self.height {
            let index = (y * self.width + x) * 3;
            self.data[index] = color.0;
            self.data[index + 1] = color.1;
            self.data[index + 2] = color.2;
        }
    }
}

/// Estrutura de uma linha da tela
pub struct Scanline {
    pub y: u8,
    pub pixels: [Color; SCREEN_W],
}

impl Scanline {
    pub fn new(y: u8) -> Self {
        Scanline {
            y,
            pixels: [Color(255, 255, 255); SCREEN_W],
        }
    }
}

/// Modos da GPU
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GPUMode {
    HBlank,
    VBlank,
    OAM,
    Transfer,
}

/// GPU principal
pub struct GPU {
    pub framebuffer: FrameBuffer,
    pub current_line: u8,
    pub mode: GPUMode,
    pub mode_clock: u32,
}

impl GPU {
    pub fn new() -> Self {
        GPU {
            framebuffer: FrameBuffer::new(SCREEN_W, SCREEN_H),
            current_line: 0,
            mode: GPUMode::OAM,
            mode_clock: 0,
        }
    }

    pub fn step(&mut self, ticks: u32) {
        self.mode_clock += ticks;
        match self.mode {
            GPUMode::OAM => self.step_oam(),
            GPUMode::Transfer => self.step_transfer(),
            GPUMode::HBlank => self.step_hblank(),
            GPUMode::VBlank => self.step_vblank(),
        }
    }

    fn step_oam(&mut self) {
        if self.mode_clock >= 80 {
            self.mode_clock = 0;
            self.mode = GPUMode::Transfer;
        }
    }

    fn step_transfer(&mut self) {
        if self.mode_clock >= 172 {
            // Aqui entraria o código de renderização da linha
            let mut scanline = Scanline::new(self.current_line);
            self.render_scanline(&mut scanline);
            for x in 0..SCREEN_W {
                self.framebuffer.set_pixel(x, self.current_line as usize, scanline.pixels[x]);
            }
            self.mode_clock = 0;
            self.mode = GPUMode::HBlank;
        }
    }

    fn step_hblank(&mut self) {
        if self.mode_clock >= 204 {
            self.mode_clock = 0;
            self.current_line += 1;
            if self.current_line == 144 {
                self.mode = GPUMode::VBlank;
            } else {
                self.mode = GPUMode::OAM;
            }
        }
    }

    fn step_vblank(&mut self) {
        if self.mode_clock >= 456 {
            self.mode_clock = 0;
            self.current_line += 1;
            if self.current_line >= 154 {
                self.current_line = 0;
                self.mode = GPUMode::OAM;
            }
        }
    }

    fn render_scanline(&self, scanline: &mut Scanline) {
        // Placeholder: apenas preenche com um gradiente de cinza
        for x in 0..SCREEN_W {
            let c = ((x + scanline.y as usize) % 256) as u8;
            scanline.pixels[x] = Color(c, c, c);
        }
    }
}
