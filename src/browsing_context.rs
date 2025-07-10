//! A browsing context is a programmatic representation of a series of documents, multiple of which can live within a single navigable.

use std::{collections::HashSet, time::Instant};

use url::Url;

use crate::{
    Document,
    url::{DOMUrl, ImmutableOrigin},
};

/// <https://html.spec.whatwg.org/multipage/document-sequences.html#browsing-context>
#[derive(Debug)]
pub struct BrowsingContext {}

impl BrowsingContext {
    /// <https://html.spec.whatwg.org/multipage/document-sequences.html#creating-a-new-top-level-browsing-context>
    pub fn create_top_browsing_context() -> (Self, Document) {
        // 1. Let group and document be the result of creating a new browsing context group and document.
        todo!()
    }

    /// <https://html.spec.whatwg.org/multipage/document-sequences.html#creating-a-new-browsing-context>
    /// TODO: implement embedder
    pub fn create_new_browsing_context(
        creator: Option<Document>,
        embedder: Option<bool>,
        group: &BrowsingContextGroup,
    ) -> (Self, Document) {
        // 1. Let browsingContext be a new browsing context.
        let context = BrowsingContext {};
        // 2. Let unsafeContextCreationTime be the unsafe shared current time.
        let time = Instant::now();
        // 3. Let creatorOrigin be null.
        let creator_origin: Option<ImmutableOrigin> = None;
        // 4. Let creatorBaseURL be null.
        let creator_url: Option<DOMUrl> = None;
        // 5. TODO: If creator is non-null, then:

        // 6. Let sandboxFlags be the result of determining the creation sandboxing flags given browsingContext and
        // embedder.
        let flags = context.determine_creation_sandbox_flags(&embedder);
        // 7. Let origin be the result of determining the origin given about:blank, sandboxFlags, and creatorOrigin.
        let origin = determin_origin(
            Some(&DOMUrl::parse("about:blank").unwrap()),
            flags,
            creator_origin,
        );
        // 8. TODO: Let permissionsPolicy be the result of creating a permissions policy given embedder and origin.
        let policy = false;
        todo!()
    }

    /// <https://html.spec.whatwg.org/multipage/browsers.html#determining-the-creation-sandboxing-flags>
    /// TODO: Implement sandbox flags
    pub fn determine_creation_sandbox_flags(&self, embedder: &Option<bool>) -> bool {
        false
    }
}

/// <https://html.spec.whatwg.org/multipage/document-sequences.html#browsing-context-group>
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct BrowsingContextGroup {}

impl BrowsingContextGroup {
    /// <https://html.spec.whatwg.org/multipage/document-sequences.html#creating-a-new-browsing-context-group-and-document>
    pub fn create_browsing_context_group_and_document() -> (Self, Document) {
        // 1. Let group be a new browsing context group.
        // 2. Append group to the user agent's browsing context group set.
        // 3. Let browsingContext and document be the result of creating a new browsing context and document with null,
        // null, and group.
        todo!()
    }
}

/// <https://html.spec.whatwg.org/multipage/document-sequences.html#determining-the-origin>
pub fn determin_origin(
    url: Option<&DOMUrl>,
    flags: bool,
    origin: Option<ImmutableOrigin>,
) -> ImmutableOrigin {
    // 1. If sandboxFlags has its sandboxed origin browsing context flag set, then return a new opaque origin.
    if flags {
        return ImmutableOrigin::new_opaque();
    }
    match (url, origin) {
        // 2. If url is null, then return a new opaque origin.
        (None, _) => ImmutableOrigin::new_opaque(),
        // 3. If url is about:srcdoc, then:
        // 4. If url matches about:blank and sourceOrigin is non-null, then return sourceOrigin.
        (Some(u), Some(o)) => {
            // TODO: Implement matches URL
            if u.as_str() == "about:srcdoc" || u.as_str() == "about:blank" {
                o
            } else {
                // 5. Return url's origin.
                u.origin()
            }
        }
        // 5. Return url's origin.
        (Some(u), None) => u.origin(),
    }
}
