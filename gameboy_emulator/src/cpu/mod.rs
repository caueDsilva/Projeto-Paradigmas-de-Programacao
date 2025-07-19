// Arquitetura do gameboy é similar a arquitetura Z80

mod registers;

use crate::{cpu, memory::Memory};
use crate::registers::Registers;

pub struct CPU {    // pub = publico
    pub regs: Registers,
    pub sp: u16,
    pub pc: u16,
    pub memory: Memory // Utilza a classe Memory
} 

impl CPU {
    // fn = function
    pub fn new() -> Self {
        CPU {
            regs: Registers,
            sp: 0xFFFE,
            pc: 0x0100, // Ponto de entrada do Game Boy
            memory: Memory::new(),
        }
    }

    // Lê o proximo byte de instrução da memoria do registrador PC e o retorna (u8);
    // Exemplo: Pc = 0x0100, ele le memory[0x0100] e passa para a proxima instrução seguinte da memory (wrapping_add(1)) para ser interpretado;
    pub fn fetch_byte(&mut self) -> u8 {
        let byte: u8 = self.memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    pub fn ADD_A (&mut self,value:u8) {
        let a: u8 = self.a;
        let result: u8 = a.wrapping_add(value);

        // Half-carry ocorre se a soma dos 4 bits inferiores ultrapassa 0xF
        let half_carry: bool = (a & 0x0F) + (value & 0x0F) > 0x0F;

        // Carry ocorre se a soma ultrapassa 0xFF (255)
        let carry: bool = (a as u16) + (value as u16) > 0xFF;

        self.a = result;

        // Atualiza os flags no registrador F
        self.f = 0; // zera os flags antes de setar

        if self.a == 0 {
            self.f |= 0b1000_0000; // Z flag (bit 7)
        }

        // N flag (bit 6) é 0 para ADD (já está zerado)

        if half_carry {
            self.f |= 0b0010_0000; // H flag (bit 5)
        }

        if carry {
            self.f |= 0b0001_0000; // C flag (bit 4)
        }
    }

    pub fn ADC_A_Carry(&mut self, value:u8){

        let a: u8 = self.a;
        // Obtém o bit de carry atual do registrador F (bit 4)
        let carry_in: u8 = if (self.f & 0b0001_0000) != 0 {1} else {0};

        // Soma A + value + carry
        let result: u8 = a.wrapping_add(value).wrapping_add(carry_in);

        //half carry: se a soma dos 4 bits anteriores forem maiores que 0x0F
        let half_carry: bool = (a & 0x0f) + (value & 0x0F) + carry_in > 0x0F;

        // Carry: se a soma ultrapassou 255 (8 bits)
        let full_carry: bool = (a as u16) + (value as u16) + (carry_in as u16) > 0xFF;

        // Atualiza o acumulador 
        self.a = result;
        // Atualiza as flags
        self.f = 0;

        if (self.a == 0){
            self.f |= 0b1000_0000; // Z flag
        }

        // N flag é 0 (porque é uma soma)
        if half_carry {
            self.f |= 0b0010_0000; // H flag
        }

        if full_carry {
            self.f |= 0b0001_0000; // C flag
        }

    }

    pub fn SUB_A(&mut self, value: u8) {
        let a: u8 = self.a;

        // Cálculo do resultado (só o valor final da subtração com wrap)
        let result: u8 = a.wrapping_sub(value);

        // Half borrow: se bit baixo de A é menor que o de value
        let half_borrow: bool = (a & 0x0F) < (value & 0x0F);

        // Borrow total (carry): se A é menor que value
        let borrow: bool = a < value;

        // Atualiza acumulador
        self.a = result;

        // Atualiza flags
        self.f = 0;
        if self.a == 0 {
            self.f |= 0b1000_0000; // Z
        }

        self.f |= 0b0100_0000; // N sempre 1 para SUB

        if half_borrow {
            self.f |= 0b0010_0000; // H
        }

        if borrow {
            self.f |= 0b0001_0000; // C
        }
    }

    pub fn SBC_A(&mut self, value: u8) {
        let a = self.a;

        // Verifica se o carry flag (bit 4) está ligado
        let carry_in = if (self.f & 0b0001_0000) != 0 { 1 } else { 0 };

        // Calcula o resultado com wrap (estilo hardware)
        let result = a.wrapping_sub(value).wrapping_sub(carry_in);

        // Half-borrow: ocorre se (A & 0xF) < ((value & 0xF) + carry)
        let half_borrow = (a & 0x0F) < ((value & 0x0F) + carry_in);

        // Borrow total: ocorre se A < (value + carry)
        let borrow = (a as u16) < (value as u16 + carry_in as u16);

        // Atualiza o acumulador
        self.a = result;

        // Atualiza flags
        self.f = 0;

        if self.a == 0 {
            self.f |= 0b1000_0000; // Z (bit 7)
        }

        self.f |= 0b0100_0000;     // N (bit 6) sempre 1

        if half_borrow {
            self.f |= 0b0010_0000; // H (bit 5)
        }

        if borrow {
            self.f |= 0b0001_0000; // C (bit 4)
        }
    }
}