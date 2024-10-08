pub struct Screen {
    pixels: [[bool; Self::WIDTH]; Self::HEIGHT],
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            pixels: [[false; Self::WIDTH]; Self::HEIGHT],
        }
    }
}

impl Screen {
    pub const SCALE: f32 = 20.0;
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;
}

use iced::{widget::canvas, Color};

impl canvas::Program<crate::Message> for &Screen {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry<iced::Renderer>> {
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        frame.fill(
            &canvas::Path::rectangle(iced::Point::ORIGIN, bounds.size()),
            Color::BLACK,
        );

        for (y, line) in self.pixels.iter().enumerate() {
            for (x, &pixel) in line.iter().enumerate() {
                if pixel {
                    frame.fill(
                        &canvas::Path::rectangle(
                            iced::Point {
                                x: x as f32 * Screen::SCALE,
                                y: y as f32 * Screen::SCALE,
                            },
                            iced::Size {
                                width: Screen::SCALE,
                                height: Screen::SCALE,
                            },
                        ),
                        Color::WHITE,
                    );
                }
            }
        }

        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}
