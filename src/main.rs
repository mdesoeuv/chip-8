mod machine;

use std::path::PathBuf;
use clap::Parser;
use machine::{Machine, Screen};

#[derive(Parser)]
struct CLA {
    program: PathBuf,
}

struct App {
    machine: Machine,
}

#[derive(Debug)]
enum Message {}
const WINDOW_SIZE: iced::Size = iced::Size {
    width: Screen::WIDTH as f32 * Screen::SCALE,
    height: Screen::HEIGHT as f32 * Screen::SCALE,
};

impl App {
    fn update(&mut self, message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        iced::widget::canvas(&self.machine.screen)
            .width(iced::Length::Fixed(WINDOW_SIZE.width))
            .height(iced::Length::Fixed(WINDOW_SIZE.height))
            .into()
    }
}

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let args = CLA::parse();

    let bytecode = std::fs::read(&args.program)?;

    let mut machine = Machine::new();
    machine.load_program(&bytecode)?;

    iced::application("chip-8", App::update, App::view)
        .centered()
        .resizable(false)
        .window_size(WINDOW_SIZE)
        .run_with(|| (App { machine }, iced::Task::none()))?;

    Ok(())
}
