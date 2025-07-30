use ohim::dom::node::Document;

// cargo component build
wit_bindgen::generate!({
    path: "../wit",
    world: "ohim:dom/imports",
});

struct GuestComponent;

export!(GuestComponent);

impl Guest for GuestComponent {
    #[allow(async_fn_in_trait)]
    fn test() -> String {
        let document = Document::new();
        let element = document.document_element();
        format!(
            "Document has url: {} with element has attributes: {}",
            document.url(),
            element.unwrap().has_attributes()
        )
    }
}
