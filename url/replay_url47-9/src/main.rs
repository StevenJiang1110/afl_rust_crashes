extern crate url;

fn main() {
    let _local1 = url::Url::parse("t:/.//").unwrap();
    let _ = url::quirks::username(&_local1);
}
