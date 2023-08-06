use chrono::Utc;
use jsonwebtoken::{TokenData, DecodingKey, Validation, Header, EncodingKey};

use super::{id::{UserId, LoginSessionId}, users::User};

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Debug, Serialize, Deserialize)]
pub struct Token(pub String);

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    // The time the token was issues at (s)
    pub iat: i64,
    // The time the token will expire (issue time + one week)
    pub exp: i64,
    // The user's id
    pub user_id: UserId,
    // The user's login session
    pub login_session: LoginSessionId,
}

impl Token {
    const COOKIE_NAME: &str = "AuthSession";

    pub fn decode(
        token: Token,
    ) -> jsonwebtoken::errors::Result<TokenData<TokenClaims>> {
        jsonwebtoken::decode::<TokenClaims>(
            &token.0,
            &DecodingKey::from_secret(&KEY),
            &Validation::default(),
        )
    }

    pub fn encode(user: &User, session: LoginSessionId) -> Self {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second

        let claims = TokenClaims {
            iat: now,
            exp: now + ONE_WEEK,
            user_id: user.id.clone(),
            login_session: session,
        };

        let token = jsonwebtoken::encode(
            &Header::default(), 
            &claims, 
            &EncodingKey::from_secret(&KEY)
        ).unwrap();

        Self(token)
    }
}