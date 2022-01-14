use std::time::Duration;
use std::thread::sleep;
use rand::Rng;
use std::fs::File;
use std::fs;
use std::io::Read;
use std::i64;

pub const MEMORY_SIZE: usize = 0x1000 ;
pub const GP_REGISTERS_COUNT: usize = 16 ;
pub const STACK_SIZE: usize = 16 ;
pub const PROGRAM_START: usize = 0x200 ;
pub const FRAME_BUFFER_LENGTH: usize = 8;
pub const FRAME_BUFFER_HEIGHT: usize = 32;
pub const DEFAULT_FREQUENCY: u32 = 600;

pub type Addr = usize ;
pub type StackAdress = usize ;
pub type VIndex = usize ;
pub type CellValue = u8 ;
pub type StackValue = Addr ;
pub type VValue = u8 ;

/// Registers strucure
/// Holds a 16 bytes long array
/// Defines methods to create a new instance, read from registers and write in it
struct Registers{
    regs: [VValue; GP_REGISTERS_COUNT]
}

impl Registers {
    fn new() -> Registers {
        Registers{
            regs: [0; GP_REGISTERS_COUNT]
        }
    }

    fn read(&self, index: VIndex) -> Result<VValue, &'static str>{
        
        if index >= GP_REGISTERS_COUNT - 1 {
            return Err("Register index too high. Please use -c to see CHIP-8 caracteristics")
        }
        
        Ok(self.regs[index])
    }

    fn write(&mut self, index: VIndex, value: VValue) -> Result<(), &'static str> {
        
        if index >= GP_REGISTERS_COUNT - 1 {
            return Err("Register index too high. Please use -c to see CHIP-8 caracteristics")
        }

        self.regs[index] = value ;

        Ok(())
    }

    fn set_f(&mut self) -> Result<(), &'static str> {
        self.regs[15] = 1 ;
        Ok(())
    }

    fn get_f(&self) -> Result<&VValue, &'static str> {
        Ok(&self.regs[15])
    }
}

/// Memory structure
struct Memory{
    cells: [CellValue; MEMORY_SIZE]
}

impl Memory {
    fn new() -> Memory {
        Memory{
            cells: [0; MEMORY_SIZE]
        }
    }

    fn read(&self, index: Addr) -> Result<CellValue, &'static str>{
        
        if index >= MEMORY_SIZE {
            return Err("Memory index too high. Please use -c to see CHIP-8 caracteristics")
        }
        
        Ok(self.cells[index])
    }

    fn write(&mut self, index: Addr, value: CellValue) -> Result<(), &'static str> {
        
        if index >= MEMORY_SIZE {
            return Err("Memory index too high. Please use -c to see CHIP-8 caracteristics")
        }

        self.cells[index] = value ;

        Ok(())
    }
}

/// Stack
struct Stack {
    cells: [StackValue; STACK_SIZE],
    sp: StackAdress
}

impl Stack {

    fn new() -> Stack {
        Stack{
            cells: [0; STACK_SIZE],
            sp: 0
        }
    }

    fn push(&mut self, value: StackValue) -> Result<(), &'static str> {
        
        if self.sp == STACK_SIZE {
            return Err("Stack overflow")
        }
        
        self.cells[self.sp] = value;
        self.sp += 1;

        Ok(())
    }

    fn pop(&mut self) -> Result<StackValue, &'static str> {

        if self.sp == 0 {
            return Err("Stack is empty, can't pop")
        }

        self.sp -= 1 ;
        Ok(self.cells[self.sp-1])
    }
}

/// Delay Timer
struct DelayTimer {
    value: u8
}

impl DelayTimer {

    fn new() -> DelayTimer {
        DelayTimer{value:0}
    }
    
    fn decrease(&mut self) -> Result<(), &'static str> {
        if self.value != 0 {
            self.value -= 1;
        }
        Ok(())
    }

    fn get(&self) -> Result<u8, &'static str> {
        Ok(self.value)
    }

    fn set(&mut self, value: u8) -> Result<(), &'static str> {
        self.value = value;
        Ok(())
    }
}

/// Sound Timer
struct SoundTimer{
    value: u8
}

impl SoundTimer {

    fn new() -> SoundTimer {
        SoundTimer{value:0}
    }

    fn decrease(&mut self) -> Result<(), &'static str> {
        if self.value != 0 {
            self.value -= 1;
        }
        Ok(())
    }

    fn sound(&self) -> Result<(), &'static str> {
        return Err("Sound not implemented yet");
    }

    fn set(&mut self, value: u8) -> Result<(), &'static str> {
        self.value = value;
        Ok(())
    }
}

/// FrameBuffer
struct FrameBuffer{
    buffer: [[u8; FRAME_BUFFER_LENGTH]; FRAME_BUFFER_HEIGHT]
}

impl FrameBuffer {
    
    fn new() -> FrameBuffer {
        FrameBuffer {
            buffer: [[0; FRAME_BUFFER_LENGTH]; FRAME_BUFFER_HEIGHT]
        }
    }
}

/// PC
struct ProgramCounter {
    register: Addr
}

impl ProgramCounter {

    fn new() -> ProgramCounter {
        ProgramCounter {
            register: PROGRAM_START
        }
    }

    fn incr(&mut self) -> Result<(), &'static str> {
        if self.register == MEMORY_SIZE - 1 {
            return Err("Program Counter Overflow")
        }

        self.register += 2 ;

        Ok(())
    }

    fn change(&mut self, addr: Addr) -> Result<(), &'static str> {

        if addr >= MEMORY_SIZE {
            return Err("Trying to set program counter to code out of the memory")
        }

        self.register = addr ;

        Ok(())
    } 

    fn get(&self) -> Addr {
        self.register
    }
}

/// I
struct IndexRegister{
    value: Addr
}

impl IndexRegister {

    fn new() -> IndexRegister {
        IndexRegister{value:0}
    }

    fn set(&mut self, value: Addr) -> Result<(), &'static str> {
        self.value = value;
        Ok(())
    }

    fn get(&self) -> Result<Addr, &'static str> {
        Ok(self.value)
    }
}

enum CpuState {
    IDLE,
    FETCH,
    DECODE,
    EXEC
}

/// CPU
pub struct CPU {
    on: bool,
    ram: Memory,
    v: Registers,
    stack: Stack,
    pc: ProgramCounter,
    index_register: IndexRegister,
    dt: DelayTimer,
    st: SoundTimer,
    frame_buff: FrameBuffer,
    keyboard: u16,
    frequency: u32,
    frequency_counter: u32,
    opcode: u16,
    instr: Option<Instruction>,
    state: CpuState
}

impl CPU {

    pub fn new(frequency: Option<u32>) -> CPU {

        let frequency = match frequency {
            Some(frequency) => frequency,
            None => DEFAULT_FREQUENCY
        };

        CPU {
            on: false,
            ram: Memory::new(),
            v: Registers::new(),
            stack: Stack::new(),
            pc: ProgramCounter::new(),
            index_register: IndexRegister::new(),
            dt: DelayTimer::new(),
            st: SoundTimer::new(),
            frame_buff: FrameBuffer::new(),
            keyboard: 0,
            frequency: frequency,
            frequency_counter: 0u32,
            opcode: 0,
            instr: None,
            state: CpuState::IDLE
        }
    }

    pub fn power_on(&mut self) {
        self.on = true;
    }

    pub fn loadt(&mut self, filename: &str) -> Result<(), &'static str>{
        let content = fs::read_to_string(filename).unwrap();
        let mut i = PROGRAM_START ;
        for line in content.lines() {
            let opcode = i64::from_str_radix(line, 16).unwrap() as u16;
            let l = ((opcode & 0xFF00) >> 8) as CellValue;
            let r = (opcode & 0x00FF) as CellValue;
            self.ram.write(i, l)?;
            self.ram.write(i+1, r)?;
            i += 2;
        }
        Ok(())
    }

    pub fn loadb(&mut self, filename: &str) -> Result<(), &'static str>{
        let mut f = File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
        let mut addr = PROGRAM_START;
        for chunk in buffer {
            self.ram.write(addr, chunk)?;
            addr += 1; 
        }
        
        Ok(())
    }

    pub fn next_cycle(&mut self) -> Result<(), &'static str> {

        if !self.on {
            self.state = CpuState::IDLE ;
        }
        else {
            match self.state {
            
                CpuState::IDLE => {
                    self.state = CpuState::FETCH ;
                },

                CpuState::FETCH => {
                    self.state = CpuState::DECODE ;
                },
                
                CpuState::DECODE => {
                    self.state = CpuState::EXEC;
                },
                
                CpuState::EXEC => {
                    self.state = CpuState::FETCH;
                }
            }
        }

        Ok(())
    }

    pub fn simulate(&mut self) -> Result<(), &'static str> {

        //Update the frequency counter for delay and sound registers
        self.frequency_counter = (self.frequency_counter + 1) % (self.frequency / 60) ;
        
        //Perform state action
        match self.state {
            CpuState::IDLE => {},
            
            CpuState::FETCH => {
                println!("[+] fetching @{}", self.pc.get());
                self.fetch()?;
            }

            CpuState::DECODE => {
                println!("[+] decoding #{}", self.opcode);
                self.decode()?;
            }

            CpuState::EXEC => {
                if let Some(instr) = &self.instr {
                println!("[+] ccrt_instr: {:?}", instr);
                }
                self.execute()?;
            }
        };

        let sleep_time: u64 = (1e6 as f64 / (self.frequency as f64)) as u64 ;

        if self.frequency_counter == 0 {
            
        }

        self.st.sound();

        sleep(Duration::from_micros(sleep_time));

        Ok(())

    }

    fn fetch(&mut self) -> Result<(), &'static str> {
        let pc_value = self.pc.get();
        let l = self.ram.read(pc_value)? as u16;
        let r = self.ram.read(pc_value + 1)? as u16;
        self.opcode = (l<<8)+ r;
        Ok(())  
    }

    fn decode(&mut self) -> Result<(), &'static str> {
        
        let bytes = (
            ((self.opcode & 0xF000) >> 12)as u8,
            ((self.opcode & 0x0F00) >> 8)as u8,
            ((self.opcode & 0x00F0) >> 4)as u8,
            (self.opcode & 0x000F) as u8
        );

        match bytes {

            (0, 0, 0xE, 0) => {
                self.instr = Some(Instruction::CLS) ;
            },

            (0,0,0xE, 0xE) => {
                self.instr = Some(Instruction::RET) ;
            },

            (1,_,_,_) => {
                let addr = (self.opcode & 0x0FFF) as Addr;
                self.instr = Some(Instruction::JP(addr));
            },

            (2,_,_,_) => {
                let addr = (self.opcode & 0x0FFF) as Addr;
                self.instr = Some(Instruction::CALL(addr));
            },

            (3,x,_,_) => {
                let k = (self.opcode & 0x00FF) as VValue;  
                self.instr = Some(Instruction::SEi(x as VIndex, k));
            },

            (4,x,_,_) => {
                let kk = (self.opcode & 0x00FF) as VValue;  
                self.instr = Some(Instruction::SNEi(x as VIndex, kk));  
            },

            (5,x,y,0) => {
                self.instr = Some(Instruction::SE(x as VIndex,y as VIndex));
            },

            (6,x,_,_) => {
                let kk = (self.opcode & 0x00FF) as VValue; 
                self.instr = Some(Instruction::LDi(x as VIndex, kk));
            },

            (7,x,_,_) => {
                let kk = (self.opcode & 0x00FF) as VValue; 
                self.instr = Some(Instruction::ADDi(x as VIndex, kk));
            },

            (8,x,y,0) => {
                self.instr = Some(Instruction::LD(x as VIndex,y as VIndex));
            },

            (8,x,y,1) => {
                self.instr = Some(Instruction::OR(x as VIndex,y as VIndex));
            },

            (8,x,y,2) => {
                self.instr = Some(Instruction::AND(x as VIndex,y as VIndex));
            },

            (8,x,y,3) => {
                self.instr = Some(Instruction::XOR(x as VIndex,y as VIndex));
            },

            (8,x,y,4) => {
                self.instr = Some(Instruction::ADD(x as VIndex,y as VIndex));
            },

            (8,x,y,5) => {
                self.instr = Some(Instruction::SUB(x as VIndex,y as VIndex));
            },

            (8,x,_,6) => {
                self.instr = Some(Instruction::SHR(x as VIndex));
            },

            (8,x,y,7) => {
                self.instr = Some(Instruction::SUBN(x as VIndex,y as VIndex));
            },

            (8,x,_,0xE) => {
                self.instr = Some(Instruction::SHL(x as VIndex));
            },

            (9,x,y,0) => {
                self.instr = Some(Instruction::SNE(x as VIndex,y as VIndex));
            },

            (0xA,_,_,_) => {
                let addr = (self.opcode & 0x0FFF) as Addr;
                self.instr = Some(Instruction::LD_I(addr));
            },

            (0xB,_,_,_) => {
                let addr = (self.opcode & 0x0FFF) as Addr;
                self.instr = Some(Instruction::JP_V0(addr));
            },

            (0xC,x,_,_) => {
                let kk = (self.opcode & 0x00FF) as VValue; 
                self.instr = Some(Instruction::RNDi(x as VIndex, kk));
            },

            (0xD,x,y,n) => {
                self.instr = Some(Instruction::DRW(x as VIndex, y as VIndex, n));
            },

            (0xE, x, 9, 0xE) => {
                self.instr = Some(Instruction::SKP(x as VIndex));
            },

            (0xE, x, 0xA, 1) => {
                self.instr = Some(Instruction::SKNP(x as VIndex));
            },

            (0xF, x, 0, 7) => {
                self.instr = Some(Instruction::LD_DT(x as VIndex));
            },

            (0xF, x, 0, 0xA) => {
                self.instr = Some(Instruction::LD_K(x as VIndex));
            },

            (0xF, x, 1, 5) => {
                self.instr = Some(Instruction::SET_DT(x as VIndex));
            },

            (0xF, x, 1, 8) => {
                self.instr = Some(Instruction::SET_ST(x as VIndex));
            },

            (0xF, x, 1, 0xE) => {
                self.instr = Some(Instruction::ADD_I(x as VIndex));
            },

            (0xF, x, 2, 9) => {
                self.instr = Some(Instruction::LD_F(x as VIndex));
            },

            (0xF, x, 3, 3) => {
                self.instr = Some(Instruction::LD_B(x as VIndex));
            },

            (0xF, x, 5, 5) => {
                self.instr = Some(Instruction::ST_UNTIL(x as VIndex));
            },

            (0xF, x, 6, 5) => {
                self.instr = Some(Instruction::LD_UNTIL(x as VIndex));
            },

            _ => {
                return Err("Can't decode current instruction");
            }
        }
        

        Ok(())
    }

    fn execute(&mut self) -> Result<(), &'static str> {
        
        let mut increase_pc = true ;

        if let Some(instr) = &self.instr {
            
            match instr {
            
                Instruction::SYS(_) => {
                    return Err("SYS instruction no more supported");
                }
                Instruction::CLS => {
                    return Err("NOT IMPLEMENTED YET");
                }
                Instruction::RET => {
                    let tmp = self.stack.pop()?;
                    self.pc.change(tmp);

                }
                Instruction::JP(addr) => {
                    self.pc.change(*addr)?;
                    increase_pc = false;
                },
                Instruction::CALL(addr) => {
                    self.stack.push(self.pc.get())?;
                    self.pc.change(*addr)?;
                    increase_pc = false;            
                },
                Instruction::SEi(vx, kk) => {
                    if self.v.read(*vx)? == *kk {
                        self.pc.incr()?; 
                    }
                },

                Instruction::SNEi(vx, kk) => {
                    if self.v.read(*vx)? != *kk {
                        self.pc.incr()?; 
                    }
                },

                Instruction::SE(vx, vy) => {
                    if self.v.read(*vx)? == self.v.read(*vy)? {
                        self.pc.incr()?; 
                    }
                }

                Instruction::LDi(vx,kk) => {
                    self.v.write(*vx, *kk)?;
                }
                Instruction::ADDi(vx,kk) => {
                    let v = self.v.read(*vx)?;
                    self.v.write(*vx, v+*kk)?;
                }
                Instruction::SE(vx,vy) => {
                    if self.v.read(*vx)? == self.v.read(*vy)? {
                        self.pc.incr()?; 
                    }
                }
                Instruction::SNE(vx,vy) => {
                    if self.v.read(*vx)? != self.v.read(*vy)? {
                        self.pc.incr()?; 
                    }
                }
                Instruction::OR(vx,vy) => {
                    let x = self.v.read(*vx)?;
                    let y = self.v.read(*vy)?;
                    let x = x | y ;
                    self.v.write(*vx, x)?;
                }
                Instruction::AND(vx,vy) => {
                    let x = self.v.read(*vx)?;
                    let y = self.v.read(*vy)?;
                    let x = x & y ;
                    self.v.write(*vx, x)?;
                }
                Instruction::XOR(vx,vy) => {
                    let x = self.v.read(*vx)?;
                    let y = self.v.read(*vy)?;
                    let x = x ^ y ;
                    self.v.write(*vx, x)?;
                }
                Instruction::ADD(vx,vy) => {
                    let x = self.v.read(*vx)?;
                    let y = self.v.read(*vy)?;
                    let result = x as u16 + y as u16;
                    self.v.write(*vx, (result & 0xFF) as CellValue)?;
                    if result > 0xFF {
                        self.v.set_f()?;
                    }
                }
                Instruction::SUB(vx,vy) => {
                    let mut x = self.v.read(*vx)?;
                    let mut y = self.v.read(*vy)?;
                    if x <= y{
                        let tmp = x;
                        x = y;
                        y = tmp;
                    }
                    else {
                        self.v.set_f()?;
                    }
                    let result = x as u16 - y as u16;
                    self.v.write(*vx, (result & 0xFF) as CellValue)?;
                }
                Instruction::SHR(vx) => {
                    let mut x = self.v.read(*vx)?;
                    if (x & 0x1) == 0x1 {
                        self.v.set_f()?;
                    }
                    x = x >> 2 ;
                    self.v.write(*vx,x)?;
                }
                Instruction::SUBN(vx,vy) => {
                    let mut x = self.v.read(*vx)?;
                    let mut y = self.v.read(*vy)?;
                    if y <= x{
                        let tmp = x;
                        x = y;
                        y = tmp;
                    }
                    else {
                        self.v.set_f()?;
                    }
                    let result = y as u16 - x as u16;
                    self.v.write(*vx, (result & 0xFF) as CellValue)?;
                }
                Instruction::SHL(vx) => {
                    let mut x = self.v.read(*vx)?;
                    if ((x & 0b1000_0000) >> 7) == 0x1 {
                        self.v.set_f()?;
                    }
                    x = x << 2 ;
                    self.v.write(*vx,x)?;
                }
                Instruction::LD(vx,vy) => {
                    let y = self.v.read(*vy)?;
                    self.v.write(*vx, y)?;
                }
                Instruction::LD_I(addr) => {
                    self.index_register.set(*addr)?;
                }
                Instruction::JP_V0(addr) => {
                    let v0 = self.v.read(0)?;
                    self.pc.change(*addr + v0 as Addr)?;
                    increase_pc = false;
                }
                Instruction::RNDi(vx, kk) => {
                    let mut rng = rand::thread_rng().gen_range(0..=255);
                    rng = rng & *kk ;
                    self.v.write(*vx, rng)?;
                }
                Instruction::DRW(vx,vy, n) => {
                    return Err("Not Implemented yet");
                }
                Instruction::SKP(vx) => {
                    let x = self.v.read(*vx)?;
                    let skip = self.keyboard & (1<<x) ;
                    if skip != 0 {
                        self.pc.incr()?;
                    }
                }
                Instruction::SKNP(vx) => {
                    let x = self.v.read(*vx)?;
                    let skip = self.keyboard & (1<<x) ;
                    if skip == 0 {
                        self.pc.incr()?;
                    }
                }
                Instruction::LD_DT(vx) => {
                    let dt_value = self.dt.get()?;
                    self.v.write(*vx, dt_value)?;
                }
                Instruction::LD_K(vx) => {
                    //TODO: Awful code, not working, consider changing it asap
                    let key = (self.keyboard & 0x1) as CellValue;
                    self.v.write(*vx, key)?;
                }
                Instruction::SET_DT(vx) => {
                    let x = self.v.read(*vx)?;
                    self.dt.set(x)?;
                }
                Instruction::SET_ST(vx) => {
                    let x = self.v.read(*vx)?;
                    self.st.set(x)?;
                }
                Instruction::ADD_I(vx) => {
                    let mut i = self.index_register.get()?;
                    let x = self.v.read(*vx)?;
                    i += x as Addr;
                    self.index_register.set(i)?;
                }
                Instruction::LD_F(vx) => {
                    let i = 0x0;
                    self.index_register.set(i)?;
                }
                Instruction::LD_B(vx) => {
                    let mut i = self.index_register.get()?;
                    //x = bcd
                    let x = self.v.read(*vx)?;
                    let b = x / 100;
                    let c = (x % 100) / 10 ;
                    let d = x % 10;
                    self.ram.write(i, b)?;
                    i += 1;
                    self.ram.write(i, c)?;
                    i += 1;
                    self.ram.write(i, d)?;                    
                }
                Instruction::ST_UNTIL(vx) => {
                    let mut i_value = self.index_register.get()?;
                    for i in 0..=*vx{
                        let x = self.v.read(i as VIndex)?;
                        self.ram.write(i_value + i, x)?;
                        i_value += 1;
                    }
                }
                Instruction::LD_UNTIL(vx) => {
                    let mut i_value = self.index_register.get()?;
                    for i in 0..=*vx{
                        let value = self.ram.read(i_value + i)?;
                        self.v.write(i, value)?;
                    }
                }
                _ => {
                    return Err("Unsupported Instruction");
                }
            }
        }

        if increase_pc {
            self.pc.incr()?;
        }

        Ok(())

    }

}

/// Instruction Set
#[derive(Debug)]
pub enum Instruction {
    SYS(Addr),
    CLS,
    RET,
    JP(Addr),
    CALL(Addr),
    SEi(VIndex, VValue),
    SNEi(VIndex, VValue),
    LDi(VIndex, VValue),
    ADDi(VIndex, VValue),
    SE(VIndex, VIndex),
    LD(VIndex, VIndex),
    OR(VIndex, VIndex),
    AND(VIndex, VIndex),
    XOR(VIndex, VIndex),
    ADD(VIndex, VIndex),
    SUB(VIndex, VIndex),
    SHR(VIndex),
    SUBN(VIndex, VIndex),
    SHL(VIndex),
    SNE(VIndex, VIndex),
    LD_I(Addr),
    JP_V0(Addr),
    RNDi(VIndex, VValue),
    DRW(VIndex, VIndex, u8),
    SKP(VIndex),
    SKNP(VIndex),
    LD_DT(VIndex),
    LD_K(VIndex),
    SET_DT(VIndex),
    SET_ST(VIndex),
    ADD_I(VIndex),
    LD_F(VIndex),
    LD_B(VIndex),
    ST_UNTIL(VIndex),
    LD_UNTIL(VIndex)
}
