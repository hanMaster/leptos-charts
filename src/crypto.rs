use crate::config::config;
use crate::error::Result;
use crate::time::now_timestamp;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::fmt::Write;

pub fn gen_signature(payload: &str) -> Result<String> {
    let api_key = &config().api_key;
    let secret_key = &config().secret_key;
    let recv_window = &config().recv_window;

    let mut hmac_sha256 = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())?;
    let ts = now_timestamp()?;
    let param_str = format!("{}{}{}{}", ts, api_key, recv_window, payload);
    hmac_sha256.update(param_str.as_bytes());
    let hmac_result = hmac_sha256.finalize().into_bytes();
    let res = hmac_result.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02x }");
        output
    });

    Ok(res)
}
