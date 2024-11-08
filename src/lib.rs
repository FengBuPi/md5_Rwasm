use hex;
use md5 as as_md5;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn md5(data: &str) -> String {
    hex::encode((as_md5::compute(data)).0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = "hello world";
        let hash = md5(data);
        println!("MD5: {}", hash);
    }
}
