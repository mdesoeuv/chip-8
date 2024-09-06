use std::default;

struct Machine {
    registers: [u8;16],
    i_register: u16,
    ip_register: u16,
    memory: [u8; 4096],
}

fn u8_from_nibbles(a: u8, b: u8) -> u8 {
    a << 4 & b
}

fn u16_from_nibbles(a: u8, b: u8, c: u8) -> u16 {
    (a as u16) << 8 & (b as u16) << 4 & (c as u16)
}

impl Machine {
    fn new() -> Self {
        Machine{
            registers: [0;16],
            i_register: 0,
            ip_register: 0,
            memory: [0;4096],
        }
    }

    fn nibbles_at(&self, addr: u16) -> [u8;4] {
        let a = self.memory[addr as usize];
        let b = self.memory[addr as usize + 1];
        [a & 0xf, a >> 4, b & 0xf, b >> 4]
    }

    fn tick(&mut self) {
        match self.nibbles_at(self.ip_register) {
            [0, 0, 0xe, 0] => self.clear_screen(),
            [0, 0, 0xe, 0xe] => self.return_from_subroutine(),
            [0, a, b, c] => self.jump_to_machine_code(u16_from_nibbles(a, b, c)),
            [1, a, b, c] => self.jump_to(u16_from_nibbles(a, b, c)),
            _ => unimplemented!()
        }
    }

    fn clear_screen(&mut self) {
        unimplemented!()
    }

    fn execute_subroutine(&mut self) {
        unimplemented!()
    }

    fn return_from_subroutine(&mut self) {
        unimplemented!()
    }

    fn jump_to(&mut self, addr: u16) {
        unimplemented!()
    }

    fn jump_to_machine_code(&mut self, _addr: u16) {
        unimplemented!()
    }
}

fn main() {
    
}
