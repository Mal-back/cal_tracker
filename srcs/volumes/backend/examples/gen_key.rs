use anyhow::Result;
use rand::RngCore;

fn main() -> Result<()> {
    let mut key = [0u8; 64];

    rand::thread_rng().fill_bytes(&mut key);

    println!("\n Generated key for hmac: \n{key:?}");

    let encoded_key = base64_url::encode(&key);
    println!("\n Encoded key: \n{encoded_key}");
    Ok(())
}
