use ohim::dom::event::Event;

// cargo component build
wit_bindgen::generate!({
    path: "../wit/",
    world: "imports",
});

struct GuestComponent;

export!(GuestComponent);

impl Guest for GuestComponent {
    #[allow(async_fn_in_trait)]
    fn test() -> String {
        let x = Event::new("hello");
        x.get_type()
    }

    #[allow(async_fn_in_trait)]
    fn call_callback(name: String, args: Vec<String>) -> String {
        todo!()
    }
}
