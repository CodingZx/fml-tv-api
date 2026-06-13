use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use smcrypto::{sm2, sm3};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::panic;

pub struct SMUtil;

impl SMUtil {
    pub fn sm2_generate_keypair() -> SM2 {
        let (sk, pk) = sm2::gen_keypair();
        SM2 { pk, sk }
    }

    pub fn sm2_from(pk: &str, sk: &str) -> SM2 {
        SM2 {
            pk: String::from(pk),
            sk: String::from(sk),
        }
    }

    pub fn sm3_hash(content: &str) -> String {
        sm3::sm3_hash(content.as_bytes())
    }

    pub fn sm3_hash_buffer(content: &[u8]) -> String {
        sm3::sm3_hash(content)
    }

}

pub struct SM2 {
    pk: String,
    sk: String,
}

impl SM2 {
    pub fn get_sk(&self) -> &str {
        &self.sk
    }

    pub fn get_pk(&self) -> &str {
        &self.pk
    }

    pub fn encrypt(&self, data: &str) -> String {
        let ctx = sm2::Encrypt::new(&self.pk);
        let encrypt_bytes = ctx.encrypt(data.as_bytes());
        STANDARD.encode(encrypt_bytes)
    }

    pub fn decrypt(&self, data: &str) -> Result<String, SMError> {
        if data.is_empty() {
            return Err(SMError::new("Data转换失败"));
        }
        let origin_bytes = STANDARD
            .decode(data)
            .map_err(|_| SMError::new("Data转换失败"))?;
        let ctx = sm2::Decrypt::new(&self.sk);

        // 可能Panic
        let decrypt_bytes = panic::catch_unwind(|| ctx.decrypt(&origin_bytes))
            .map_err(|_| SMError::new("解密字符串转换失败"))?;

        let origin_str =
            String::from_utf8(decrypt_bytes).map_err(|_| SMError::new("解密字符串转换失败"))?;
        Ok(origin_str)
    }
}

#[derive(Debug)]
pub struct SMError {
    message: String,
}

impl SMError {
    fn new(str: &'static str) -> Self {
        Self {
            message: str.to_string(),
        }
    }
}

impl Error for SMError {}

impl Display for SMError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "sm base: {}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let sm2 = SMUtil::sm2_generate_keypair();

        let pk = sm2.pk.clone();
        let sk = sm2.sk.clone();
        println!("pk->{:?}", pk);
        println!("sk->{:?}", sk);

        assert_ne!(sk, "".to_string());
        assert_ne!(pk, "".to_string());

        let model2 = SMUtil::sm2_from(pk.as_str(), sk.as_str());

        let d1 = sm2.encrypt("abc");
        assert_ne!(d1, "".to_string());
        let d2 = model2.decrypt(d1.as_str()).expect("decrypt base");
        assert_eq!("abc", d2);

        let hash = SMUtil::sm3_hash("abc");
        println!("sm3 hash: -> {}", hash);

        assert_eq!(
            "66c7f0f462eeedd9d1f2d46bdc10e4e24167c4875cf2f7a2297da02b8f4ba8e0",
            hash.as_str()
        );
    }
}
