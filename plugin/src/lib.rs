wit_bindgen::generate!({
    path: "../wit/test.wit",
    world: "testw"
});

struct MyObject;

impl Testw for MyObject {
    fn testfun(mut input: String) -> String {
        input.push_str(" world");
        input
    }
}

export_testw!(MyObject);
