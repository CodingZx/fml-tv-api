use crate::common::error::{BoxError, ServerError};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use rsa::pkcs8::{
    DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding,
};
use rsa::rand_core::OsRng;
use rsa::traits::PublicKeyParts;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use std::error::Error;
use std::fmt::{Display, Formatter};

pub struct RSAUtil;

impl RSAUtil {
    pub fn from_keypair(pub_key: &str, pri_key: &str) -> Result<RSAPair, RSAError> {
        let public_key = if pub_key.is_empty() {
            None
        } else {
            let pub_key_base64 = STANDARD
                .decode(pub_key.as_bytes())
                .map_err(|_| RSAError::KeyError("公钥转换失败"))?;
            Some(RsaPublicKey::from_public_key_der(&pub_key_base64).map_err(|_| RSAError::KeyError("公钥转换失败"))?)
        };
        let private_key = if pri_key.is_empty() {
            None
        } else {
            let pri_key_base64 = STANDARD
                .decode(pri_key.as_bytes())
                .map_err(|_| RSAError::KeyError("私钥转换失败"))?;
            Some(RsaPrivateKey::from_pkcs8_der(&pri_key_base64).map_err(|_| RSAError::KeyError("私钥转换失败"))?)
        };
        Ok(RSAPair { public_key, private_key })
    }

    /// 生成秘钥对
    pub fn generate_keypair(length: usize) -> Result<RSAPair, RSAError> {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, length).map_err(|_| RSAError::KeyError("私钥生成失败"))?;
        let public_key = private_key.to_public_key();

        Ok(RSAPair {
            public_key: Some(public_key),
            private_key: Some(private_key),
        })
    }
}

pub struct RSAPair {
    public_key: Option<RsaPublicKey>,
    private_key: Option<RsaPrivateKey>,
}

impl RSAPair {
    /// 获得私钥字符串
    pub fn get_private_key_str(&self) -> Result<String, RSAError> {
        let private_key = self.get_pri_key()?;

        let pem = private_key
            .to_pkcs8_pem(LineEnding::LF)
            .map_err(|_| RSAError::KeyError("私钥转换失败"))?
            .to_string();
        Ok(pem_to_base64(pem))
    }

    /// 获得公钥字符串
    pub fn get_public_key_str(&self) -> Result<String, RSAError> {
        let public_key = self.get_pub_key()?;
        let pem = public_key
            .to_public_key_pem(LineEnding::LF)
            .map_err(|_| RSAError::KeyError("公钥转换失败"))?;
        Ok(pem_to_base64(pem))
    }

    /// 加密
    pub fn encrypt(&self, data: &str) -> Result<String, RSAError> {
        let mut rng = OsRng;
        let public_key = self.get_pub_key()?;
        let key_size = public_key.size(); // 密钥长度（字节）
        // 根据填充模式计算最大分块长度 (PKCS1v1.5 需预留 11 字节)
        let max_chunk_size = key_size - 11;
        // 将输入文本转换为字节
        let input_bytes = data.as_bytes();
        let mut encrypted_data = Vec::new();
        // 分段加密处理
        for chunk in input_bytes.chunks(max_chunk_size) {
            let encrypted_chunk = public_key.encrypt(
                &mut rng,
                Pkcs1v15Encrypt,
                chunk
            ).map_err(RSAError::encrypt_error)?;

            encrypted_data.extend(encrypted_chunk);
        }

        // let encrypt_bytes = public_key
        //     .encrypt(&mut rng, Pkcs1v15Encrypt, data.as_bytes())
        //     .map_err(RSAError::encrypt_error)?;
        let base64_val = STANDARD.encode(&encrypted_data);

        Ok(base64_val)
    }

    /// 解密
    pub fn decrypt(&self, data: &str) -> Result<String, RSAError> {
        let origin_bytes = STANDARD.decode(data.as_bytes()).map_err(RSAError::decrypt_error)?;
        let private_key = self.get_pri_key()?;
        let key_size = private_key.size(); // 密钥长度（字节）

        let mut decrypted_data = Vec::new();

        // 按密钥长度分块解密
        for chunk in origin_bytes.chunks(key_size) {
            let decrypted_chunk = private_key.decrypt(
                Pkcs1v15Encrypt,
                chunk
            ).map_err(RSAError::decrypt_error)?;

            decrypted_data.extend(decrypted_chunk);
        }
        // 原始内容
        let str = String::from_utf8(decrypted_data).map_err(RSAError::decrypt_error)?;
        Ok(str)
    }

    fn get_pub_key(&self) -> Result<RsaPublicKey, RSAError> {
        let pub_key = self.public_key.clone().ok_or(RSAError::KeyError("公钥不存在"))?;
        Ok(pub_key)
    }

    fn get_pri_key(&self) -> Result<RsaPrivateKey, RSAError> {
        let pri_key = self.private_key.clone().ok_or(RSAError::KeyError("私钥不存在"))?;
        Ok(pri_key)
    }
}

fn pem_to_base64(pem: String) -> String {
    pem.lines()
        .filter(|line| !line.starts_with("-----"))
        .collect::<String>()
}

#[derive(Debug)]
pub enum RSAError {
    KeyError(&'static str),
    EncryptError(BoxError),
    DecryptError(BoxError),
}

impl Display for RSAError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RSAError::KeyError(e) => write!(f, "rsa key err: {e}"),
            RSAError::EncryptError(e) => write!(f, "rsa encrypt err: {e}"),
            RSAError::DecryptError(e) => write!(f, "rsa decrypt base: {e}"),
        }
    }
}

impl Error for RSAError {}

impl RSAError {
    fn encrypt_error<E>(error: E) -> RSAError
    where E: Error + Send + Sync + 'static,
    {
        RSAError::DecryptError(Box::new(error))
    }

    fn decrypt_error<E>(error: E) -> RSAError
    where E: Error + Send + Sync + 'static,
    {
        RSAError::DecryptError(Box::new(error))
    }
}

impl From<RSAError> for ServerError {
    fn from(err: RSAError) -> Self {
        ServerError::InnerError(Box::new(err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let rsa = RSAUtil::generate_keypair(2048).expect("generate_keypair base");

        let pub_key = rsa.get_public_key_str().expect("to base64 base");
        let pri_key = rsa.get_private_key_str().expect("to base64 base");
        println!("pub->{}", pub_key);
        println!("pri->{}", pri_key);

        assert_ne!(pub_key, "".to_string());
        assert_ne!(pri_key, "".to_string());

        let rsa2 = RSAUtil::from_keypair(pub_key.as_str(), pri_key.as_str()).expect("new base");

        let d1 = rsa.encrypt("abc").expect("encrypt base");

        println!("d1 -> {}", d1);
        assert_ne!(d1, "".to_string());
        let d2 = rsa2.decrypt(d1.as_str()).expect("decrypt base");
        assert_eq!("abc", d2);

        println!("d2 -> {}", d2);
    }
}
