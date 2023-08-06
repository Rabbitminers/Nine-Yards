const ID_RETRY_COUNT: usize = 20; 

const BASE62_CHARS: [u8; 62] =
    *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Check if the given ID is already used in the specified database table.
///
/// # Arguments
///
/// - `table_name`: The name of the database table to check for identifier uniqueness.
/// - `id`: The ID to check for uniqueness.
/// - `transaction`: The database transaction to use for the query.
///
/// # Returns
///
/// `true` if the ID is already used in the table, `false` otherwise.
async fn is_id_used(
    table_name: &str,
    id: &str, 
    transaction: &mut sqlx::Transaction<'_, crate::database::Database>
) -> Result<bool, sqlx::error::Error> {
    let result = sqlx::query!(
        "
        SELECT COUNT(*) 
        AS count 
        FROM users
        WHERE $1 = $2",
        table_name, id
    )
    .fetch_one(&mut **transaction)
    .await?;

    Ok(result.count > 0 )
}

/// Generate a new base62-encoded identifier of the specified length.
///
/// # Arguments
///
/// - `length`: The length of the identifier to be generated.
///
/// # Returns
///
/// A new base62-encoded identifier of the specified length.
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

/// Macro to define a new id struct that can seamlessly be used in SQL queries and be used in Poem
/// Json objects
/// 
/// # Arguments
/// 
/// - `$vis`: Visibility specifier for the generated identifier function.
/// - `$struct`: The name of the struct for which the identifier is generated.
/// - `$id_length`: The length of the identifier to be generated.
/// - `$table_name`: The name of the database table to check for identifier uniqueness.
/// 
/// # Example
/// 
/// ```
/// // Declare a new identifier type `UserId` with a length of 8 characters and a table name "users".
/// id!(pub, UserId, 8, "users");
/// ```
/// 
macro_rules! id {
    ($vis:vis, $struct:ident, $id_length:expr, $table_name:literal) => {
        #[derive(Clone, serde::Serialize, serde::Deserialize, sqlx::Encode, sqlx::Decode, sqlx::FromRow)]
        $vis struct $struct(pub String);

        id_generator!($vis, $struct, $id_length, $table_name);

        id_conversions!($struct);
    };
}

/// Macro to generate a new identifier for the given struct using a transaction.
///
/// # Arguments
///
/// - `$vis`: Visibility specifier for the generated identifier function.
/// - `$struct`: The name of the struct for which the identifier is generated.
/// - `$id_length`: The length of the identifier to be generated.
/// - `$table_name`: The name of the database table to check for identifier uniqueness.
///
/// # Example
/// ```
/// // Generate a new `UserId` identifier using the provided database transaction.
/// let new_user_id = UserId::generate(&mut transaction).await?;
/// ```
macro_rules! id_generator {
    ($vis:vis, $struct:ident, $id_length:expr, $table_name:literal) => {
        impl $struct {
            $vis async fn generate(
                executor: &mut sqlx::Transaction<'_, crate::database::Database>
            ) -> Result<$struct, sqlx::error::Error>  {
                let mut retry_count = 0;
                let length = $id_length;

                let censor = censor::Censor::Standard + censor::Censor::Sex;
                let mut id;

                loop {
                    id = crate::models::id::generate_base62_id(length);

                    let used = is_id_used($table_name, &id, executor).await?;

                    if !censor.check(&id) && !used {
                        break;
                    }
                    
                    retry_count += 1;
                    if retry_count > ID_RETRY_COUNT {
                        return Err(sqlx::error::Error::WorkerCrashed);
                    }
                }
                
                Ok(Self(id))
            }
        }
    };
}

/// Macro to provide conversions for the specified identifier type.
///
/// # Arguments
///
/// - `$struct`: The name of the struct representing the identifier.
///
/// # Example
/// ```rust
/// // Convert a `String` into a `UserId`.
/// sqlx::query_as!(
///     UserId,
///     SELECT id FROM users
///     WHERE username = $1,
///     username
/// )
/// .fetch_optional(executor)
/// .await?;
/// ```
/// 
macro_rules! id_conversions {
    ($struct:ident) => {
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

id!(pub, UserId, 8, "users");

id!(pub, LoginSessionId, 12, "users");

id!(pub, ProjectId, 8, "projects");

id!(pub, ProjectMemberId, 8, "project_members");

id!(pub, TaskGroupId, 10, "task_groups" );

id!(pub, TaskId, 10, "tasks");

id!(pub, SubTaskId, 12, "task_groups");

id!(pub, AuditId, 12, "audits");
