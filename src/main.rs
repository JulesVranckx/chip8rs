use clap::{Arg, App};
use std::process;

mod cpu;
use cpu::{CPU, Instruction, FRAME_BUFFER_LENGTH, FRAME_BUFFER_HEIGHT};

mod drivers;
use drivers::{DisplayDriver, AudioDriver};

fn main() {

    let matches = App::new("chip-8 emulator")
        .version("0.0.0")
        .author("Jules Vranckx")
        .about("chip-8 emulator")
        .arg(Arg::new("file")
                 .short('f')
                 .long("file")
                 .takes_value(true)
                 .value_name("FILE")
                 .help("program to be executed"))
        .arg(Arg::new("text input")
                  .short('t')
                  .long("text")
                  .takes_value(false)
                  .help("input file as text")
        )
        .arg(Arg::new("raw input")
                  .short('r')
                  .long("raw")
                  .takes_value(false)
                  .help("input file as raw")
        )
        .get_matches();
    


    //Load arguments
    let filename = matches.value_of("file").unwrap();
    let bin = matches.is_present("raw input");
    let text = matches.is_present("text input");

    // Set up drivers
    let sdl_context = sdl2::init().unwrap();
    let audio_driver = AudioDriver::new(&sdl_context);
    let mut display_driver = DisplayDriver::new(&sdl_context);
    
    if bin && text {
        println!("Can't handle both bianry and text file. Use -r OR -t");
        process::exit(-1);
    }
    if !bin && !text {
        println!("Please select an input file type: either -r or -t");
        process::exit(-1);
    }

    let mut cpu = CPU::new(Some(600));

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
                println!("{}", e);
                process::exit(-1);
            }
        }
        match cpu.simulate() {
            Ok(_) => {},
            Err(e) => {
                println!("{}", e);
                process::exit(-1);
            }
        }

        if let Ok(refresh) = cpu.consume_refresh() {
            if refresh {
                let buff =  cpu.get_image().unwrap();
                display_driver.draw(buff);
            }
        }

        if let Ok(sound) = cpu.sound() {
            if sound {
                audio_driver.start_beep();
            }
            else {
                audio_driver.stop_beep();
            }
        }


    }
}