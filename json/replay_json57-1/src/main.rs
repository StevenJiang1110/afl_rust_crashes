extern crate json;

fn main() {
    let capacity = 673957684733028;
    let _ = json::object::Object::with_capacity(capacity);
}

