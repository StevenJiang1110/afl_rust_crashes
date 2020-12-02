extern crate url;

fn main() {
    let mut _local1 = url::Url::parse("p0:/.//0").unwrap();
    let _ = url::quirks::set_hostname(&mut _local1 ,"/0000000");
}
