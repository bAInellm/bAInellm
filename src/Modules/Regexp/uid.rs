extern crate md5;
use std::env;
fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len()!= 2{
        println!("Usage: {} <string>", &args[0]);
        return;
    }

    let s = args[1].clone(); // clone so we don't modify original argument
    let mut hasher = md5::Md5::new();
    hasher.update(&s);
    let result = hasher.finalize();
    let encoded_result = base64::encode_config(&result[..], base64::URL_SAFE);
    println!("{}",encoded_result);

}