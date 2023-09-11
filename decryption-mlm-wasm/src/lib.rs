use des::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
use base64::{Engine as _, engine::general_purpose};
use md5::compute;
use regex::Regex;
use wasm_bindgen::prelude::*;

type Des64CbcDec = cbc::Decryptor<des::Des>;

fn decrypt_des(data: &str, key: &str) -> String{
    let base64 = general_purpose::STANDARD.decode(data.replace("@", "/")).unwrap();

    process_des(&base64, &key)
}

fn process_des(decrypt_string: &[u8], key: &str) -> String{
    let hash = compute(key.as_bytes()).0;
    let key = &hash[..8];
    let iv = &hash[8..16];

    let res = Des64CbcDec::new(key.into(), iv.into()).decrypt_padded_vec_mut::<Pkcs7>(decrypt_string).unwrap();
    String::from_utf8(res).unwrap()
}

fn decode(src: &str) -> String {
    if src.starts_with("mlm-") {
        let src = &src[4..];
        decrypt_des(src, "mlm")
    } else {
        src.to_string()
    }
}

#[wasm_bindgen]
pub fn decryption_mlm(data: String) -> String {
    Regex::new(r"\bMlmAction\b").unwrap().replace_all(&decode(&data), "Action").to_string()
}