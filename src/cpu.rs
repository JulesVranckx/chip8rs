use std::time::{Instant, Duration};

mod setup ;
mod registers;
mod stack;
mod ram;
mod components;

use setup::types::{VIndex, VValue, Addr};
use registers::Registers;
use stack::Stack;
use ram::Memory;
use components::{ProgramCounter, DelayTimer, SoundTimer, IndexRegister, FrameBuffer};


enum CpuState {
    IDLE,
    FETCH,
    DECODE,
    EXEC
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
    frequency: u32,
    frequency_counter: u32,
    opcode: CellValue,
    instr: Option<Instruction>,
    state: CpuState
}

impl CPU {

    fn new(frequency: Option<u32>) -> CPU {

        let frequency = match frequency {
            Some(frequency) => frequency,
            None => setup::DEFAULT_FREQUENCY
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
            frequency: frequency,
            frequency_counter: 0u32,
            opcode: 0,
            instr: None,
            state: CpuState::IDLE
        }
    }

    fn next_cycle(&mut self) -> Result<(), &'static str> {

        if off {
            self.state == CpuState::IDLE ;
        }
        else match self.state {
            CpuState::IDLE => {
            self.state == CpuState::FETCH ;
            }

            CpuState::FETCH => {
            self.state == CpuState::DECODE ;
            }
            
            CpuState::DECODE => {
            self.state == CpuState::EXEC;
            }
            
            CpuState::EXEC => {
            self.sate == CpuState::FETCH;
        }

        Ok(())
    }

    fn simulate(&mut self) -> Result<(), &'static str> {

        let t1 = Instant::now();

        //Update the frequency counter for delay and sound registers
        self.frequency_counter = (self.frequency_counter + 1) % (frequency / 60) ;
        
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
        }

        let t2 = t1.elapsed() ;

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

        match self.instr.unwrap() {
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
                self.pc.change(addr)?;
                increase_pc = false;
            },
            Instruction::CALL(addr) => {
                self.stack.push(self.pc.get())?;
                self.pc.change(addr)?;
                increase_pc = false;            
            },
            Instruction::SEi(vx, kk) => {
                if self.v.read(vx)? == kk {
                    self.pc.incr()?; 
                }
            },

            Instruction::SNEi(vx, kk) => {
                if self.v.read(vx)? != kk {
                    self.pc.incr()?; 
                }
            },

            Instruction::SE(vx, vy) => {
                if self.v.read(vx)? == self.v.read(vy)? {
                    self.pc.incr()?; 
                }
            }

            Instruction::LDi(vx,vy) => {
                return Err("Not Implemented yet");
            }
            Instruction::ADDi(vx,vy) => {
                return Err("Not Implemented yet");
            }
            Instruction::SE(vx,vy) => {
                return Err("Not Implemented yet");
            }
            Instruction::LD(vx,vy) => {
                return Err("Not Implemented yet");
            }
            Instruction::OR(vx,vy) => {
                return Err("Not Implemented yet");
            }
            Instruction::AND(vx,vy) => {
                return Err("Not Implemented yet");
            }
            Instruction::XOR(vx,vy) => {
                return Err("Not Implemented yet");
            }
            Instruction::ADD(vx,vy) => {
                return Err("Not Implemented yet");
            }
            Instruction::SUB(vx,vy) => {
                return Err("Not Implemented yet");
            }
            Instruction::SHR(vx,vy) => {
                return Err("Not Implemented yet");
            }
            Instruction::SUBN(vx,vy) => {
                return Err("Not Implemented yet");
            }
            Instruction::SHL(vx,vy) => {
                return Err("Not Implemented yet");
            }
            Instruction::SNE(vx,vy) => {
                return Err("Not Implemented yet");
            }
            Instruction::LD_I(addr) => {
                return Err("Not Implemented yet");
            }
            Instruction::JP_VO(addr) => {
                return Err("Not Implemented yet");
            }
            Instruction::RNDi(vx, kk) => {
                return Err("Not Implemented yet");
            }
            Instruction::DRW(vx,vy, n) => {
                return Err("Not Implemented yet");
            }
            Instruction::SKP(vx) => {
                return Err("Not Implemented yet");
            }
            Instruction::SKNP(vx) => {
                return Err("Not Implemented yet");
            }
            Instruction::LD_DT(vx) => {
                return Err("Not Implemented yet");
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
        };

        if increase_pc {
            self.pc.incr()?;
        }

        Ok(())

    }

}
