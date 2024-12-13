use super::{u16_from_nibbles, u8_from_nibbles};
use super::{Address, Register};
use std::fmt;

#[derive(Debug)]
pub enum Instruction {
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
    pub fn decode(nibbles: [u8; 4]) -> Option<Self> {
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
            [9, x, y, 0] => SkipNeq(x, y),
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

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instruction::*;
        match self {
            ClearScreen => write!(f, "clear"),
            ReturnFromSubroutine => write!(f, "return"),
            JumpToMachineCode(address) => write!(f, "machine_jump {address:#x}"),
            JumpTo(address) => write!(f, "jump {address:#x}"),
            ExecuteSubroutine(address) => write!(f, "call {address:#x}"),
            SkipEqTo(x, value) => write!(f, "skip_if v{x:x} == {value}"),
            SkipNeqTo(x, value) => write!(f, "skip_if v{x:x} != {value}"),
            SkipEq(x, y) => write!(f, "skip_if v{x:x} == v{y:x}"),
            StoreValue(x, value) => write!(f, "v{x:x} := {value}"),
            AddValue(x, value) => write!(f, "v{x:x} += {value}"),
            StoreRegister(x, y) => write!(f, "v{x:x} := v{y:x}"),
            Or(x, y) => write!(f, "v{x:x} |= v{y:x}"),
            And(x, y) => write!(f, "v{x:x} &= v{y:x}"),
            Xor(x, y) => write!(f, "v{x:x} ^= v{y:x}"),
            AddRegister(x, y) => write!(f, "v{x:x} += v{y:x}"),
            SubRegister(x, y) => write!(f, "v{x:x} -= v{y:x}"),
            ShiftRight(x, y) => write!(f, "v{x:x} = v{y:x} >> 1"),
            SubRegisterReverse(x, y) => write!(f, "v{x:x} = v{y:x} - v{x:x}"),
            ShiftLeft(x, y) => write!(f, "v{x:x} = v{y:x} << 1"),
            SkipNeq(x, y) => write!(f, "skip_if v{x:x} != v{y:x}"),
            StoreAddr(address) => write!(f, "i := {address:#x}"),
            JumpToOffset(offset) => write!(f, "jump v0 + {offset:#x}"),
            StoreRandom(x, mask) => write!(f, "v{x:x} := random() & {mask:#b}"),
            DrawSprite(x, y, count) => write!(f, "draw v{x:x}, v{y:x}, {count}"),
            SkipIfKeyPressed(x) => write!(f, "skip_if_pressed v{x:x}"),
            SkipIfKeyNotPressed(x) => write!(f, "skip_if_not_pressed v{x:x}"),
            StoreDelayTimer(x) => write!(f, "v{x:x} := delay"),
            WaitForKeypress(x) => write!(f, "wait_keypress v{x:x}"),
            SetDelayTimer(x) => write!(f, "delay := v{x:x}"),
            SetSoundTimer(x) => write!(f, "sound := v{x:x}"),
            AddToI(x) => write!(f, "i += v{x:x}"),
            StoreDigitLocation(x) => write!(f, "i := digit_location v{x:x}"),
            StoreBinaryCoded(x) => write!(f, "binary_encode v{x:x}"),
            StoreRegisters(x) => write!(f, "store_registers v0 .. v{x:x}"),
            LoadRegisters(x) => write!(f, "load_registers v0 .. v{x:x}"),
        }
    }
}

pub fn dissassemble(bytes: &[u8]) -> Result<String, ()> {
    use std::fmt::Write;
    let mut result = String::new();
    for chunk in bytes.chunks_exact(2) {
        let a = chunk[0] >> 4;
        let b = chunk[0] & 0xf;
        let c = chunk[1] >> 4;
        let d = chunk[1] & 0xf;

        let Some(instruction) = Instruction::decode([a, b, c, d]) else {
            break;
        };
        writeln!(&mut result, "{instruction}").map_err(|_| ())?;
    }
    Ok(result)
}
