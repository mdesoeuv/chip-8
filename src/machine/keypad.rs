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
    just_released: [bool; Self::KEY_COUNT],
}

impl Keypad {
    pub const KEY_COUNT: usize = 16;

    pub fn reset(&mut self) {
        self.just_pressed = [false; Self::KEY_COUNT];
        self.just_released = [false; Self::KEY_COUNT];
    }

    pub fn press(&mut self, key: Key) {
        log::debug!("press: {key:X}");
        if self.pressed[key as usize] == false {
            self.just_pressed[key as usize] = true;
        }
        self.pressed[key as usize] = true;
    }

    pub fn release(&mut self, key: Key) {
        log::debug!("release: {key:X}");
        self.just_released[key as usize] = self.pressed(key);
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

    pub fn just_released(&self) -> Option<Key> {
        self.just_released
            .iter()
            .position(|released| *released)
            .map(|index| index as Key)
    }
}
