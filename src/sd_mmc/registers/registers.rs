pub trait Register<T> {
    fn value(&self) -> T;
    fn address() -> u8;
}