use std::time::{Instant, Duration};
use std::thread::sleep;
use chip8::Instruction;
use chip8::*;

fn main() {


    let t1 = Instant::now();

    println!("WELCOME TO THE CHIP-8 EMULATOR");
    println!("\t[+] RAM SIZE: {}", chip8::MEMORY_SIZE);
    let instr = Instruction::OR(7,0) ;
    
    match instr {
        Instruction::OR(Vx, Vy) => {
            println!("DEBUG: VX= {}, Vy:{}", Vx, Vy);
        }
        _ => ()
    };

    let mut cpu = CPU::new(None);

    match cpu.loadt("example.txt"){
        Ok(_) => {},
        Err(e) => {
            panic!("{}", e);
        }
    }

    cpu.power_on();
    loop {
        match cpu.next_cycle() {
            Ok(_) => {},
            Err(e) => {
                panic!("{}", e);
            }
        }
        match cpu.simulate() {
            Ok(_) => {},
            Err(e) => {
                panic!("{}", e);
            }
        }
    }


    sleep(Duration::from_millis(500));
    let t2 = t1.elapsed().as_millis();
    println!("Time elapsed: {})", t2);
}