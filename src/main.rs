mod machine;

use clap::Parser;
use iced::keyboard::{key, Key};
use machine::{Machine, Screen};
use std::path::PathBuf;

use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};


#[derive(Parser)]
struct CLA {
    program: PathBuf,
}

struct App {
    machine: Machine,
    _stream: OutputStream,
    audio_sink: Sink,
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
                if self.machine.sound_timer > 0 {
                    self.audio_sink.play();
                } else {
                    self.audio_sink.pause();
                }
                match self.machine.run() {
                    Ok(_) => {}
                    Err(error) => panic!("{error}"),
                }
                self.machine.keypad.reset();
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

        let key_pressed = iced::keyboard::on_key_press(|key, _modifier| match key {
            Key::Character(c) => Some(Message::KeyPressed(keymap(c.as_str())?)),
            _ => None,
        });

        let key_release = iced::keyboard::on_key_release(|key, _modifier| match key {
            Key::Character(c) => Some(Message::KeyReleased(keymap(c.as_str())?)),
            _ => None,
        });

        iced::Subscription::batch([key_pressed, key_release, frames])
    }
}

const FRAME_RATE: u32 = 60;

fn main() -> Result<(), Box<dyn core::error::Error>> {
    env_logger::init();

    // _stream must live as long as the sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let audio_sink = Sink::try_new(&stream_handle).unwrap();

    // Add a dummy source of the sake of the example.
    let source = SineWave::new(440.0).amplify(0.20);
    audio_sink.append(source);

    let args = CLA::parse();

    let bytecode = std::fs::read(&args.program)?;

    let mut machine = Machine::new();
    machine.load_program(&bytecode)?;

    iced::application("chip-8", App::update, App::view)
        .centered()
        .resizable(false)
        .window_size(WINDOW_SIZE)
        .subscription(App::subscription)
        .run_with(|| (App { machine, _stream, audio_sink }, iced::Task::none()))?;

    Ok(())
}