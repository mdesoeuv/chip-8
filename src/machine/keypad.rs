/// [Keypad](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#keypad-input)
/// 1 2 3 C
/// 4 5 6 D
/// 7 8 9 E
/// A 0 B F

pub type Key = u8;

#[derive(Default)]
pub struct Keypad {
    pressed: [bool; Self::KEY_COUNT],
    just_pressed: [bool; Self::KEY_COUNT],
}

impl Keypad {
    pub const KEY_COUNT: usize = 16;

    pub fn reset(&mut self) {
        self.just_pressed = [false; Self::KEY_COUNT];
    }

    pub fn press(&mut self, key: Key) {
        if self.pressed[key as usize] == false {
            self.just_pressed[key as usize] = true;
        }
        self.pressed[key as usize] = true;
    }

    pub fn release(&mut self, key: Key) {
        self.pressed[key as usize] = false;
    }

    pub fn pressed(&self, key: Key) -> bool {
        match self.pressed.get(key as usize) {
            Some(pressed) => *pressed,
            None => false,
        }
    }

    pub fn just_pressed(&self) -> Option<Key> {
        self.just_pressed
            .iter()
            .position(|pressed| *pressed)
            .map(|index| index as Key)
    }
}
