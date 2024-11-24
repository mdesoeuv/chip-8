mod call_stack;
mod instruction;
mod keypad;
mod memory;
mod screen;

use thiserror::Error;

use call_stack::CallStack;
pub use keypad::{Key, Keypad};
pub use memory::{Address, Memory};
pub use screen::Screen;

pub struct Machine {
    pub registers: [u8; 16],
    pub i_register: Address,
    pub ip_register: Address,
    pub memory: Memory,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub call_stack: CallStack,
    pub screen: Screen,
    pub keypad: Keypad,
}

pub type TickResult = Result<TickFlow, TickError>;

pub enum TickFlow {
    Advance,
    Skip,
    GoTo(Address),
    Wait,
}

#[derive(Error, Debug)]
pub enum TickError {
    #[error(transparent)]
    StackError(#[from] call_stack::Error),
    #[error(transparent)]
    MemoryError(#[from] memory::Error),
    #[error("unimplemented instruction")]
    Unimplemented,
    #[error("unknown instruction")]
    Unknown,
}

pub type RunResult = Result<RunFlow, TickError>;

pub enum RunFlow {
    Wait,
}

#[derive(Debug)]
enum Instruction {
    ClearScreen,
    ReturnFromSubroutine,
    JumpToMachineCode(Address),
    JumpTo(Address),
    ExecuteSubroutine(Address),
    SkipEqTo(Register, u8),
    SkipNeqTo(Register, u8),
    SkipEq(Register, Register),
    StoreValue(Register, u8),
    AddValue(Register, u8),
    StoreRegister(Register, Register),
    Or(Register, Register),
    And(Register, Register),
    Xor(Register, Register),
    AddRegister(Register, Register),
    SubRegister(Register, Register),
    ShiftRight(Register, Register),
    SubRegisterReverse(Register, Register),
    ShiftLeft(Register, Register),
    SkipNeq(Register, Register),
    StoreAddr(Address),
    JumpToOffset(Address),
    StoreRandom(Register, u8),
    DrawSprite(Register, Register, u8),
    SkipIfKeyPressed(Register),
    SkipIfKeyNotPressed(Register),
    StoreDelayTimer(Register),
    WaitForKeypress(Register),
    SetDelayTimer(Register),
    SetSoundTimer(Register),
    AddToI(Register),
    StoreDigitLocation(Register),
    StoreBinaryCoded(Register),
    StoreRegisters(Register),
    LoadRegisters(Register),
}

impl Instruction {
    fn decode(nibbles: [u8; 4]) -> Option<Self> {
        use Instruction::*;
        Some(match nibbles {
            [0, 0, 0xe, 0] => ClearScreen,
            [0, 0, 0xe, 0xe] => ReturnFromSubroutine,
            [0, a, b, c] => JumpToMachineCode(u16_from_nibbles(a, b, c)),
            [1, a, b, c] => JumpTo(u16_from_nibbles(a, b, c)),
            [2, a, b, c] => ExecuteSubroutine(u16_from_nibbles(a, b, c)),
            [3, x, a, b] => SkipEqTo(x, u8_from_nibbles(a, b)),
            [4, x, a, b] => SkipNeqTo(x, u8_from_nibbles(a, b)),
            [5, x, y, 0] => SkipEq(x, y),
            [6, x, a, b] => StoreValue(x, u8_from_nibbles(a, b)),
            [7, x, a, b] => AddValue(x, u8_from_nibbles(a, b)),
            [8, x, y, 0] => StoreRegister(x, y),
            [8, x, y, 1] => Or(x, y),
            [8, x, y, 2] => And(x, y),
            [8, x, y, 3] => Xor(x, y),
            [8, x, y, 4] => AddRegister(x, y),
            [8, x, y, 5] => SubRegister(x, y),
            [8, x, y, 6] => ShiftRight(x, y),
            [8, x, y, 7] => SubRegisterReverse(x, y),
            [8, x, y, 0xe] => ShiftLeft(x, y),
            [9, x, y, 0] => SkipEq(x, y),
            [0xa, a, b, c] => StoreAddr(u16_from_nibbles(a, b, c)),
            [0xb, a, b, c] => JumpToOffset(u16_from_nibbles(a, b, c)),
            [0xc, x, a, b] => StoreRandom(x, u8_from_nibbles(a, b)),
            [0xd, x, y, a] => DrawSprite(x, y, a),
            [0xe, x, 9, 0xe] => SkipIfKeyPressed(x),
            [0xe, x, 0xa, 1] => SkipIfKeyNotPressed(x),
            [0xf, x, 0, 7] => StoreDelayTimer(x),
            [0xf, x, 0, 0xa] => WaitForKeypress(x),
            [0xf, x, 1, 5] => SetDelayTimer(x),
            [0xf, x, 1, 8] => SetSoundTimer(x),
            [0xf, x, 1, 0xe] => AddToI(x),
            [0xf, x, 2, 9] => StoreDigitLocation(x),
            [0xf, x, 3, 3] => StoreBinaryCoded(x),
            [0xf, x, 5, 5] => StoreRegisters(x),
            [0xf, x, 6, 5] => LoadRegisters(x),
            _ => return None,
        })
    }
}

pub type Register = u8;

pub const INSTRUCTION_SIZE: Address = 2;

pub fn u8_from_nibbles(a: u8, b: u8) -> u8 {
    a << 4 | b
}

pub fn u16_from_nibbles(a: u8, b: u8, c: u8) -> u16 {
    (a as u16) << 8 | (b as u16) << 4 | (c as u16)
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            registers: [0; 16],
            i_register: 0,
            ip_register: Memory::PROGRAM_ENTRYPOINT,
            memory: Memory::default(),
            delay_timer: 0,
            sound_timer: 0,
            call_stack: CallStack::new(),
            screen: Screen::default(),
            keypad: Keypad::default(),
        }
    }

    pub fn load_program(&mut self, program: &[u8]) -> Result<(), memory::Error> {
        self.memory.load_program(program)
    }

    pub fn run(&mut self) -> RunResult {
        for _ in 0..200 {
            match self.tick()? {
                TickFlow::Advance => self.ip_register += INSTRUCTION_SIZE,
                TickFlow::Skip => self.ip_register += INSTRUCTION_SIZE * 2,
                TickFlow::GoTo(addr) => self.ip_register = addr,
                TickFlow::Wait => return Ok(RunFlow::Wait),
            }
        }
        Ok(RunFlow::Wait)
    }

    /// Get register X
    /// # Panic
    /// Panics if x isn't in range [0x0, 0xF]
    pub fn register(&self, x: Register) -> u8 {
        self.registers[x as usize]
    }

    /// Get mutable reference to register X
    /// # Panic
    /// Panics if x isn't in range [0x0, 0xF]
    pub fn register_mut(&mut self, x: Register) -> &mut u8 {
        &mut self.registers[x as usize]
    }

    /// Run current instruction without updating [Machine::ip_register]
    pub fn tick(&mut self) -> TickResult {
        let nibbles = self.memory.nibbles_at(self.i_register)?;
        let instruction = Instruction::decode(nibbles).ok_or(TickError::Unknown)?;

        log::trace!("{instruction:?}");
    }

    fn execute(&mut self, instruction: Instruction) -> TickResult {
        match instruction {
            
        }
    }
}
