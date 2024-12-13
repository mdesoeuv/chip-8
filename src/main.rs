mod machine;

use clap::Parser;
use iced::keyboard::Key;
use machine::{instruction, Machine, Screen};
use std::path::PathBuf;

use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};

#[derive(Parser)]
struct CLA {
    program: PathBuf,
    #[arg(short, long)]
    debug: bool,
}

struct App {
    pub debugging: bool,
    machine: Machine,
    _stream: OutputStream,
    audio_sink: Sink,
    last_draw: Option<std::time::Instant>,
}

impl App {
    fn new(machine: Machine) -> Self {
        // _stream must live as long as the sink
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let audio_sink = Sink::try_new(&stream_handle).unwrap();

        // Add a dummy source of the sake of the example.
        let source = SineWave::new(440.0).amplify(0.20);
        audio_sink.append(source);
        audio_sink.pause();

        App {
            debugging: false,
            machine,
            _stream,
            audio_sink,
            last_draw: None,
        }
    }
}

#[derive(Debug)]
enum Message {
    Render(iced::time::Instant),
    KeyPadPressed(machine::Key),
    KeyPadReleased(machine::Key),
    DebuggerStep,
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
            Message::Render(last_draw) => {
                // Frame rate log
                if let Some(t) = self.last_draw {
                    let delay = last_draw - t;
                    log::debug!("frame_rate: {}", 1.0 / delay.as_secs_f32());
                }
                self.last_draw = Some(last_draw);

                // Update clocks
                self.machine.delay_timer = self.machine.delay_timer.saturating_sub(1);
                self.machine.sound_timer = self.machine.sound_timer.saturating_sub(1);

                // Manage Audio
                if self.machine.sound_timer > 0 {
                    self.audio_sink.play();
                } else {
                    self.audio_sink.pause();
                }

                // Run code
                if !self.debugging {
                    match self.machine.run() {
                        Ok(_) => {}
                        Err(error) => panic!("{error}"),
                    }
                }

                // Reset keypad
                self.machine.keypad.reset();
            }
            Message::KeyPadPressed(key) => self.machine.keypad.press(key),
            Message::KeyPadReleased(key) => self.machine.keypad.release(key),
            Message::DebuggerStep => {
                match self.machine.step() {
                    Ok(_) => {}
                    Err(error) => panic!("{error}"),
                }
            },
        }
    }

    fn view(&self) -> iced::Element<Message> {
        // iced::widget::canvas(&self.machine.screen)
        //     .width(iced::Length::Fixed(WINDOW_SIZE.width))
        //     .height(iced::Length::Fixed(WINDOW_SIZE.height))
        //     .into()
        iced::Element::new(&self.machine.screen)
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        // let frame_delay = iced::time::Duration::from_secs(1) / FRAME_RATE;
        // let frames = iced::time::every(frame_delay).map(Message::Render);
        let frames = iced::window::frames().map(Message::Render);

        let key_pressed = iced::keyboard::on_key_press(|key, _modifier| match key {
            Key::Character(c) => Some(Message::KeyPadPressed(keymap(c.as_str())?)),
            _ => None,
        });

        let key_release = iced::keyboard::on_key_release(|key, _modifier| match key {
            Key::Character(c) => Some(Message::KeyPadReleased(keymap(c.as_str())?)),
            _ => None,
        });

        let debugger_step = iced::keyboard::on_key_press(|key, _modifier| {
            match key {
                Key::Named(iced::keyboard::key::Named::Enter) => Some(Message::DebuggerStep),
                _ => None,
            }
        });

        iced::Subscription::batch([
            key_pressed,
            key_release,
            frames,
            debugger_step,
        ])
    }
}

fn main() -> Result<(), Box<dyn core::error::Error>> {
    env_logger::init();


    let args = CLA::parse();

    let bytecode = std::fs::read(&args.program)?;

    let mut machine = Machine::new();

    machine.load_program(&bytecode)?;

    let mut app = App::new(machine);

    app.debugging = args.debug;

    iced::application("chip-8", App::update, App::view)
        .centered()
        .window_size(WINDOW_SIZE)
        .subscription(App::subscription)
        .run_with(|| {
            (
                app,
                iced::Task::none(),
            )
        })?;

    Ok(())
}
