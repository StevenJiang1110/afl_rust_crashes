extern crate url;

fn main() {
    let mut _local0 = url::Url::parse("l:/.//").unwrap();
    let _ = url::quirks::set_host(&mut _local0 ,":00000");
}
