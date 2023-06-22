use sqlx::Type;

const BASE62_CHARS: [u8; 62] =
    *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const ID_RETRY_COUNT: usize = 20; 

macro_rules! id_type {
    ($vis:vis, $struct:ty, $id_length:expr, $select_stmnt:literal) => {
        impl $struct {
            $vis async fn generate(
                transaction: &mut sqlx::Transaction<'_, crate::database::Database>,
            ) -> Result<$struct, super::DatabaseError> {
                let mut retry_count = 0;
                let length = $id_length;

                let censor = censor::Censor::Standard + censor::Censor::Sex;
                let mut id;

                loop {
                    id = crate::models::ids::generate_base62_id(length);

                    let results = sqlx::query!($select_stmnt, id)
                        .fetch_one(&mut *transaction)
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
    };
}

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

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct UserId(pub String);

id_type!(
    pub,
    UserId, 
    8,
    "SELECT COUNT(*) as count FROM users WHERE id = ?"
);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct ProjectMemberId(pub String);

id_type!(
    pub,
    ProjectMemberId, 
    10,
    "SELECT COUNT(*) as count FROM project_members WHERE id = ?"
);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct ProjectId(pub String);

id_type!(
    pub,
    ProjectId, 
    8,
    "SELECT COUNT(*) as count FROM projects WHERE id = ?"
);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct TaskGroupId(pub String);

id_type!(
    pub,
    TaskGroupId,
    8,
    "SELECT COUNT(*) as count FROM task_groups WHERE id = ?"
);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct TaskId(pub String);

id_type!(
    pub,
    TaskId,
    8,
    "SELECT COUNT(*) as count FROM tasks WHERE id = ?"
);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct SubTaskId(pub String);

id_type!(
    pub,
    SubTaskId,
    12,
    "SELECT COUNT(*) as count FROM sub_tasks WHERE id = ?"
);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct LoginHistoryId(pub String);

id_type!(
    pub,
    LoginHistoryId,
    10,
    "SELECT COUNT(*) as count FROM login_history WHERE id = ?"
);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct AuditId(pub String);

id_type!(
    pub,
    AuditId,
    12,
    "SELECT COUNT(*) as count FROM audit_log WHERE id = ?"
);



#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct NotifcationId(pub String);

id_type!(
    pub,
    NotifcationId,
    12,
    "SELECT COUNT(*) as count FROM notifications WHERE id = ?"
);




