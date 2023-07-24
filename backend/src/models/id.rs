use serde_derive::{Serialize, Deserialize};

const ID_RETRY_COUNT: usize = 20; 

macro_rules! id {
    ($vis:vis, $struct:ty, $id_length:expr, $select_stmnt:literal) => {
        impl $struct {
            $vis async fn generate(
                executor: &mut sqlx::Transaction<'_, crate::database::Database>
            ) -> Result<$struct, super::DatabaseError>  {
                let mut retry_count = 0;
                let length = $id_length;

                let censor = censor::Censor::Standard + censor::Censor::Sex;
                let mut id;

                loop {
                    id = crate::models::id::generate_base62_id(length);

                    let results = sqlx::query!($select_stmnt, id)
                        .fetch_one(&mut **executor)
                        .await?;

                    let count = results.count;

                    if !censor.check(&id) && count == 0 {
                        break;
                    }
                    
                    retry_count += 1;
                    if retry_count > ID_RETRY_COUNT {
                        return Err(super::DatabaseError::RandomId);
                    }

                }
                
                Ok(Self(id))
            }
        }

        impl sqlx::Type<crate::database::Database> for $struct {
            fn type_info() -> crate::database::TypeInfo {
                <::std::primitive::str as ::sqlx::Type<sqlx::Sqlite>>::type_info()
            }

            fn compatible(ty: &crate::database::TypeInfo) -> ::std::primitive::bool {
                <&::std::primitive::str as ::sqlx::types::Type<sqlx::sqlite::Sqlite>>::compatible(ty)
            }
        }

        impl From<std::string::String> for $struct {
            fn from(value: std::string::String) -> Self {
                Self(value)
            }
        }
    };
}

const BASE62_CHARS: [u8; 62] =
    *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn generate_base62_id(length: usize) -> String {
    let mut id = String::with_capacity(length);
    let base: u64 = 62;

    let mut num = rand::random::<u64>();

    for _ in 0..length {
        id.push(BASE62_CHARS[(num % base) as usize] as char);
        num /= base;
    }

    id.chars().rev().collect()
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Encode, sqlx::Decode, sqlx::FromRow)]
pub struct UserId(pub String);

id!(
    pub,
    UserId,
    8,
    "SELECT COUNT(*) as count FROM users WHERE id = ?"
);


#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Encode, sqlx::Decode, sqlx::FromRow)]
pub struct LoginHistoryEntryId(pub String);

id!(
    pub,
    LoginHistoryEntryId,
    12,
    "SELECT COUNT(*) as count FROM login_history WHERE id = ?"
);

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Encode, sqlx::Decode, sqlx::FromRow)]
pub struct LoginSessionId(pub String);

id!(
    pub,
    LoginSessionId,
    10,
    "SELECT COUNT(*) as count FROM login_history WHERE id = ?"
);


#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Encode, sqlx::Decode, sqlx::FromRow)]
pub struct ProjectId(pub String);

id!(
    pub,
    ProjectId,
    8,
    "SELECT COUNT(*) as count FROM projects WHERE id = ?"
);


#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Encode, sqlx::Decode, sqlx::FromRow)]
pub struct ProjectMemberId(pub String);

id!(
    pub,
    ProjectMemberId,
    8,
    "SELECT COUNT(*) as count FROM project_members WHERE id = ?"
);


#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Encode, sqlx::Decode, sqlx::FromRow)]
pub struct TaskGroupId(pub String);

id!(
    pub,
    TaskGroupId,
    10,
    "SELECT COUNT(*) as count FROM task_groups WHERE id = ?"
);

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Encode, sqlx::Decode, sqlx::FromRow)]
pub struct TaskId(pub String);

id!(
    pub,
    TaskId,
    10,
    "SELECT COUNT(*) as count FROM tasks WHERE id = ?"
);


#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Encode, sqlx::Decode, sqlx::FromRow)]
pub struct SubTaskId(pub String);

id!(
    pub,
    SubTaskId,
    12,
    "SELECT COUNT(*) as count FROM task_groups WHERE id = ?"
);

