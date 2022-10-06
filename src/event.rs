// use crate::devices::{Device, DeviceType};

/// State of a Key or Button
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum State {
    Pressed,
    Released,
}

/// Key Identifier (UK Keyboard Layout)
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum KeyId {
    Escape,
    Return,
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Space,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Shift,
    LeftCtrl,
    RightCtrl,
    LeftAlt,
    RightAlt,
    CapsLock,
    Pause,
    PageUp,
    PageDown,
    PrintScreen,
    Insert,
    End,
    Home,
    Delete,
    Add,
    Subtract,
    Multiply,
    Separator,
    Decimal,
    Divide,
    BackTick,
    BackSlash,
    ForwardSlash,
    Plus,
    Minus,
    FullStop,
    Comma,
    Tab,
    Numlock,
    LeftSquareBracket,
    RightSquareBracket,
    SemiColon,
    Apostrophe,
    Hash,
}

/// Mouse Buttons
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
    RX,
    RY,
    RZ,
    SLIDER,
}

/// Event types
///
/// The usize entry acts as a device ID unique to each DeviceType (Mouse, Keyboard, Hid).
/// Keyboard press events repeat when a key is held down.
#[derive(Clone, Debug)]
pub enum RawEvent {
    KeyboardEvent(usize, KeyId, State),
}

