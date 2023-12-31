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
    SetTitleMode,
    SetBracketedPasteMode(bool),
    Bell,
    Back,
    ShowCursor,
    HideCursor,
    AutoWrap(bool),
    EraseInLine(u8),
    EraseInDisplay(u8),
    SetCharSet(CharSet),
}

#[derive(Debug, PartialEq)]
pub enum CharSet {
    ASCII,
}
