//! A navigable presents a Document to the user via its active session history entry.

/// <https://html.spec.whatwg.org/multipage/document-sequences.html#navigable>
#[derive(Debug)]
pub struct Navigable {}

impl Navigable {
    /// <https://html.spec.whatwg.org/multipage/document-sequences.html#create-a-fresh-top-level-traversable>
    /// TODO: implement URL
    pub fn create_fresh_top_traversable() -> Self {
        Self {}
    }
}
