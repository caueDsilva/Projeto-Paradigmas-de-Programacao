use crate::joypad::Joypad;

#[derive(Default)]
pub struct Mmu {
    pub rom: Vec<u8>,
    pub vram: [u8; 0x2000],
    pub eram: [u8; 0x2000],
    pub wram: [u8; 0x2000],
    pub oam: [u8; 0xA0],
    pub io: [u8; 0x80],
    pub hram: [u8; 0x7F],
    pub interrupt_enable: u8,

    pub rom_bank: u8,
    pub ram_bank: u8,
    pub ram_enabled: bool,
    pub mbc_type: u8,

    pub bios: Option<Vec<u8>>,
    pub in_bios: bool,

    pub joypad: Joypad,
}

impl Mmu {
    pub fn new(joypad: Joypad) -> Self {
        Self {
            rom: vec![0; 0x8000],
            joypad,
            in_bios: true,
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        self.vram = [0; 0x2000];
        self.eram = [0; 0x2000];
        self.wram = [0; 0x2000];
        self.oam = [0; 0xA0];
        self.io = [0; 0x80];
        self.hram = [0; 0x7F];
        self.interrupt_enable = 0;

        self.rom_bank = 1;
        self.ram_bank = 0;
        self.ram_enabled = false;
        self.in_bios = true;
        self.mbc_type = 0;
    }

    pub fn load_rom(&mut self, data: Vec<u8>) {
        self.rom = data;
        self.mbc_type = match self.rom.get(0x147).copied().unwrap_or(0) {
            0x01..=0x03 => 1,
            0x0F..=0x13 => 3,
            _ => 0,
        };
    }

    pub fn load_bios(&mut self, bios: Vec<u8>) {
        self.bios = Some(bios);
        self.in_bios = true;
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x00FF if self.in_bios => {
                self.bios.as_ref().map(|b| b[addr as usize]).unwrap_or(0xFF)
            }
            0x0000..=0x3FFF => self.rom.get(addr as usize).copied().unwrap_or(0xFF),
            0x4000..=0x7FFF => {
                let bank_offset = (self.rom_bank as usize) * 0x4000;
                self.rom.get(bank_offset + (addr as usize - 0x4000)).copied().unwrap_or(0xFF)
            }
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    self.eram[(addr - 0xA000) as usize]
                } else {
                    0xFF
                }
            }
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize],
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            0xFEA0..=0xFEFF => 0xFF,
            0xFF00 => self.joypad.read(),
            0xFF01..=0xFF7F => self.io[(addr - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.interrupt_enable,
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.enable_ram(value),
            0x2000..=0x3FFF => self.switch_rom_bank(value),
            0x4000..=0x5FFF => self.switch_ram_bank(value),
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = value,
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    self.eram[(addr - 0xA000) as usize] = value;
                }
            }
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize] = value,
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize] = value,
            0xFEA0..=0xFEFF => {},
            0xFF00 => self.joypad.write(value),
            0xFF01..=0xFF7F => self.io[(addr - 0xFF00) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = value,
            0xFF50 => self.in_bios = false,
            0xFFFF => self.interrupt_enable = value,
            _ => {}
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let lo = self.read_byte(addr) as u16;
        let hi = self.read_byte(addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    pub fn write_word(&mut self, addr: u16, value: u16) {
        self.write_byte(addr, value as u8);
        self.write_byte(addr.wrapping_add(1), (value >> 8) as u8);
    }

    pub fn enable_ram(&mut self, value: u8) {
        self.ram_enabled = (value & 0x0F) == 0x0A;
    }

    pub fn switch_rom_bank(&mut self, value: u8) {
        let bank = value & 0x1F;
        self.rom_bank = if bank == 0 { 1 } else { bank };
    }

    pub fn switch_ram_bank(&mut self, value: u8) {
        self.ram_bank = value & 0x03;
    }

    pub fn dma_transfer(&mut self, start: u8) {
        let base = (start as u16) << 8;
        for i in 0..0xA0 {
            let byte = self.read_byte(base + i);
            self.oam[i] = byte;
        }
    }

    pub fn dump_memory(&self, start: u16, end: u16) {
        for addr in (start..=end).step_by(16) {
            print!("{:04X}: ", addr);
            for offset in 0..16 {
                let b = self.read_byte(addr + offset);
                print!("{:02X} ", b);
            }
            println!();
        }
    }
}
