use embedded_hal::i2c::{AddressMode as EHalI2cAddressMode, ErrorKind, ErrorType, I2c};
use embedded_hal_0_2::blocking::i2c::{AddressMode, SevenBitAddress};

/*
    Flushing<'a, T, F>:     Handler<'a, T, F>
    FlushingT<T, F>:        HandlerT<T, F>
    Flushable               HandlesI2C
*/

pub trait Transformer {
    type AddressMode: AddressMode;
    type Error;

    type I2c<'a>: I2c<Self::AddressMode>
    where
        Self: 'a,
        <Self as Transformer>::AddressMode: EHalI2cAddressMode;

    fn transform<'a>(&'a mut self) -> Self::I2c<'a>
    where
        <Self as Transformer>::AddressMode: EHalI2cAddressMode;

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

pub struct HandlerT<T, F>(T, F);

impl<T, F> Transformer for HandlerT<T, F>
where
    T: I2c + 'static,
    F: FnMut(&mut T) -> Result<(), T::Error> + Send + Clone + 'static,
{
    type AddressMode = SevenBitAddress;
    type Error = T::Error;

    type I2c<'a> = Handler<'a, T, F> where Self: 'a;

    fn transform<'a>(&'a mut self) -> Self::I2c<'a> {
        self.0.handler(self.1.clone())
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

impl<T> HandlesI2C for Owned<T>
where
    T: Transformer,
    // NOTE this ensures the transformer is able to call the correct method.
    for<'a> T::I2c<'a>: HandlesI2C,
    <T as Transformer>::AddressMode: EHalI2cAddressMode,
{
    type Error = embedded_hal::i2c::ErrorKind;

    fn handle(&mut self) -> Result<(), <Self as HandlesI2C>::Error> {
        log::info!("impl HandlesI2C for Owned<T>");
        self.0.transform().handle();

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
// HandlesI2C
//

pub trait HandlesI2C: I2c {
    type Error;

    fn handle(&mut self) -> Result<(), <Self as HandlesI2C>::Error>;
}

pub struct Handler<'a, T, F> {
    parent: &'a mut T,
    handler: F,
}

impl<'a, T, F> Handler<'a, T, F> {
    pub fn new(parent: &'a mut T, handler: F) -> Self {
        Self { parent, handler }
    }
}

impl<'a, T> Handler<'a, T, fn(&mut T) -> Result<(), T::Error>>
where
    T: I2c,
{
    pub fn noop(parent: &'a mut T) -> Self {
        Self::new(parent, |_| Ok(()))
    }
}

impl<'a, T, F> ErrorType for Handler<'a, T, F> {
    type Error = ErrorKind;
}

impl<'a, T, F> HandlesI2C for Handler<'a, T, F>
where
    T: I2c,
    F: FnMut(&mut T) -> Result<(), T::Error>,
{
    type Error = <T as embedded_hal::i2c::ErrorType>::Error;

    fn handle(&mut self) -> Result<(), <Self as HandlesI2C>::Error> {
        let Self {
            parent: target,
            handler,
        } = self;

        (handler)(target)
    }
}

impl<'a, T, F> I2c for Handler<'a, T, F>
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
    fn handler<F: FnMut(&mut Self) -> Result<(), Self::Error>>(
        &mut self,
        handler: F,
    ) -> Handler<'_, Self, F>;
}

impl<T> TargetExt2 for T
where
    T: I2c,
{
    fn handler<F: FnMut(&mut Self) -> Result<(), Self::Error>>(
        &mut self,
        handler: F,
    ) -> Handler<'_, Self, F> {
        log::info!("inside handler");

        Handler::new(self, handler)
    }
}

pub trait OwnedTargetExt: I2c + Sized {
    fn owned_handler<F: FnMut(&mut Self) -> Result<(), Self::Error> + Send + Clone + 'static>(
        self,
        handler: F,
    ) -> Owned<HandlerT<Self, F>>
    where
        Self: 'static,
        Self::Error: 'static;
}

impl<T> OwnedTargetExt for T
where
    T: I2c,
{
    fn owned_handler<F: FnMut(&mut Self) -> Result<(), Self::Error> + Send + Clone + 'static>(
        self,
        handler: F,
    ) -> Owned<HandlerT<Self, F>>
    where
        Self: 'static,
        Self::Error: 'static,
    {
        HandlerT(self, handler).into_owned()
    }
}
