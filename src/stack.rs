
/// Stack
pub struct Stack {
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