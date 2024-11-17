use super::{
    memory, Address, Machine, Memory, Register, TickError, TickFlow, TickResult, INSTRUCTION_SIZE,
};

impl Machine {
    /// 2NNN: Execute subroutine starting at address NNN
    pub fn execute_subroutine(&mut self, addr: Address) -> TickResult {
        self.call_stack.push(self.ip_register + INSTRUCTION_SIZE)?;
        Ok(TickFlow::GoTo(addr))
    }

    /// 00E0: Clear the screen
    pub fn clear_screen(&mut self) -> TickResult {
        self.screen.clear();
        Ok(TickFlow::Advance)
    }

    /// 3XNN: Skip the following instruction if the value of register VX equals NN
    pub fn skip_eq_to(&mut self, x: Register, value: u8) -> TickResult {
        if self.register(x) == value {
            Ok(TickFlow::Skip)
        } else {
            Ok(TickFlow::Advance)
        }
    }

    /// 4XNN: Skip the following instruction if the value of register VX is not equal to NN
    pub fn skip_neq_to(&mut self, x: Register, value: u8) -> TickResult {
        if self.register(x) != value {
            Ok(TickFlow::Skip)
        } else {
            Ok(TickFlow::Advance)
        }
    }

    /// 5XY0: Skip the following instruction if the value of register VX is equal to the value of register VY
    pub fn skip_eq(&mut self, x: Register, y: Register) -> TickResult {
        if self.register(x) == self.register(y) {
            Ok(TickFlow::Skip)
        } else {
            Ok(TickFlow::Advance)
        }
    }

    /// 00EE: Return from a subroutine
    pub fn return_from_subroutine(&mut self) -> TickResult {
        let addr = self.call_stack.pop()?;
        Ok(TickFlow::GoTo(addr))
    }

    /// 1NNN: Jump to address NNN
    pub fn jump_to(&mut self, addr: Address) -> TickResult {
        Ok(TickFlow::GoTo(addr))
    }

    /// 0NNN: Execute machine language subroutine at address NNN
    pub fn jump_to_machine_code(&mut self, _addr: Address) -> TickResult {
        Err(TickError::Unimplemented)
    }

    /// 6XNN: Store number NN in register VX
    pub fn store_value(&mut self, x: Register, value: u8) -> TickResult {
        *self.register_mut(x) = value;
        Ok(TickFlow::Advance)
    }

    /// 7XNN: Add the value NN to register VX
    pub fn add_value(&mut self, x: Register, value: u8) -> TickResult {
        *self.register_mut(x) = self.register(x).wrapping_add(value);
        Ok(TickFlow::Advance)
    }

    /// 8XY0: Store the value of register VY in register VX
    pub fn store_register(&mut self, x: Register, y: Register) -> TickResult {
        *self.register_mut(x) = self.register(y);
        Ok(TickFlow::Advance)
    }

    /// 8XY1: Set VX to VX OR VY
    pub fn or(&mut self, x: Register, y: Register) -> TickResult {
        *self.register_mut(x) |= self.register(y);
        Ok(TickFlow::Advance)
    }

    /// 8XY2: Set VX to VX AND VY
    pub fn and(&mut self, x: Register, y: Register) -> TickResult {
        *self.register_mut(x) &= self.register(y);
        Ok(TickFlow::Advance)
    }

    /// 8XY3: Set VX to VX XOR VY
    pub fn xor(&mut self, x: Register, y: Register) -> TickResult {
        *self.register_mut(x) ^= self.register(y);
        Ok(TickFlow::Advance)
    }

    /// 8XY4: Add the value of register VY to register VX
    /// Set VF to 01 if a carry occurs
    /// Set VF to 00 if a carry does not occur
    pub fn add_register(&mut self, x: Register, y: Register) -> TickResult {
        let (result, overflowed) = self.register(x).overflowing_add(self.register(y));
        *self.register_mut(x) = result;
        *self.register_mut(0xF) = u8::from(overflowed);
        Ok(TickFlow::Advance)
    }

    /// 8XY5: Subtract the value of register VY from register VX
    /// Set VF to 00 if a borrow occurs
    /// Set VF to 01 if a borrow does not occur
    pub fn sub_register(&mut self, x: Register, y: Register) -> TickResult {
        let (result, overflowed) = self.register(x).overflowing_sub(self.register(y));
        *self.register_mut(x) = result;
        *self.register_mut(0xF) = u8::from(overflowed);
        Ok(TickFlow::Advance)
    }

    /// 8XY6: Store the value of register VY shifted right one bit in register VX
    /// Set register VF to the least significant bit prior to the shift
    /// VY is unchanged
    pub fn shift_right(&mut self, x: Register, y: Register) -> TickResult {
        *self.register_mut(0xF) = self.register(y) & 1;
        *self.register_mut(x) = self.register(y) >> 1;
        Ok(TickFlow::Advance)
    }

    /// 8XY7: Set register VX to the value of VY minus VX
    /// Set VF to 00 if a borrow occurs
    /// Set VF to 01 if a borrow does not occur
    pub fn sub_register_reverse(&mut self, x: Register, y: Register) -> TickResult {
        let (result, overflowed) = self.register(y).overflowing_sub(self.register(x));
        *self.register_mut(x) = result;
        *self.register_mut(0xF) = u8::from(overflowed);
        Ok(TickFlow::Advance)
    }

    /// 8XYE: Store the value of register VY shifted left one bit in register VX
    /// Set register VF to the most significant bit prior to the shift
    /// VY is unchanged
    pub fn shift_left(&mut self, x: Register, y: Register) -> TickResult {
        let msb = self.register(y) & 0b_1000_0000;
        *self.register_mut(x) = self.register(y) << 1;
        *self.register_mut(0xF) = msb;
        Ok(TickFlow::Advance)
    }

    /// 9XY0: Skip the following instruction if the value of register VX is not equal to the value of register VY
    pub fn skip_neq(&mut self, x: Register, y: Register) -> TickResult {
        if self.register(x) != self.register(y) {
            Ok(TickFlow::Skip)
        } else {
            Ok(TickFlow::Advance)
        }
    }

    /// ANNN: Store memory address NNN in register I
    pub fn store_addr(&mut self, addr: Address) -> TickResult {
        self.i_register = addr;
        Ok(TickFlow::Advance)
    }

    /// BNNN: Jump to address NNN + V0
    pub fn jump_to_offset(&mut self, reference: Address) -> TickResult {
        Ok(TickFlow::GoTo(reference + self.register(0) as Address))
    }

    /// CNNN: Set VX to a random number with a mask of NN
    pub fn store_random(&mut self, x: Register, mask: u8) -> TickResult {
        *self.register_mut(x) = rand::random::<u8>() & mask;
        Ok(TickFlow::Advance)
    }

    /// DXYN: Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in I
    /// Set VF to 01 if any set pixels are changed to unset, and 00 otherwise
    pub fn draw_sprite(&mut self, x: Register, y: Register, line_count: u8) -> TickResult {
        let x = self.register(x) as usize;
        let y = self.register(y) as usize;

        let sprite = self
            .memory
            .range_mut(self.i_register..self.i_register + line_count as Address)?;

        self.screen.draw_sprite(x, y, sprite);
        self.i_register += line_count as Address;
        Ok(TickFlow::Advance)
    }

    /// EX9E: Skip the following instruction if the key corresponding to the hex value currently stored in register VX is pressed
    pub fn skip_if_key_pressed(&mut self, x: Register) -> TickResult {
        if self.keypad.pressed(self.register(x)) {
            Ok(TickFlow::Skip)
        } else {
            Ok(TickFlow::Advance)
        }
    }

    /// EXA1: Skip the following instruction if the key corresponding to the hex value currently stored in register VX is not pressed
    pub fn skip_if_key_not_pressed(&mut self, x: Register) -> TickResult {
        if self.keypad.pressed(self.register(x)) {
            Ok(TickFlow::Skip)
        } else {
            Ok(TickFlow::Advance)
        }
    }

    /// FX07: Store the current value of the delay timer in register VX
    pub fn store_delay_timer(&mut self, x: Register) -> TickResult {
        *self.register_mut(x) = self.delay_timer;
        Ok(TickFlow::Advance)
    }

    /// FX0A: Wait for a keypress and store the result in register VX
    pub fn wait_for_keypress(&mut self, x: Register) -> TickResult {
        match self.keypad.just_pressed() {
            Some(key) => *self.register_mut(x) = key,
            None => return Ok(TickFlow::Wait),
        }
        Ok(TickFlow::Advance)
    }

    /// FX15: Set the delay timer to the value of register VX
    pub fn set_delay_timer(&mut self, x: Register) -> TickResult {
        self.delay_timer = self.register(x);
        Ok(TickFlow::Advance)
    }

    /// FX18: Set the sound timer to the value of register VX
    pub fn set_sound_timer(&mut self, x: Register) -> TickResult {
        self.sound_timer = self.register(x);
        Ok(TickFlow::Advance)
    }

    /// FX1E: Add the value stored in register VX to register I
    pub fn add_to_i(&mut self, x: Register) -> TickResult {
        let value = u16::from(self.register(x));
        self.i_register = self.i_register.wrapping_add(value);
        Ok(TickFlow::Advance)
    }

    /// FX29: Set I to the memory address of the sprite data corresponding to the hexadecimal digit stored in register VX
    pub fn store_digit_location(&mut self, x: Register) -> TickResult {
        self.i_register = Memory::FONT_LOCATION + self.register(x) as u16 * memory::GLYPH_SIZE;
        Ok(TickFlow::Advance)
    }

    /// FX33: Store the [binary-coded decimal](https://en.wikipedia.org/wiki/Binary-coded_decimal)
    /// equivalent of the value stored in register VX at addresses I, I + 1, and I + 2
    pub fn store_binary_coded(&mut self, x: Register) -> TickResult {
        let n = self.register(x);

        *self.memory.get_mut(self.i_register)? = n / 100;
        *self.memory.get_mut(self.i_register + 1)? = (n / 10) % 10;
        *self.memory.get_mut(self.i_register + 2)? = n % 10;

        Ok(TickFlow::Advance)
    }

    /// FX55: Store the values of registers V0 to VX inclusive in memory starting at address I
    /// I is set to I + X + 1 after operation
    pub fn store_registers(&mut self, x: Register) -> TickResult {
        for i in 0..=x {
            *self.memory.get_mut(self.i_register)? = self.register(i);
            self.i_register += 1;
        }
        Ok(TickFlow::Advance)
    }

    /// FX65: Fill registers V0 to VX inclusive with the values stored in memory starting at address I
    /// I is set to I + X + 1 after operation
    pub fn load_registers(&mut self, x: Register) -> TickResult {
        for i in 0..=x {
            *self.register_mut(i) = self.memory.get(self.i_register)?;
            self.i_register += 1;
        }
        Ok(TickFlow::Advance)
    }
}
