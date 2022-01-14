mod setup;

/// Registers strucure
/// Holds a 16 bytes long array
/// Defines methods to create a new instance, read from registers and write in it
pub struct Registers{
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