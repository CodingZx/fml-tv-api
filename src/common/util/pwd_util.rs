use crate::common::consts::system::{ADMIN_PWD_MAX_LEN, ADMIN_PWD_MIN_LEN};
use crate::common::error::ServerError;
use crate::common::error::ServerError::BusinessStrError;
use crate::common::util::code_util::CodeUtil;
use crate::common::util::sm_util::{SMUtil, SM2};
use crate::common::ServerResult;
use serde::{Deserialize, Serialize};

const PWD_PK: &str = "fab66885e9f4f0870a20df0353788bb9d192f9eefd6c5bda3b9fa0e32b23577b6b571a37385288f33460b8a8a846ec6bbed0d772038a7f338a006f27999d1d61";
const PWD_SK: &str = "19aa6f7254fbe6501c43185c64c25134a64d67e7db1be29b5e5d4ae60937eeeb";

#[derive(Serialize, Deserialize, Debug)]
struct Password {
    origin: String,
    salt: String,
}

impl Password {
    fn new(origin: &str, salt: &str) -> Self {
        Self {
            origin: origin.to_string(),
            salt: salt.to_string(),
        }
    }
}

pub struct PasswordUtil;

impl PasswordUtil {
    /// 生成密码
    pub fn generate(pwd: &str) -> Result<String, ServerError> {
        let code = CodeUtil::generate_num_code(6);
        let sm2 = Self::get_sm2();

        let pwd_obj = Password::new(pwd, &code);
        let pwd_json = serde_json::to_string(&pwd_obj)?;

        Ok(sm2.encrypt(&pwd_json))
    }

    /// 校验密码是否正确
    pub fn verify(pwd: &str, hash: &str) -> bool {
        let sm2 = Self::get_sm2();
        let decrypt = match sm2.decrypt(hash) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let pwd_obj = match serde_json::from_str::<Password>(&decrypt) {
            Ok(v) => v,
            Err(_) => return false,
        };

        pwd_obj.origin == pwd
    }

    pub fn check_admin_len(pwd: &str) -> ServerResult<()> {
        let len = pwd.chars().count();
        if len < ADMIN_PWD_MIN_LEN {
            return Err(BusinessStrError(format!("密码长度不能小于{}", ADMIN_PWD_MIN_LEN)))
        }
        if len > ADMIN_PWD_MAX_LEN {
            return Err(BusinessStrError(format!("密码长度不能大于{}", ADMIN_PWD_MAX_LEN)))
        }
        Ok(())
    }

    fn get_sm2() -> SM2 {
        SMUtil::sm2_from(PWD_PK, PWD_SK)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let pwd = "router".to_string();

        let hash = PasswordUtil::generate(pwd.as_str()).unwrap();
        println!("pwd -> {}", hash);

        let verify = PasswordUtil::verify(pwd.as_str(), hash.as_str());

        assert!(verify);
    }
}
