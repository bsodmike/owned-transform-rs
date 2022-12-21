// #![forbid(unused_imports)]

use core::convert::TryInto;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{Gray8, GrayColor},
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};

use embedded_hal::i2c::I2c;
use graphics::{Flushable, OwnedDrawTargetExt};
use serial::{HandlesI2C, OwnedTargetExt};
use std::fmt;
use std::{any, convert::Infallible, error};
use std::{fmt::Debug, marker::PhantomData};

pub mod graphics;
pub mod serial;

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

pub fn get_display<D>(
    display: D,
) -> Result<impl DrawTarget<Color = Gray8, Error = impl Debug + 'static> + 'static>
where
    D: DrawTarget<Color = Gray8> + 'static,
    <D as DrawTarget>::Error: Debug,
{
    Ok(display)
}

// I2C

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// I2C communication error
#[derive(Debug)]
pub struct I2cCommError {
    inner: BoxError,
}

impl I2cCommError {
    /// Create a new `Error` from a boxable error.
    pub fn new(error: impl Into<BoxError>) -> Self {
        Self {
            inner: error.into(),
        }
    }

    #[allow(dead_code)]
    /// Convert an `Error` back into the underlying boxed trait object.
    pub fn into_inner(self) -> BoxError {
        self.inner
    }
}

impl fmt::Display for I2cCommError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl std::error::Error for I2cCommError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.inner)
    }
}

impl embedded_hal::i2c::Error for I2cCommError {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        embedded_hal::i2c::ErrorKind::Other
    }
}

struct DummyI2c {}

impl DummyI2c {
    fn new() -> Self {
        Self {}
    }
}

struct ExampleDevice<I2C> {
    iface: I2C,
}

impl<I2C> I2c for ExampleDevice<I2C> {
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> std::result::Result<(), Self::Error> {
        todo!()
    }

    fn write(&mut self, address: u8, bytes: &[u8]) -> std::result::Result<(), Self::Error> {
        todo!()
    }

    fn write_iter<B>(&mut self, address: u8, bytes: B) -> std::result::Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>,
    {
        todo!()
    }

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> std::result::Result<(), Self::Error> {
        todo!()
    }

    fn write_iter_read<B>(
        &mut self,
        address: u8,
        bytes: B,
        buffer: &mut [u8],
    ) -> std::result::Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>,
    {
        todo!()
    }

    fn transaction<'a>(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'a>],
    ) -> std::result::Result<(), Self::Error> {
        todo!()
    }

    fn transaction_iter<'a, O>(
        &mut self,
        address: u8,
        operations: O,
    ) -> std::result::Result<(), Self::Error>
    where
        O: IntoIterator<Item = embedded_hal::i2c::Operation<'a>>,
    {
        todo!()
    }
}

impl<I2C> embedded_hal::i2c::ErrorType for ExampleDevice<I2C> {
    type Error = I2cCommError;
}

pub fn get_device<E, D>(device: D) -> Result<impl I2c<Error = E> + 'static>
where
    D: I2c<Error = E> + 'static,
{
    Ok(device)
}

fn main() -> Result<()> {
    env_logger::init();

    log::info!("Starting.");

    // HandlesI2C
    let i2c1 = DummyI2c::new();
    let device = ExampleDevice { iface: i2c1 };
    let device = get_device(device)?;

    let mut wrapped = device
        //
        .owned_handler(|target| {
            //
            log::info!("This is the closure");

            Ok(())
        });

    wrapped.handle().unwrap();

    // Graphics
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
