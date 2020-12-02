extern crate url;

fn main() {
    let mut _local1 = url::Url::parse("arhttpsps:/.//eom/dae.com/\\\\t\\:").unwrap();
    let _ = url::quirks::set_hostname(&mut _local1 ,"//eom/datcom/\\\\t\\://eom/data.cs");
}
