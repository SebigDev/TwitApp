use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn get_jwt_key() -> Hmac<Sha256> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not provided");
    let key: Hmac<Sha256> = HmacSha256::new_from_slice(jwt_secret.as_bytes()).unwrap();
    key
}

