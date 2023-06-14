use uuid::Uuid;

const ID_RETRY_COUNT: usize = 20; 

macro_rules! id_type {
    ($vis:vis $function_name:ident, $struct:ty, $id_length:expr, $select_stmnt:literal, $id_function:expr) => {
        $vis async fn $function_name (
            conn: &sqlx::SqlitePool,
        ) -> Result<$struct, super::DatabaseError> {
            let mut retry_count = 0;
            let length = $id_length;

            let censor = censor::Censor::Standard + censor::Censor::Sex;
            let mut id;

            loop {
                id = Uuid::new_v4().to_simple().to_string();

                let results = sqlx::query!($select_stmnt, id)
                    .fetch_one(conn)
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
            
            Ok($id_function(id))
        }
    };
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserId(pub String);

id_type!(
    pub generate_user_id,
    UserId, 
    10,
    "SELECT COUNT(*) as count FROM users WHERE id = ?",
    UserId
);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectMemberId(pub String);

id_type!(
    pub generate_project_member_id,
    ProjectMemberId, 
    10,
    "SELECT COUNT(*) as count FROM project_members WHERE id = ?",
    ProjectMemberId
);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectId(pub String);

id_type!(
    pub generate_project_id,
    ProjectId, 
    10,
    "SELECT COUNT(*) as count FROM projects WHERE id = ?",
    ProjectId
);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskGroupId(pub String);

id_type!(
    pub generate_task_group_id,
    TaskGroupId,
    8,
    "SELECT COUNT(*) as count FROM task_groups WHERE id = ?",
    TaskGroupId
);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskId(pub String);

id_type!(
    pub generate_tasks_id,
    TaskId,
    8,
    "SELECT COUNT(*) as count FROM tasks WHERE id = ?",
    TaskId
);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginHistoryId(pub String);

id_type!(
    pub generate_login_history_id,
    LoginHistoryId,
    8,
    "SELECT COUNT(*) as count FROM login_history WHERE id = ?",
    LoginHistoryId
);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotifcationId(pub String);

id_type!(
    pub generate_notification_id,
    NotifcationId,
    8,
    "SELECT COUNT(*) as count FROM notifications WHERE id = ?",
    NotifcationId
);


