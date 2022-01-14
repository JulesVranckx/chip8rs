mod const_values{
    pub const MEMORY_SIZE: usize = 0xFFF ;
    pub const GP_REGISTERS_COUNT: usize = 16 ;
    pub const STACK_SIZE: usize = 16 ;
    pub const PROGRAM_START: usize = 0x200 ;
    pub const FRAME_BUFFER_LENGTH: usize = 8;
    pub const FRAME_BUFFER_HEIGHT: usize = 32;
    pub const DEFAULT_FREQUENCY: u32 = 600;
}

mod types {
    pub type Addr = usize ;
    pub type StackAdress = usize ;
    pub type VIndex = usize ;
    pub type CellValue = u8 ;
    pub type StackValue = Addr ;
    pub type VValue = u8 ;
}