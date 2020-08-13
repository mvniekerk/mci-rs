pub trait Register<T> {
    fn value(&self) -> T;
    fn address() -> u8;
}

pub trait SdMmcRegister<T> {
    fn value(&self) -> T;
    fn address() -> u32;
}


