mod machine;

use iced::keyboard::{Key, key};
use clap::Parser;
use std::path::PathBuf;
use machine::{Machine, Screen};

#[derive(Parser)]
struct CLA {
    program: PathBuf,
}

struct App {
    machine: Machine,
}

#[derive(Debug)]
enum Message {
    Render(iced::time::Instant),
    KeyPressed(machine::Key),
    KeyReleased(machine::Key),
}

const WINDOW_SIZE: iced::Size = iced::Size {
    width: Screen::WIDTH as f32 * Screen::SCALE,
    height: Screen::HEIGHT as f32 * Screen::SCALE,
};

fn keymap(keyname: &str) -> Option<machine::Key> {
    Some(match keyname {
        "1" => 0x1,
        "2" => 0x2,
        "3" => 0x3,
        "4" => 0xc,
        "q" => 0x4,
        "w" => 0x5,
        "e" => 0x6,
        "r" => 0xd,
        "a" => 0x7,
        "s" => 0x8,
        "d" => 0x9,
        "f" => 0xe,
        "z" => 0xa,
        "x" => 0x0,
        "c" => 0xb,
        "v" => 0xf,
        _ => return None,
    })
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Render(_) => {
                self.machine.delay_timer = self.machine.delay_timer.saturating_sub(1);
                self.machine.sound_timer = self.machine.sound_timer.saturating_sub(1);
                match self.machine.run() {
                    Ok(_) => {},
                    Err(error) => panic!("{error}"),
                }
            }
            Message::KeyPressed(key) => self.machine.keypad.press(key),
            Message::KeyReleased(key) => self.machine.keypad.release(key),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        iced::widget::canvas(&self.machine.screen)
            .width(iced::Length::Fixed(WINDOW_SIZE.width))
            .height(iced::Length::Fixed(WINDOW_SIZE.height))
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        let frame_delay = iced::time::Duration::from_secs(1) / FRAME_RATE;
        let frames = iced::time::every(frame_delay).map(Message::Render);

        let key_pressed = iced::keyboard::on_key_press(|key, _modifier| {
            match key {
                Key::Character(c) => Some(Message::KeyPressed(keymap(c.as_str())?)),
                _ => None,
            }
        });

        let key_release = iced::keyboard::on_key_release(|key, _modifier| {
            match key {
                Key::Character(c) => Some(Message::KeyReleased(keymap(c.as_str())?)),
                _ => None,
            }
        });

        iced::Subscription::batch([
            key_pressed,
            key_release,
            frames
        ])
    }
}

const FRAME_RATE: u32 = 60;

fn main() -> Result<(), Box<dyn core::error::Error>> {

    
    let args = CLA::parse();
    
    let bytecode = std::fs::read(&args.program)?;
    
    let mut machine = Machine::new();
    machine.load_program(&bytecode)?;
    
    iced::application("chip-8", App::update, App::view)
        .centered()
        .resizable(false)
        .window_size(WINDOW_SIZE)
        .subscription(App::subscription)
        .run_with(|| (App { machine }, iced::Task::none()))?;

    Ok(())
}
