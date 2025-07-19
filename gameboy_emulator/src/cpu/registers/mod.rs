pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

// ====== ANUM =======

    #[derive(Debug, Copy, Clone)]
    pub enum Register8 {
        A, F,
        B, C,
        D, E,
        H, L,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum Register16 {
        AF,
        BC,
        DE,
        HL,
    }

impl Registers {
    // === AF ===
    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | ((self.f & 0xF0) as u16)
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0xF0) as u8; // Bits 0-3 são sempre 0
    }

    // === BC ===
    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    // === DE ===
    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    // === HL ===
    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    // === Registradores Individuais ===

    pub fn get_a(&self) -> u8 {
        self.a
    }
    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn get_f(&self) -> u8 {
        self.f & 0xF0 // Apenas os 4 bits superiores são válidos, no casso 0000-xxxx
    }
    pub fn set_f(&mut self, value: u8) {
        self.f = value & 0xF0;
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }
    pub fn set_b(&mut self, value: u8) {
        self.b = value;
    }

    pub fn get_c(&self) -> u8 {
        self.c
    }
    pub fn set_c(&mut self, value: u8) {
        self.c = value;
    }

    pub fn get_d(&self) -> u8 {
        self.d
    }
    pub fn set_d(&mut self, value: u8) {
        self.d = value;
    }

    pub fn get_e(&self) -> u8 {
        self.e
    }
    pub fn set_e(&mut self, value: u8) {
        self.e = value;
    }

    pub fn get_h(&self) -> u8 {
        self.h
    }
    pub fn set_h(&mut self, value: u8) {
        self.h = value;
    }

    pub fn get_l(&self) -> u8 {
        self.l
    }
    pub fn set_l(&mut self, value: u8) {
        self.l = value;
    }

        // ===================== FLAGS =====================//


        // === GETTERS ===
    pub fn get_flag_z(&self) -> bool {
        self.f & 0b1000_0000 != 0
    }
    pub fn get_flag_n(&self) -> bool {
        self.f & 0b0100_0000 != 0
    }
    pub fn get_flag_h(&self) -> bool {
        self.f & 0b0010_0000 != 0
    }
    pub fn get_flag_c(&self) -> bool {
        self.f & 0b0001_0000 != 0
    }

    // === SETTERS ===
    pub fn set_flag_z(&mut self, value: bool) {
        if value {
            self.f |= 0b1000_0000;
        } else {
            self.f &= !0b1000_0000;
        }
    }
    pub fn set_flag_n(&mut self, value: bool) {
        if value {
            self.f |= 0b0100_0000;
        } else {
            self.f &= !0b0100_0000;
        }
    }
    pub fn set_flag_h(&mut self, value: bool) {
        if value {
            self.f |= 0b0010_0000;
        } else {
            self.f &= !0b0010_0000;
        }
    }
    pub fn set_flag_c(&mut self, value: bool) {
        if value {
            self.f |= 0b0001_0000;
        } else {
            self.f &= !0b0001_0000;
        }
    }

    // ======= GET E SAT 8b ====== //

    pub fn get_8(&self, reg: Register8) -> u8 {
        match reg {
            Register8::A => self.get_a(),
            Register8::F => self.get_f(),
            Register8::B => self.get_b(),
            Register8::C => self.get_c(),
            Register8::D => self.get_d(),
            Register8::E => self.get_e(),
            Register8::H => self.get_h(),
            Register8::L => self.get_l(),
        }
    }

    pub fn set_8(&mut self, reg: Register8, value: u8) {
        match reg {
            Register8::A => self.set_a(value),
            Register8::F => self.set_f(value),
            Register8::B => self.set_b(value),
            Register8::C => self.set_c(value),
            Register8::D => self.set_d(value),
            Register8::E => self.set_e(value),
            Register8::H => self.set_h(value),
            Register8::L => self.set_l(value),
        }
    }

    // ======= GET E SAT 8b ====== //
}