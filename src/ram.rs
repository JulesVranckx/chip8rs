mod setup;


/// Memory structure
pub struct Memory{
    cells: [CellValue; MEMORY_SIZE]
}

impl Memory {
    pub fn new() -> Memory {
        Memory{
            cells: [0; MEMORY_SIZE]
        }
    }

    pub fn read(&self, index: Addr) -> Result<CellValue, &'static str>{
        
        if index >= MEMORY_SIZE {
            return Err("Memory index too high. Please use -c to see CHIP-8 caracteristics")
        }
        
        Ok(self.cells[index])
    }

    pub fn write(&mut self, index: Addr, value: CellValue) -> Result<(), &'static str> {
        
        if index >= MEMORY_SIZE {
            return Err("Memory index too high. Please use -c to see CHIP-8 caracteristics")
        }

        self.cells[index] = value ;

        Ok(())
    }
}