use serde::{Deserialize, Serialize};

/// Valid ways how `Reedline::read_line()` can return
#[derive(Debug)]
pub enum Signal {
    /// Entry succeeded with the provided content
    Success(String),
    /// Entry was aborted with `Ctrl+C`
    CtrlC, // Interrupt current editing
    /// Abort with `Ctrl+D` signalling `EOF` or abort of a whole interactive session
    CtrlD, // End terminal session
    /// Signal to clear the current screen. Buffer content remains untouched.
    CtrlL, // FormFeed/Clear current screen
}

/// Editing actions which can be mapped to key bindings.
///
/// Executed by `Reedline::run_edit_commands()`
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum EditCommand {
    /// Move to the start of the buffer
    MoveToStart,

    /// Move to the end of the buffer
    MoveToEnd,

    /// Move one character to the left
    MoveLeft,

    /// Move one character to the right
    MoveRight,

    /// Move one word to the left
    MoveWordLeft,

    /// Move one word to the right
    MoveWordRight,

    /// Insert a character at the current insertion point
    InsertChar(char),

    /// Backspace delete from the current insertion point
    Backspace,

    /// Delete in-place from the current insertion point
    Delete,

    /// Backspace delete a word from the current insertion point
    BackspaceWord,

    /// Delete in-place a word from the current insertion point
    DeleteWord,

    /// Clear the current buffer
    Clear,

    /// Cut from the start of the buffer to the insertion point
    CutFromStart,

    /// Cut from the insertion point to the end of the buffer
    CutToEnd,

    /// Cut the word left of the insertion point
    CutWordLeft,

    /// Cut the word right of the insertion point
    CutWordRight,

    /// Paste the cut buffer at the insertion point
    PasteCutBuffer,

    /// Upper case the current word
    UppercaseWord,

    /// Lower case the current word
    LowercaseWord,

    /// Capitalize the current character
    CapitalizeChar,

    /// Swap the current word with the word to the right
    SwapWords,

    /// Swap the current grapheme/character with the one to the right
    SwapGraphemes,

    /// Undo the previous edit command
    Undo,

    /// Redo an edit command from the undo history
    Redo,
}

/// Reedline supported actions.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum ReedlineEvent {
    /// No op event
    None,

    /// Trigger Tab
    HandleTab,

    /// Handle EndOfLine event
    ///
    /// Expected Behavior:
    ///
    /// - On empty line breaks execution to exit with [`Signal::CtrlD`]
    /// - Secondary behavior [`EditCommand::Delete`]
    CtrlD,

    /// Handle SIGTERM key input
    ///
    /// Expected behavior:
    ///
    /// Abort entry
    /// Run [`EditCommand::Clear`]
    /// Clear the current undo
    /// Bubble up [`Signal::CtrlC`]
    CtrlC,

    /// Clears the screen and sets prompt to first line
    ClearScreen,

    /// Handle enter event
    Enter,

    /// Mouse
    Mouse, // Fill in details later

    /// trigger termimal resize
    Resize(u16, u16),

    /// Run these commands in the editor
    Edit(Vec<EditCommand>),

    /// Trigger full repaint
    Repaint,

    /// Navigate to the previous historic buffer
    PreviousHistory,

    /// Move up to the previous line, if multiline, or up into the historic buffers
    Up,

    /// Move down to the next line, if multiline, or down through the historic buffers
    Down,

    /// Navigate to the next historic buffer
    NextHistory,

    /// Search the history for a string
    SearchHistory,
}
