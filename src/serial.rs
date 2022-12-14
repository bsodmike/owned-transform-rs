use anyhow;
use std::marker::PhantomData;

use super::I2cCommError;
use embedded_hal::i2c::{ErrorKind, ErrorType, I2c};
use embedded_hal_0_2::blocking::i2c::{AddressMode, SevenBitAddress, Write, WriteRead};

struct I2cWritable<I2C> {
    phantom: PhantomData<I2C>,
}

// impl<I2C, A> Write<A> for I2cWritable<I2C>
// where
//     A: AddressMode,
// {
//     type Error = I2cCommError;

//     fn write(&mut self, address: A, bytes: &[u8]) -> std::result::Result<(), Self::Error> {
//         todo!()
//     }
// }

//
// Owned
//

pub trait Transformer {
    // type AddressMode: AddressMode;
    type Error;

    type I2c<'a>: I2c<SevenBitAddress>
    where
        Self: 'a;

    fn transform<'a>(&'a mut self) -> Self::I2c<'a>;

    // fn source<'a>(&'a mut self) -> <Self as Transformer>::DrawTarget<'a>
    // where
    //     Self: Sized,
    // {
    //     self
    // }

    fn into_owned(self) -> Owned<Self>
    where
        Self: Sized,
    {
        Owned::new(self)
    }
}

pub struct I2cT<T, F>(T, F);

impl<T, F> Transformer for I2cT<T, F>
where
    T: I2c + 'static,
    F: FnMut(&mut T) -> Result<(), T::Error> + Send + Clone + 'static,
    //for<'a> I2cRunning<'a, T, F>: I2c,
{
    // type AddressMode = SevenBitAddress;
    type Error = T::Error;

    type I2c<'a> = I2cRunning<'a, T, F> where Self: 'a;

    fn transform<'a>(&'a mut self) -> Self::I2c<'a> {
        self.0.yank(self.1.clone())
    }
}

pub struct Owned<T>(T);

impl<T> Owned<T>
where
    T: Transformer,
{
    fn new(mut transformer: T) -> Self {
        Self(transformer)
    }
}

impl<T> I2c for Owned<T>
where
    T: Transformer,
{
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        todo!()
    }

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        todo!()
    }

    fn write_iter<B>(&mut self, address: u8, bytes: B) -> Result<(), Self::Error>
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
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn write_iter_read<B>(
        &mut self,
        address: u8,
        bytes: B,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>,
    {
        todo!()
    }

    fn transaction<'a>(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'a>],
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn transaction_iter<'a, O>(&mut self, address: u8, operations: O) -> Result<(), Self::Error>
    where
        O: IntoIterator<Item = embedded_hal::i2c::Operation<'a>>,
    {
        todo!()
    }
}

impl<T> UsesI2C for Owned<T>
where
    T: Transformer,
    for<'a> T::I2c<'a>: UsesI2C,
{
    type Error = embedded_hal::i2c::ErrorKind;

    fn run_flusher(&mut self) -> Result<(), <Self as UsesI2C>::Error> {
        log::info!("impl UsesI2C for Owned<T>");
        self.0.transform().run_flusher();

        Ok(())
    }
}

impl<T> ErrorType for Owned<T>
where
    T: Transformer,
{
    type Error = ErrorKind;
}

//
// UsesI2C
//

pub trait UsesI2C: I2c {
    type Error;

    fn run_flusher(&mut self) -> Result<(), <Self as UsesI2C>::Error>;
}

pub struct I2cRunning<'a, T, F> {
    parent: &'a mut T,
    flusher: F,
}

impl<'a, T, F> I2cRunning<'a, T, F> {
    pub fn new(parent: &'a mut T, flusher: F) -> Self {
        Self { parent, flusher }
    }
}

impl<'a, T> I2cRunning<'a, T, fn(&mut T) -> Result<(), T::Error>>
where
    T: I2c,
{
    pub fn noop(parent: &'a mut T) -> Self {
        Self::new(parent, |_| Ok(()))
    }
}

impl<'a, T, F> ErrorType for I2cRunning<'a, T, F> {
    type Error = ErrorKind;
}

impl<'a, T, F> UsesI2C for I2cRunning<'a, T, F>
where
    T: I2c,
    F: FnMut(&mut T) -> Result<(), T::Error>,
{
    type Error = <T as embedded_hal::i2c::ErrorType>::Error;

    fn run_flusher(&mut self) -> Result<(), <Self as UsesI2C>::Error> {
        let Self {
            parent: target,
            flusher,
        } = self;

        (flusher)(target)
    }
}

impl<'a, T, F> I2c for I2cRunning<'a, T, F>
where
    T: I2c,
{
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        todo!()
    }

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        todo!()
    }

    fn write_iter<B>(&mut self, address: u8, bytes: B) -> Result<(), Self::Error>
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
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn write_iter_read<B>(
        &mut self,
        address: u8,
        bytes: B,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>,
    {
        todo!()
    }

    fn transaction<'b>(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'b>],
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn transaction_iter<'b, O>(&mut self, address: u8, operations: O) -> Result<(), Self::Error>
    where
        O: IntoIterator<Item = embedded_hal::i2c::Operation<'b>>,
    {
        todo!()
    }
}

//
// TargetExt2
//

pub trait TargetExt2: I2c + Sized {
    fn yank<F: FnMut(&mut Self) -> Result<(), Self::Error>>(
        &mut self,
        flusher: F,
    ) -> I2cRunning<'_, Self, F>;
}

impl<T> TargetExt2 for T
where
    T: I2c,
{
    fn yank<F: FnMut(&mut Self) -> Result<(), Self::Error>>(
        &mut self,
        flusher: F,
    ) -> I2cRunning<'_, Self, F> {
        log::info!("inside yank");

        I2cRunning::new(self, flusher)
    }
}

pub trait OwnedTargetExt: I2c + Sized {
    fn owned_yank<F: FnMut(&mut Self) -> Result<(), Self::Error> + Send + Clone + 'static>(
        self,
        flusher: F,
    ) -> Owned<I2cT<Self, F>>
    where
        Self: 'static,
        Self::Error: 'static;
}

impl<T> OwnedTargetExt for T
where
    T: I2c,
{
    fn owned_yank<F: FnMut(&mut Self) -> Result<(), Self::Error> + Send + Clone + 'static>(
        self,
        flusher: F,
    ) -> Owned<I2cT<Self, F>>
    where
        Self: 'static,
        Self::Error: 'static,
    {
        I2cT(self, flusher).into_owned()
    }
}
