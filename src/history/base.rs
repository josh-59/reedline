use std::collections::vec_deque::Iter;

use crate::core_editor::LineBuffer;

/// Browsing modes for a [`History`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HistoryNavigationQuery {
    /// `bash` style browsing through the history. Contained `LineBuffer` is used to store the state of manual entry before browsing through the history
    Normal(LineBuffer),
    /// Search for entries starting with a particular string.
    PrefixSearch(String),
    /// Full exact search for all entries containing a string.
    SubstringSearch(String),
    // Suffix Search
    // Fuzzy Search
}

/// Interface of a history datastructure that supports stateful navigation via [`HistoryNavigationQuery`].
pub trait History {
    /// Append entry to the history, if capacity management is part of the implementation may perform that as well
    fn append(&mut self, entry: String);

    /// Chronologic interation over all entries present in the history
    fn iter_chronologic(&self) -> Iter<'_, String>;

    /// This moves the cursor backwards respecting the navigation query that is set
    /// - Results in a no-op if the cursor is at the initial point
    fn back(&mut self);

    /// This moves the cursor forwards respecting the navigation-query that is set
    /// - Results in a no-op if the cursor is at the latest point
    fn forward(&mut self);

    /// Returns the string (if present) at the cursor
    fn string_at_cursor(&self) -> Option<String>;

    /// Set a new navigation mode for search based on input query defined in [`HistoryNavigationQuery`]
    ///
    /// By current convention, resets the position in the stateful browsing to the default.
    fn set_navigation(&mut self, navigation: HistoryNavigationQuery);

    /// Poll the current [`HistoryNavigationQuery`] mode
    fn get_navigation(&self) -> HistoryNavigationQuery;
}
