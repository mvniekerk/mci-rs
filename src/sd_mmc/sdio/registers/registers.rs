pub trait Register<T> {
    fn value(&self) -> T;
    fn address() -> u8;
}

pub trait RegisterU8 = Register<u8>;