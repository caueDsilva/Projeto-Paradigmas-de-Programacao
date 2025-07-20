mod cpu;
mod gpu;
mod input;
mod mmu;
mod memory;

use gpu::{GPU, SCREEN_W, SCREEN_H};
use cpu::CPU;
use mmu::MMU;

fn main() {
    let mut gpu = GPU::new();
    let mut cpu = CPU::new();
    let mut mmu = MMU::new();

    for _ in 0..(154 * 456 / 4) {
        let interrupts = gpu.step(4);
        mmu.set_interrupts(interrupts);
        cpu.step(&mut mmu);
    }

    save_framebuffer_as_png(&gpu.framebuffer.data, SCREEN_W, SCREEN_H, "frame.png");
    println!("Renderização concluída. Imagem salva como 'frame.png'.");
}

fn save_framebuffer_as_png(buffer: &[u8], width: usize, height: usize, filename: &str) {
    use image::{Rgb, RgbImage};

    let mut img = RgbImage::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            let i = (y * width + x) * 3;
            let pixel = Rgb([buffer[i], buffer[i + 1], buffer[i + 2]]);
            img.put_pixel(x as u32, y as u32, pixel);
        }
    }

    img.save(filename).expect("Erro ao salvar imagem");
}