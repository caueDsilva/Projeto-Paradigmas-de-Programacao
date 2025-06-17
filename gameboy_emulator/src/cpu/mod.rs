// Arquitetura do gameboy é similar a arquitetura Z80

use crate::memory::Memory;

pub struct CPU {    // pub = publico
    pub a: u8,      // u8 = 8 bits inteiros ou seja de 0 a 255
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub memory: Memory // Utilza a classe Memory
} 

impl CPU {
    // fn = function
    pub fn new() -> Self {
        CPU {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
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

}