use thiserror::Error;

use super::Address;

pub struct CallStack {
    stack: [Address; 12],
    top: usize,
}


#[derive(Error, Debug)]
pub enum Error {
    #[error("overflow")]
    Overflow,
    #[error("underflow")]
    Underflow,
}

impl CallStack {
    pub fn new() -> Self {
        Self {
            stack: [0; 12],
            top: 0,
        }
    }

    pub fn push(&mut self, addr: Address) -> Result<(), Error> {
        match self.stack.get_mut(self.top) {
            Some(top) => {
                *top = addr;
                self.top += 1;
                Ok(())
            }
            None => Err(Error::Overflow),
        }
    }

    pub fn pop(&mut self) -> Result<Address, Error> {
        match self.top.checked_sub(1) {
            Some(result) => {
                self.top = result;
                Ok(self.stack[self.top])
            }
            None => Err(Error::Underflow),
        }
    }
}
