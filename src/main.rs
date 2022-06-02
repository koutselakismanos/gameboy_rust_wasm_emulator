use std::path::Path;

use eframe::egui;

use gameboy_rust_webassembly_emulator::hardware::cartridge::Cartridge;
use gameboy_rust_webassembly_emulator::hardware::cpu::{CPU, instructions};
use gameboy_rust_webassembly_emulator::hardware::cpu::instructions::{Instruction, Target};
use gameboy_rust_webassembly_emulator::hardware::cpu::memory::Memory;
use gameboy_rust_webassembly_emulator::hardware::cpu::registers::flags::Flag;
use gameboy_rust_webassembly_emulator::hardware::cpu::registers::Registers;

fn main() {
    let cartridge = Cartridge::load(Path::new("roms/tetris.gb"));
    println!("TITLE: {}", cartridge.get_title());
    println!("ROM_SIZE: {:?}", cartridge.get_header());

    let mut cpu = CPU::new(cartridge);
    println!("yo {:X} {:X}", cpu.bus_read(0x216), cpu.bus_read(0x217));

    while cpu.is_running {
        cpu.step();
        if cpu.registers.pc == 0x21B {
            cpu.stop();
        }
    }

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My emulator",
        options,
        Box::new(|_cc| Box::new(MyApp::new(cpu))),
    );
}

struct MyApp {
    cpu: CPU,
}

impl MyApp {
    pub fn new(cpu: CPU) -> Self {
        Self {
            cpu
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("My egui Application");
            // ui.horizontal(|ui| {
            //     ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Click each year").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.label("Registers:");
            ui.label(format!("A: 0x{:X}", self.cpu.registers.a));
            ui.horizontal(|ui| {
                ui.label("F:");
                ui.checkbox(&mut self.cpu.registers.get_flag(Flag::Zero), "Zero");
                ui.checkbox(&mut self.cpu.registers.get_flag(Flag::Negative), "Negative");
                ui.checkbox(&mut self.cpu.registers.get_flag(Flag::HalfCarry), "HalfCarry");
                ui.checkbox(&mut self.cpu.registers.get_flag(Flag::Carry), "Carry");
            });
            ui.label(format!("B: 0x{:X}", self.cpu.registers.b));
            ui.label(format!("C: 0x{:X}", self.cpu.registers.c));
            ui.label(format!("H: 0x{:X}", self.cpu.registers.h));
            ui.label(format!("L: 0x{:X}", self.cpu.registers.l));
            ui.label(format!("AF: 0x{:X}", self.cpu.registers.get_af()));
            ui.label(format!("BC: 0x{:X}", self.cpu.registers.get_bc()));
            ui.label(format!("HL: 0x{:X}", self.cpu.registers.get_hl()));
            ui.label(format!("SP: 0x{:X}", self.cpu.registers.sp));
            ui.label(format!("PC: 0x{:X}", self.cpu.registers.pc));

            if ui.button("Step").clicked() {
                self.cpu.step();
            }
        });
    }
}