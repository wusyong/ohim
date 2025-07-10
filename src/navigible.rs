//! A navigable presents a Document to the user via its active session history entry.

use crate::{browsing_context::BrowsingContext, url::DOMUrl};

/// <https://html.spec.whatwg.org/multipage/document-sequences.html#navigable>
#[derive(Debug)]
pub struct Navigable {}

impl Navigable {
    /// <https://html.spec.whatwg.org/multipage/document-sequences.html#create-a-fresh-top-level-traversable>
    /// TODO: implement POST resource
    pub fn create_fresh_top_traversable(url: DOMUrl, resource: Option<bool>) -> Self {
        // 1. Let traversable be the result of creating a new top-level traversable given null and the empty string.
        Self {}
    }

    /// <https://html.spec.whatwg.org/multipage/document-sequences.html#creating-a-new-top-level-traversable>
    /// TODO: implement BrowsingContext
    pub fn create_top_traversable(
        opener: Option<bool>,
        target: String,
        navigable: Option<Navigable>,
    ) -> Self {
        // 1. Let document be null.
        let document = match opener {
            // 2. If opener is null, then set document to the second return value of creating a new top-level browsing
            // context and document.
            None => BrowsingContext::create_top_browsing_context().1,
            // 3. Otherwise, set document to the second return value of creating a new auxiliary browsing context and
            // document given opener.
            Some(_) => {
                todo!()
            }
        };
        Self {}
    }
}
