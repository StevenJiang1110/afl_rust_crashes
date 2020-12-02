extern crate url;

fn main() {
    let mut _local1 = url::Url::parse("r00:/.//00000000").unwrap();
    let _ = url::quirks::set_hostname(&mut _local1 ,"/000000000000000");
}
