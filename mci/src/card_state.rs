#[derive(PartialEq)]
pub enum CardState {
    Ready,
    Debounce,
    Init,
    Unusable,
    NoCard,
}
