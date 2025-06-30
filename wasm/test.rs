use ohim::dom::event::Event;

// Use wit_bindgen to generate the bindings from the component model to Rust.
// For more information see: https://github.com/bytecodealliance/wit-bindgen/
wit_bindgen::generate!({
    path: "../wit/",
    world: "all",
});

struct GuestComponent;

export!(GuestComponent);

impl Guest for GuestComponent {
    #[allow(async_fn_in_trait)]
    fn test() -> String {
        let x = Event::new("hello");
        x.get_type()
    }
}
