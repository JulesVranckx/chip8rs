

use chip8::Instruction;

fn main() {

    println!("WELCOME TO THE CHIP-8 EMULATOR");
    println!("\t[+] RAM SIZE: {}", chip8::MEMORY_SIZE);
    let instr = Instruction::OR(7,0) ;
    
    match instr {
        Instruction::OR(Vx, Vy) => {
            println!("DEBUG: VX= {}, Vy:{}", Vx, Vy);
        }
        _ => ()
    };
}