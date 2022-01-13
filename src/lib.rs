use std::io ;

pub const MEMORY_SIZE: usize = 4096 ;
pub const GP_REGISTERS_COUNT: usize = 16 ;
pub const STACK_SIZE: usize = 16 ;
pub const PROGRAM_START: usize = 0x200 ;
pub const FRAME_BUFFER_LENGTH: usize = 8;
pub const FRAME_BUFFER_HEIGHT: usize = 32;

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
            return Err("Register Index too high. Please use -c to see CHIP-8 caracteristics")
        }

        self.regs[index] = value ;

        Ok(())
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
struct DelayTimer;

impl DelayTimer {

    fn new() -> DelayTimer {
        DelayTimer
    }
}

/// Sound Timer
struct SoundTimer;

impl SoundTimer {

    fn new() -> SoundTimer {
        SoundTimer
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

        self.register += 1 ;

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
struct IndexRegister;

impl IndexRegister {

    fn new() -> IndexRegister {
        IndexRegister
    }
}

/// CPU
pub struct CPU {
    ram: Memory,
    v: Registers,
    stack: Stack,
    pc: ProgramCounter,
    index_register: IndexRegister,
    dt: DelayTimer,
    st: SoundTimer,
    frame_buff: FrameBuffer,
}

impl CPU {

    fn new() -> CPU {
        CPU {
            ram: Memory::new(),
            v: Registers::new(),
            stack: Stack::new(),
            pc: ProgramCounter::new(),
            index_register: IndexRegister::new(),
            dt: DelayTimer::new(),
            st: SoundTimer::new(),
            frame_buff: FrameBuffer::new(),
        }
    }

    fn fetch(&self, addr: Addr) -> Result<CellValue, &'static str> {
        return Err("fetching not implemented")
    }

    fn decode(&self, data: CellValue) -> Result<&Instruction, 'static str> {
        return Err("decoding not implemented    ")
    }

    fn execute(&mut self, instr: &Instruction) -> Result<(), &'static str> {
        
        let mut increase_pc = true ;

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

            Instruction::LDi(VIndex, VValue) => {
                return Err("Not Implemented yet");
            }
            Instruction::ADDi(VIndex, VValue) => {
                return Err("Not Implemented yet");
            }
            Instruction::SE(VIndex, VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::LD(VIndex, VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::OR(VIndex, VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::AND(VIndex, VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::XOR(VIndex, VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::ADD(VIndex, VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::SUB(VIndex, VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::SHR(VIndex, VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::SUBN(VIndex, VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::SHL(VIndex, VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::SNE(VIndex, VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::LD_I(Addr) => {
                return Err("Not Implemented yet");
            }
            Instruction::JP_VO(Addr) => {
                return Err("Not Implemented yet");
            }
            Instruction::RNDi(VIndex, VValue) => {
                return Err("Not Implemented yet");
            }
            Instruction::DRW(VIndex, VIndex, u8) => {
                return Err("Not Implemented yet");
            }
            Instruction::SKP(VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::SKNP(VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::LD_DT(VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::LD_K(VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::SET_DT(VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::SET_ST(VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::ADD_I(VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::LD_F(VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::LD_B(VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::ST_UNTIL(VIndex) => {
                return Err("Not Implemented yet");
            }
            Instruction::LD_UNTIL(VIndex) => {
                return Err("Not Implemented yet");
            }
            _ => ()
        };

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
    SHR(VIndex, VIndex),
    SUBN(VIndex, VIndex),
    SHL(VIndex, VIndex),
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