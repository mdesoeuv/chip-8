struct Machine {
    registers: [u8; 16],
    i_register: Address,
    ip_register: Address,
    memory: [u8; 4096],
    delay_timer: u8,
    sound_timer: u8,
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
        Machine {
            registers: [0; 16],
            i_register: 0,
            ip_register: 0,
            memory: [0; 4096],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    fn register(&self, x: Register) -> u8 {
        self.registers[x as usize]
    }

    fn register_mut(&mut self, x: Register) -> &mut u8 {
        &mut self.registers[x as usize]
    }

    fn nibbles_at(&self, addr: Address) -> [u8; 4] {
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
            _ => unimplemented!(),
        }
    }

    /// 00E0: Clear the screen
    fn clear_screen(&mut self) {
        unimplemented!()
    }

    /// Execute subroutine starting at address NNN
    fn execute_subroutine(&mut self, addr: Address) {
        unimplemented!()
    }

    fn go_to_next_instruction(&mut self) {
        self.ip_register += INSTRUCTION_SIZE;
    }

    /// 3XNN: Skip the following instruction if the value of register VX equals NN
    fn skip_eq_to(&mut self, x: Register, value: u8) {
        if self.register(x) == value {
            self.go_to_next_instruction()
        }
    }

    /// 4XNN: Skip the following instruction if the value of register VX is not equal to NN
    fn skip_neq_to(&mut self, x: Register, value: u8) {
        if self.register(x) != value {
            self.go_to_next_instruction()
        }
    }

    /// 5XY0: Skip the following instruction if the value of register VX is equal to the value of register VY
    fn skip_eq(&mut self, x: Register, y: Register) {
        if self.register(x) == self.register(y) {
            self.go_to_next_instruction()
        }
    }

    /// 00EE: Return from a subroutine
    fn return_from_subroutine(&mut self) {
        unimplemented!()
    }

    /// 1NNN: Jump to address NNN
    fn jump_to(&mut self, addr: Address) {
        self.ip_register = addr;
    }

    /// 0NNN: Execute machine language subroutine at address NNN
    fn jump_to_machine_code(&mut self, _addr: Address) {
        unimplemented!()
    }

    /// 6XNN: Store number NN in register VX
    fn store_value(&mut self, x: Register, value: u8) {
        *self.register_mut(x) = value;
    }

    /// 7XNN: Add the value NN to register VX
    fn add_value(&mut self, x: Register, value: u8) {
        *self.register_mut(x) = self.register(x).wrapping_add(value);
    }

    /// 8XY0: Store the value of register VY in register VX
    fn store_register(&mut self, x: Register, y: Register) {
        *self.register_mut(x) = self.register(y);
    }

    /// 8XY1: Set VX to VX OR VY
    fn or(&mut self, x: Register, y: Register) {
        *self.register_mut(x) |= self.register(y);
    }

    /// 8XY2: Set VX to VX AND VY
    fn and(&mut self, x: Register, y: Register) {
        *self.register_mut(x) &= self.register(y);
    }

    /// 8XY3: Set VX to VX XOR VY
    fn xor(&mut self, x: Register, y: Register) {
        *self.register_mut(x) ^= self.register(y);
    }

    /// 8XY4: Add the value of register VY to register VX
    /// Set VF to 01 if a carry occurs
    /// Set VF to 00 if a carry does not occur
    fn add_register(&mut self, x: Register, y: Register) {
        let (result, overflowed) = self.register(x).overflowing_add(self.register(y));
        *self.register_mut(x) = result;
        *self.register_mut(0xF) = u8::from(overflowed);
    }

    /// 8XY5: Subtract the value of register VY from register VX
    /// Set VF to 00 if a borrow occurs
    /// Set VF to 01 if a borrow does not occur
    fn sub_register(&mut self, x: Register, y: Register) {
        let (result, overflowed) = self.register(x).overflowing_sub(self.register(y));
        *self.register_mut(x) = result;
        *self.register_mut(0xF) = u8::from(overflowed);
    }

    /// 8XY6: Store the value of register VY shifted right one bit in register VX
    /// Set register VF to the least significant bit prior to the shift
    /// VY is unchanged
    fn shift_right(&mut self, x: Register, y: Register) {
        *self.register_mut(0xF) = self.register(y) & 1;
        *self.register_mut(x) >>= self.register(y)
    }

    /// 8XY7: Set register VX to the value of VY minus VX
    /// Set VF to 00 if a borrow occurs
    /// Set VF to 01 if a borrow does not occur
    fn sub_register_reverse(&mut self, x: Register, y: Register) {
        let (result, overflowed) = self.register(y).overflowing_sub(self.register(x));
        *self.register_mut(x) = result;
        *self.register_mut(0xF) = u8::from(overflowed);
    }

    /// 8XYE: Store the value of register VY shifted left one bit in register VX
    /// Set register VF to the most significant bit prior to the shift
    /// VY is unchanged
    fn shift_left(&mut self, x: Register, y: Register) {
        let msb = self.register(y) & 0b_1000_0000;
        *self.register_mut(x) = self.register(y) << 1;
        *self.register_mut(0xF) = msb;
    }

    /// 9XY0: Skip the following instruction if the value of register VX is not equal to the value of register VY
    fn skip_neq(&mut self, x: Register, y: Register) {
        if self.register(x) != self.register(y) {
            self.go_to_next_instruction()
        }
    }

    /// ANNN: Store memory address NNN in register I
    fn store_addr(&mut self, addr: Address) {
        self.i_register = addr;
    }

    /// BNNN: Jump to address NNN + V0
    fn jump_to_offset(&mut self, reference: u16) {
        unimplemented!()
    }

    /// CNNN: Set VX to a random number with a mask of NN
    fn store_random(&mut self, x: Register, mask: u8) {
        *self.register_mut(x) = todo!()
    }

    /// DXYN: Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in I
    /// Set VF to 01 if any set pixels are changed to unset, and 00 otherwise
    fn draw_sprite(&mut self, x: Register, y: Register, value: u8) {
        unimplemented!()
    }

    /// EX9E: Skip the following instruction if the key corresponding to the hex value currently stored in register VX is pressed
    fn skip_if_key_pressed(&mut self, x: Register) {
        unimplemented!()
    }

    /// EXA1: Skip the following instruction if the key corresponding to the hex value currently stored in register VX is not pressed
    fn skip_if_key_not_pressed(&mut self, x: Register) {
        unimplemented!()
    }

    /// FX07: Store the current value of the delay timer in register VX
    fn store_delay_timer(&mut self, x: Register) {
        *self.register_mut(x) = self.delay_timer;
    }

    /// FX0A: Wait for a keypress and store the result in register VX
    fn wait_for_keypress(&mut self, x: Register) {
        unimplemented!()
    }

    /// FX15: Set the delay timer to the value of register VX
    fn set_delay_timer(&mut self, x: Register) {
        self.delay_timer = self.register(x);
    }

    /// FX18: Set the sound timer to the value of register VX
    fn set_sound_timer(&mut self, x: Register) {
        self.sound_timer = self.register(x)
    }

    /// FX1E: Add the value stored in register VX to register I
    fn add_to_i(&mut self, x: Register) {
        let value = u16::from(self.register(x));
        self.i_register = self.i_register.wrapping_add(value);
    }

    /// FX29: Set I to the memory address of the sprite data corresponding to the hexadecimal digit stored in register VX
    fn store_digit_location(&mut self, x: Register) {
        unimplemented!()
    }

    /// FX33: Store the [binary-coded decimal](https://en.wikipedia.org/wiki/Binary-coded_decimal)
    /// equivalent of the value stored in register VX at addresses I, I + 1, and I + 2
    fn store_binary_coded(&mut self, x: Register) {
        unimplemented!()
    }

    /// FX55: Store the values of registers V0 to VX inclusive in memory starting at address I
    /// I is set to I + X + 1 after operation
    fn store_registers(&mut self, x: Register) {
        unimplemented!()
    }

    /// FX65: Fill registers V0 to VX inclusive with the values stored in memory starting at address I
    /// I is set to I + X + 1 after operation
    fn load_registers(&mut self, x: Register) {
        unimplemented!()
    }
}

fn main() {
    let mut machine = Machine::new();
    machine.tick();
}
