use std::time::Duration;
use std::thread::sleep;
use rand::Rng;

pub const MEMORY_SIZE: usize = 0xFFF ;
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
    opcode: CellValue,
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
        Err("FIle loading not implemented yet")
    }

    pub fn loadb(&mut self, filename: &str) -> Result<(), &'static str>{
        Err("File loading not yet implemented")
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
                self.fetch()?;
            }

            CpuState::DECODE => {
                self.decode()?;
            }

            CpuState::EXEC => {
                self.execute()?;
            }
        };

        let sleep_time: u64 = (1e6 as f64 / (self.frequency as f64)) as u64 ;

        if self.frequency_counter == 0 {
            return Err("Internal register gestion not implemented");
        }

        self.st.sound();

        sleep(Duration::from_micros(sleep_time));

        Ok(())

    }

    fn fetch(&self) -> Result<CellValue, &'static str> {
        return Err("fetching not implemented")
    }

    fn decode(&self) -> Result<Instruction, &'static str> {
        return Err("decoding not implemented")
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
                    self.v.write(*vx, (result & 0xFF) as u8)?;
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
                    self.v.write(*vx, (result & 0xFF) as u8)?;
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
                    self.v.write(*vx, (result & 0xFF) as u8)?;
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
                Instruction::JP_VO(addr) => {
                    let v0 = self.v.read(0)?;
                    self.pc.change(*addr + v0 as usize)?;
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
                    return Err("Not Implemented yet");
                }
                Instruction::SET_DT(vx) => {
                    return Err("Not Implemented yet");
                }
                Instruction::SET_ST(vx) => {
                    return Err("Not Implemented yet");
                }
                Instruction::ADD_I(vx) => {
                    return Err("Not Implemented yet");
                }
                Instruction::LD_F(vx) => {
                    return Err("Not Implemented yet");
                }
                Instruction::LD_B(vx) => {
                    return Err("Not Implemented yet");
                }
                Instruction::ST_UNTIL(vx) => {
                    return Err("Not Implemented yet");
                }
                Instruction::LD_UNTIL(vx) => {
                    return Err("Not Implemented yet");
                }
                _ => ()
            }
        }

        if increase_pc {
            self.pc.incr()?;
        }

        Ok(())

    }

}


/// Instruction Set
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
    JP_VO(Addr),
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

// fn CPU_execute_single(instr: &Instruction) -> Result<(), io::Error> {

//     
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn execute_JP() {
//         let instr = Instruction::JP(0x100);

//         CPU_execute_single(&instr);

//         unsafe{
//             assert_eq!(PC, 0x100);
//         }
//     }

//     #[test]
//     fn execute_CALL() {
//         let instr = Instruction::CALL(0x100);

//         let previous_pc;
//         let previous_sp;
//         unsafe{
//             previous_sp = STACK_POINTER ;
//             previous_pc = PC;
//         }

//         CPU_execute_single(&instr);

//         unsafe{
//             assert_eq!(PC, 0x100);
//             assert_eq!(STACK[STACK_POINTER], previous_pc as u16);
//             assert_eq!(STACK_POINTER - 1, previous_sp);
//         }
//     }
// }