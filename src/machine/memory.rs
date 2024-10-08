use std::ops::Range;
use thiserror::Error;

pub type Address = u16;

pub struct Memory([u8; Self::SIZE]);

impl Default for Memory {
    fn default() -> Self {
        Self([0; Self::SIZE])
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("out of bound access at {0}")]
    OutOfBound(Address),
    #[error("out of bound access at {0:?}")]
    RangeOutOfBound(Range<Address>)
}

impl Memory {
    pub const SIZE: usize = 4096;
    pub const MEMORY_END: Address = Self::SIZE as Address;
    pub const PROGRAM_ENTRYPOINT: Address = 0x200;

    pub fn load_program(&mut self, program: &[u8]) -> Result<(), Error> {
        let program_memory = &mut self.range_mut(Self::PROGRAM_ENTRYPOINT..Self::MEMORY_END)?;
        if program_memory.len() < program.len() {
            // TODO(Mehdi): Change :eyes:
            return Err(Error::OutOfBound(Self::MEMORY_END));
        }
        program_memory[..program.len()].copy_from_slice(program);
        Ok(())
    }

    pub fn get(&self, addr: Address) -> Result<u8, Error> {
        self.0.get(addr as usize).copied().ok_or(Error::OutOfBound(addr))
    }

    pub fn get_mut(&mut self, addr: Address) -> Result<&mut u8, Error> {
        self.0.get_mut(addr as usize).ok_or(Error::OutOfBound(addr))
    }

    pub fn range(&self, range: Range<Address>) -> Result<&[u8], Error> {
        self.0
            .get(range.start as usize..range.end as usize)
            .ok_or(Error::RangeOutOfBound(range))
    }

    pub fn range_mut(&mut self, range: Range<Address>) -> Result<&mut [u8], Error> {
        self.0
            .get_mut(range.start as usize..range.end as usize)
            .ok_or(Error::RangeOutOfBound(range))
    }

    /// Get 4 nibbles at an address
    /// # Panic
    /// Panics `addr` or `addr + 1` are out of [Machine::memory] range
    pub fn nibbles_at(&self, addr: Address) -> Result<[u8; 4], Error> {
        let a = self.get(addr)?;
        let b = self.get(addr + 1)?;
        Ok([a & 0xf, a >> 4, b & 0xf, b >> 4])
    }
}