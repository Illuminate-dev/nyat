#[derive(Debug, PartialEq)]
pub enum AnsiSequence {
    Character(char),
    Escape,
    CursorPos(u16, u16),
    CursorUp(u16),
    CursorDown(u16),
    CursorForward(u16),
    CursorBackward(u16),
    SetGraphicsMode(Vec<u8>),
}
