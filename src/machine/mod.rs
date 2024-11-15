mod call_stack;
mod instruction;
mod memory;
mod screen;

use thiserror::Error;

use call_stack::CallStack;
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
        }
    }

    pub fn load_program(&mut self, program: &[u8]) -> Result<(), memory::Error> {
        self.memory.load_program(program)
    }

    pub fn run(&mut self) -> RunResult {
        loop {
            match self.tick()? {
                TickFlow::Advance => self.ip_register += INSTRUCTION_SIZE,
                TickFlow::Skip => self.ip_register += INSTRUCTION_SIZE * 2,
                TickFlow::GoTo(addr) => self.ip_register = addr,
                TickFlow::Wait => return Ok(RunFlow::Wait),
            }
        }
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
        match self.memory.nibbles_at(self.ip_register)? {
            [0, 0, 0xe, 0] => self.clear_screen(),
            [0, 0, 0xe, 0xe] => self.return_from_subroutine(),
            [0, a, b, c] => self.jump_to_machine_code(u16_from_nibbles(a, b, c)),
            [1, a, b, c] => self.jump_to(u16_from_nibbles(a, b, c)),
            [2, a, b, c] => self.execute_subroutine(u16_from_nibbles(a, b, c)),
            [3, x, a, b] => self.skip_eq_to(x, u8_from_nibbles(a, b)),
            [4, x, a, b] => self.skip_neq_to(x, u8_from_nibbles(a, b)),
            [5, x, y, 0] => self.skip_eq(x, y),
            [6, x, a, b] => self.store_value(x, u8_from_nibbles(a, b)),
            [7, x, a, b] => self.add_value(x, u8_from_nibbles(a, b)),
            [8, x, y, 0] => self.store_register(x, y),
            [8, x, y, 1] => self.or(x, y),
            [8, x, y, 2] => self.and(x, y),
            [8, x, y, 3] => self.xor(x, y),
            [8, x, y, 4] => self.add_register(x, y),
            [8, x, y, 5] => self.sub_register(x, y),
            [8, x, y, 6] => self.shift_right(x, y),
            [8, x, y, 7] => self.sub_register_reverse(x, y),
            [8, x, y, 0xe] => self.shift_left(x, y),
            [9, x, y, 0] => self.skip_neq(x, y),
            [0xa, a, b, c] => self.store_addr(u16_from_nibbles(a, b, c)),
            [0xb, a, b, c] => self.jump_to_offset(u16_from_nibbles(a, b, c)),
            [0xc, x, a, b] => self.store_random(x, u8_from_nibbles(a, b)),
            [0xd, x, y, a] => self.draw_sprite(x, y, a),
            [0xe, x, 9, 0xe] => self.skip_if_key_pressed(x),
            [0xe, x, 0xa, 1] => self.skip_if_key_not_pressed(x),
            [0xf, x, 0, 7] => self.store_delay_timer(x),
            [0xf, x, 0, 0xa] => self.wait_for_keypress(x),
            [0xf, x, 1, 5] => self.set_delay_timer(x),
            [0xf, x, 1, 8] => self.set_sound_timer(x),
            [0xf, x, 1, 0xe] => self.add_to_i(x),
            [0xf, x, 2, 9] => self.store_digit_location(x),
            [0xf, x, 3, 3] => self.store_binary_coded(x),
            [0xf, x, 5, 5] => self.store_registers(x),
            [0xf, x, 6, 5] => self.load_registers(x),
            _ => Err(TickError::Unknown),
        }
    }
}
