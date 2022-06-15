use crate::instructions;

pub struct VM {
    ijvm: IJVMFile,
    pc: usize,
    stack: Vec<i32>,
    running: bool
}

impl VM {
    pub fn new(ijvm: IJVMFile) -> VM {
        VM {
            ijvm,
            pc: 0,
            stack: Vec::new(),
            running: true
        }
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.ijvm.text[self.pc];
            match instruction {
                0xFF => {
                    println!("NOP");
                    self.running = false;
                }
            }
            self.pc += 1;
        }
    }
}