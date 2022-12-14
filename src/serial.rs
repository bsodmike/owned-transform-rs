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

pub trait Transformer
where
    <Self as Transformer>::AddressMode: embedded_hal::i2c::AddressMode,
{
    type AddressMode: AddressMode;
    type Error;

    type I2c: I2c<Self::AddressMode>;

    fn transform<'a, A>(&'a mut self) -> Self::I2c;

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

pub struct I2cT<'a, T, F, A>(T, F, PhantomData<A>, PhantomData<&'a A>);

impl<'a, A, T, F> Transformer for I2cT<'a, T, F, A>
where
    T: I2c + 'a,
    F: FnMut(&mut T) -> Result<(), T::Error> + Send + Clone + 'static,
    A: AddressMode,
    I2cRunning<'a, T, F>: I2c,
{
    type AddressMode = SevenBitAddress;
    type Error = T::Error;

    type I2c = I2cRunning<'a, T, F> where Self: 'a;

    fn transform(&'a mut self) -> Self::I2c {
        self.0.I2cRunning(self.1.clone())
    }

    fn into_owned(self) -> Owned<Self>
    where
        Self: Sized,
    {
        Owned::new(self)
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
{
    type AddressMode = SevenBitAddress;

    fn foo(&mut self) -> Result<(), Self::Error> {
        self.0.transform().foo()
    }
}

impl<T> ErrorType for Owned<T>
where
    T: Transformer,
{
    type Error;
}

//
// UsesI2C
//

pub trait UsesI2C: I2c {
    type AddressMode;

    fn foo(&mut self) -> Result<(), Self::Error>;
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

impl<'a, T, F> ErrorType for I2cRunning<'a, T, F>
where
    T: Transformer,
{
    type Error = Box<dyn embedded_hal::i2c::Error>;
}

impl<'a, T, F> UsesI2C for I2cRunning<'a, T, F>
where
    T: Transformer + I2c,
    F: FnMut(&mut T) -> Result<(), <T as Transformer>::Error>,
{
    type AddressMode = SevenBitAddress;

    fn foo(&mut self) -> Result<(), Self::Error> {
        let Self {
            parent: target,
            flusher,
        } = self;

        (flusher)(target)
    }
}

impl<'a, T, F> I2c for I2cRunning<'a, T, F>
where
    T: Transformer + I2c,
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

    fn transaction(
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
