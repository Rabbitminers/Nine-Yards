use super::DatabaseError;
use crate::models::ids::base62_impl::to_base62;
use crate::models::ids::random_base62_rng;
use censor::Censor;
use serde::Deserialize;
use sqlx::sqlx_macros::Type;

const ID_RETRY_COUNT: usize = 20;

macro_rules! generate_ids {
    ($vis:vis $function_name:ident, $return_type:ty, $id_length:expr, $select_stmnt:literal, $id_function:expr) => {
        $vis async fn $function_name(
            conn: &sqlx::sqlite::SqlitePool,
        ) -> Result<$return_type, DatabaseError> {
            let mut rng = rand::thread_rng();
            let length = $id_length;
            let mut id = random_base62_rng(&mut rng, length);
            let mut retry_count = 0;
            let censor = Censor::Standard + Censor::Sex;

            // Check if ID is unique
            loop {
                let signed_id = id as i64;
                let query = sqlx::query!($select_stmnt, signed_id);
                let results = query.fetch_one(conn).await?;

                if results.count > 0 || censor.check(&*to_base62(id)) {
                    id = random_base62_rng(&mut rng, length);
                } else {
                    break;
                }

                retry_count += 1;
                if retry_count > ID_RETRY_COUNT {
                    return Err(DatabaseError::RandomId);
                }
            }

            Ok($id_function(id as i64))
        }
    };
}

generate_ids!(
    pub generate_user_id,
    UserId,
    8,
    "SELECT COUNT(*) as count FROM users WHERE id=$1",
    UserId
);


#[derive(Copy, Clone, Debug, PartialEq, Eq, Type, Deserialize)]
#[sqlx(transparent)]
pub struct UserId(pub i64);

use crate::models::ids::UserId as DataUserId;

impl From<DataUserId> for UserId {
    fn from(id: DataUserId) -> Self {
        UserId(id.0 as i64)
    }
}
