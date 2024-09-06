struct Machine {
    registers: [u8;16],
    i_register: Address,
    ip_register: Address,
    memory: [u8; 4096],
}

type Address = u16;
type Register = u8;
const INSTRUCTION_SIZE: Address = 2;

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

    fn register(&self, x: Register) -> u8 {
        return self.registers[x as usize]
    }

    fn nibbles_at(&self, addr: Address) -> [u8;4] {
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
            [2, a, b, c] => self.execute_subroutine(u16_from_nibbles(a, b, c)),
            [3, x, a, b] => self.skip_eq_to(x, u8_from_nibbles(a, b)),
            [4, x, a, b] => self.skip_neq_to(x, u8_from_nibbles(a, b)),
            _ => unimplemented!()
        }
    }

    fn clear_screen(&mut self) {
        unimplemented!()
    }

    fn execute_subroutine(&mut self, addr: Address) {
        unimplemented!()
    }

    fn go_to_next_instruction(&mut self) {
        self.ip_register += INSTRUCTION_SIZE;
    }

    fn skip_eq_to(&mut self, x: Register, value: u8) {
        if self.register(x) == value {
            self.go_to_next_instruction()
        }
    }

    fn skip_neq_to(&mut self, x: Register, value: u8) {
        if self.register(x) != value {
            self.go_to_next_instruction()
        }
    }

    fn skip_eq(&mut self, x: Register, y: Register) {
        if self.register(x) == self.register(y) {
            self.go_to_next_instruction()
        }
    }

    fn return_from_subroutine(&mut self) {
        unimplemented!()
    }

    fn jump_to(&mut self, addr: Address) {
        self.ip_register = addr;
    }

    fn jump_to_machine_code(&mut self, _addr: Address) {
        unimplemented!()
    }
}

fn main() {
    let mut machine = Machine::new();
    machine.tick();
}
