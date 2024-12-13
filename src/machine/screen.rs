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
    pub const SIZE: Size<f32> = Size::new(Self::WIDTH as f32, Self::HEIGHT as f32);

    pub fn clear(&mut self) {
        self.pixels = [[false; Self::WIDTH]; Self::HEIGHT]
    }

    // TODO: Should wrap around the screen
    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        log::trace!("draw_sprite: x: {x} y: {y}, sprite: {:x}", sprite.as_ptr() as usize);
        let mut colision_found = false;
        for (line, &sprite_line) in sprite.iter().enumerate() {
            let Some(screen_line) = self.pixels.get_mut(y + line) else {
                break;
            };
            for column in 0..(u8::BITS as usize) {
                let Some(image_pixel) = screen_line.get_mut(x + column) else {
                    break;
                };
                let sprite_pixel = ((sprite_line >> (u8::BITS as usize - column - 1)) & 1) != 0;
                colision_found |= *image_pixel & sprite_pixel;
                *image_pixel ^= sprite_pixel;
            }
        }
        colision_found
    }
}

use iced::{
    advanced::{graphics::core::event, layout, mouse, renderer, widget, Layout, Widget},
    window::RedrawRequest,
    Background, Color, Length, Rectangle, Shadow, Size,
};

impl<M, T, R> Widget<M, T, R> for &Screen
where
    R: iced::advanced::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &R,
        limits: &layout::Limits,
    ) -> layout::Node {
        let max_size = limits.max();

        let too_narrow =
            max_size.width * Screen::SIZE.height < Screen::SIZE.width * max_size.height;

        let ratio = match too_narrow {
            true => max_size.width / Screen::SIZE.width,
            false => max_size.height / Screen::SIZE.height,
        };

        // TODO: apply limits.min()
        layout::Node::new(Screen::SIZE * ratio)
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        renderer: &mut R,
        _theme: &T,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        log::trace!("Render!");
        let bounds = layout.bounds();
        let border = iced::Border::default();
        let shadow = Shadow::default();

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border,
                shadow,
            },
            Background::Color(Color::BLACK),
        );

        let scale = bounds.width / Screen::SIZE.width;

        for (y, line) in self.pixels.iter().enumerate() {
            for (x, &pixel) in line.iter().enumerate() {
                if pixel {
                    let bounds = Rectangle {
                        x: x as f32 * scale,
                        y: y as f32 * scale,
                        width: scale,
                        height: scale,
                    };
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds,
                            border,
                            shadow,
                        },
                        Background::Color(Color::WHITE),
                    );
                }
            }
        }
    }

    fn on_event(
        &mut self,
        _state: &mut widget::Tree,
        event: iced::Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &R,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, M>,
        _viewport: &Rectangle,
    ) -> event::Status {
        if let iced::Event::Window(e) = event {
            if let iced::window::Event::RedrawRequested(_) = e {
                shell.request_redraw(RedrawRequest::NextFrame);
                return event::Status::Captured;
            }
        }
        event::Status::Ignored
    }
}
