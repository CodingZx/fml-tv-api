use crate::common::error::ServerError::TokenError;
use crate::common::state::AppState;
use crate::common::ServerResult;
use crate::database::dao::sys_account::SysAccountDao;
use crate::database::model::sys_account::SysAccountModel;
use chrono::{Duration, Local, NaiveDateTime};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const JWT_SECRET: &[u8] = b"HR4QA:EoWt8v{x-a+ad)BplDw'ID0KQo";
const ACCESS_EXPIRE_SEC: i64 = 30 * 60;
const REFRESH_EXPIRE_SEC: i64 = 3 * 24 * 60 * 60; // 3天

const REFRESH_SUB: &str = "r";
const ACCESS_SUB: &str = "a";

/// 管理后台Token
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct AdminToken {
    account_id: Uuid,
    refresh: bool,
    time: NaiveDateTime,
}

impl AdminToken {
    pub fn access(account_id: Uuid) -> Self {
        Self {
            account_id,
            refresh: false,
            time: Local::now().naive_local(),
        }
    }

    pub fn refresh(account_id: Uuid) -> Self {
        Self {
            account_id,
            refresh: true,
            time: Local::now().naive_local(),
        }
    }

    pub fn parse_str(token: &str) -> Option<AdminToken> {
        if token.is_empty() {
            return None;
        }
        let rs = decode::<Claims>(&token, &DecodingKey::from_secret(JWT_SECRET), &Validation::default()).map(|data| data.claims);
        let claim = match rs {
            Ok(r) => r,
            Err(_) => {
                return None;
            }
        };

        let token = AdminToken {
            account_id: claim.account_id,
            refresh: claim.sub.eq(REFRESH_SUB),
            time: claim.time,
        };
        Some(token)
    }

    pub fn account_id(&self) -> Uuid {
        self.account_id
    }

    pub fn is_access(&self) -> bool {
        !self.refresh
    }

    pub fn to_jwt_str(&self) -> String {
        let sub = if self.refresh { REFRESH_SUB } else { ACCESS_SUB };
        let sec = if self.refresh { REFRESH_EXPIRE_SEC } else { ACCESS_EXPIRE_SEC };
        let expiration = (Local::now() + Duration::seconds(sec)).timestamp();
        let claims = Claims {
            sub: sub.to_string(),
            exp: expiration,
            account_id: self.account_id,
            time: self.time,
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET)).unwrap_or_default()
    }
    
    pub async fn require_account(&self, state: &AppState) -> ServerResult<SysAccountModel> {
        let account = SysAccountDao::new(&state.db)
            .find_by_id(self.account_id)
            .await?
            .filter(|f| f.status)
            .filter(|f| !f.deleted)
            .ok_or(TokenError)?;
        Ok(account)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
    account_id: Uuid,
    time: NaiveDateTime,
}


#[cfg(test)]
mod tests {
    use super::AdminToken;
    use super::Claims;
    use chrono::{Duration, Local};
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
    use std::str::FromStr;
    use uuid::Uuid;

    #[test]
    fn works() {
        let id = Uuid::from_str("00000000-0000-0000-0000-000000000000").expect("err");
        let token = AdminToken::access(id);

        let expiration = Local::now()
            .checked_add_signed(Duration::hours(24)) // 24小时有效期
            .expect("Invalid timestamp")
            .timestamp();
        let claims = Claims {
            sub: "u".to_string(),
            exp: expiration,
            account_id: token.account_id,
            time: token.time,
        };

        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap_or_default();
        println!("jwt -> {}", token);

        let secret = "secret";
        let decode_rs = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default(), ).map(|data| data.claims);
        match decode_rs {
            Ok(r) => {
                println!("{:?}", r);
            }
            Err(e) => {
                println!("err -> {:?}", e);
            }
        }
    }
}
