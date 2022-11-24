use core::convert::TryInto;
use embedded_graphics::{
    pixelcolor::{Gray8, GrayColor},
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};
use std::{convert::Infallible, error};

pub mod adaptors;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// SPI communication error
#[derive(Debug)]
struct CommError;

/// A fake 64px x 64px display.
struct ExampleDisplay<SPI1> {
    /// The framebuffer with one `u8` value per pixel.
    framebuffer: [u8; 64 * 64],

    /// The interface to the display controller.
    iface: SPI1,
}

impl<SPI1> ExampleDisplay<SPI1>
where
    SPI1: SpiWrite,
{
    /// Updates the display from the framebuffer.
    pub fn flush(&self) -> Result<()> {
        self.iface.send_bytes(&self.framebuffer);

        Ok(())
    }
}

impl<SPI1> DrawTarget for ExampleDisplay<SPI1> {
    type Color = Gray8;
    // `ExampleDisplay` uses a framebuffer and doesn't need to communicate with the display
    // controller to draw pixel, which means that drawing operations can never fail. To reflect
    // this the type `Infallible` was chosen as the `Error` type.
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> std::result::Result<(), Infallible>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // Check if the pixel coordinates are out of bounds (negative or greater than
            // (63,63)). `DrawTarget` implementation are required to discard any out of bounds
            // pixels without returning an error or causing a panic.
            if let Ok((x @ 0..=63, y @ 0..=63)) = coord.try_into() {
                // Calculate the index in the framebuffer.
                let index: u32 = x + y * 64;
                self.framebuffer[index as usize] = color.luma();
            }
        }

        Ok(())
    }
}

impl<SPI1> OriginDimensions for ExampleDisplay<SPI1> {
    fn size(&self) -> Size {
        Size::new(64, 64)
    }
}

struct DummySpi {}

trait SpiWrite {
    fn send_bytes(&self, buffer: &[u8]);
}

impl DummySpi {
    fn new() -> Self {
        Self {}
    }
}

impl SpiWrite for DummySpi {
    fn send_bytes(&self, buffer: &[u8]) {}
}

fn main() -> Result<()> {
    let spi1 = DummySpi::new();
    let mut display = ExampleDisplay {
        framebuffer: [0; 4096],
        iface: spi1,
    };

    // Draw a circle with top-left at `(22, 22)` with a diameter of `20` and a white stroke
    let circle = Circle::new(Point::new(22, 22), 20)
        .into_styled(PrimitiveStyle::with_stroke(Gray8::WHITE, 1));

    circle.draw(&mut display)?;

    // Update the display
    display.flush().unwrap();

    Ok(())
}
