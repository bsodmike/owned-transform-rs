#![forbid(unused_imports)]

use crate::adaptors::OwnedDrawTargetExt;
use adaptors::Flushable;
use core::convert::TryInto;
use embedded_graphics::{
    pixelcolor::{Gray8, GrayColor},
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};
use std::fmt::Debug;
use std::{convert::Infallible, error};

pub mod adaptors;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// SPI communication error
#[derive(Debug)]
struct CommError;

/// A fake 64px x 64px display.
struct ExampleDisplay<SPI> {
    /// The framebuffer with one `u8` value per pixel.
    framebuffer: [u8; 64 * 64],

    /// The interface to the display controller.
    iface: SPI,
}

impl<SPI> ExampleDisplay<SPI>
where
    SPI: SpiWrite,
{
    /// Updates the display from the framebuffer.
    pub fn flush(&self) -> Result<()> {
        self.iface.send_bytes(&self.framebuffer);

        Ok(())
    }
}

impl<SPI> DrawTarget for ExampleDisplay<SPI> {
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

impl<SPI> OriginDimensions for ExampleDisplay<SPI> {
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
    fn send_bytes(&self, _buffer: &[u8]) {}
}

impl<SPI> Flushable for ExampleDisplay<SPI> {
    fn flush(&mut self) -> std::result::Result<(), Self::Error> {
        Ok(()) // do we really care about this?
    }
}

pub fn get_display<D>(
    display: D,
) -> Result<impl Flushable<Color = Gray8, Error = impl Debug + 'static> + 'static>
where
    D: adaptors::Flushable + embedded_graphics::draw_target::DrawTarget<Color = Gray8> + 'static,
    <D as embedded_graphics::draw_target::DrawTarget>::Error: Debug,
{
    Ok(display)
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

    // Calling `flush` here calls this directly on our instance of
    // `ExampleDisplay`.
    display.flush().unwrap();

    // However, due to the type-erasure, does this end up making
    // the same call to `flush` on the display instance?
    //
    // This now has type `impl Flushable<Color = Gray8, Error = impl Debug>`.
    let type_erased = get_display(display)?;
    type_erased.owned_noop_flushing();

    Ok(())
}
