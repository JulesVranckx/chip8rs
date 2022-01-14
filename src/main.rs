use std::time::{Instant, Duration};
use std::thread::sleep;
use chip8::*;
use clap::{Arg, App};
use std::process;

fn main() {

    let matches = App::new("chip-8 emulator")
        .version("0.0.0")
        .author("Jules Vranckx")
        .about("chip-8 emulator")
        .arg(Arg::with_name("file")
                 .short('f')
                 .long("file")
                 .takes_value(true)
                 .value_name("FILE")
                 .help("program to be executed"))
        .arg(Arg::with_name("text input")
                  .short('t')
                  .long("text")
                  .takes_value(false)
                  .help("input file as text")
        )
        .arg(Arg::with_name("raw input")
                  .short('r')
                  .long("raw")
                  .takes_value(false)
                  .help("input file as raw")
        )
        .get_matches();

    // println!("WELCOME TO THE CHIP-8 EMULATOR");
    // println!("\t[+] RAM SIZE: {}", chip8::MEMORY_SIZE);
    let instr = Instruction::OR(7,0) ;
    
    // match instr {
    //     Instruction::OR(Vx, Vy) => {
    //         println!("DEBUG: VX= {}, Vy:{}", Vx, Vy);
    //     }
    //     _ => ()
    // };

    let filename = matches.value_of("file").unwrap();
    let bin = matches.is_present("raw input");
    let text = matches.is_present("text input");
    
    if bin && text {
        println!("Can't handle both bianry and text file. Use -r OR -t");
        process::exit(-1);
    }
    if !bin && !text {
        println!("Please select an input file type: either -r or -t");
        process::exit(-1);
    }

    let mut cpu = CPU::new(Some(10));

    if bin {
        match cpu.loadb(filename){
            Ok(_) => {},
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
    else {
        match cpu.loadt(filename){
            Ok(_) => {},
            Err(e) => {
                panic!("{}", e);
            }
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

        if let Ok(refresh) = cpu.consume_refresh() {
            if refresh {
                let buff =  cpu.get_image().unwrap();
                println!("Refresh screen");
            }
        }

        if let Ok(sound) = cpu.sound() {
            if sound {
                println!("BMP");
            }
        }


    }
}