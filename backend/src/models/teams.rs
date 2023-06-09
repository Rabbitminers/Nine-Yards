use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use super::{ids::{TeamId, UserId, generate_team_id, generate_team_member_id, TeamMemberId}, users::User};

pub struct Team {
    pub id: TeamId,
}

impl Team {
    pub async fn create(creator: User, conn: &SqlitePool) -> Result<TeamId, super::DatabaseError> {
        let team_id = generate_team_id(conn).await?;

        let team = sqlx::query!(
            "
            INSERT INTO teams (id)
            VALUES ($1)
            ",
            team_id.0
        )
        .execute(conn)
        .await?;

        let team_member_id = generate_team_member_id(conn).await?;
        let team_member_permissions = Permissions::ALL.bits() as i64;

        let team_member = sqlx::query!(
            "
            INSERT INTO team_members (
                id, team_id, user_id, 
                permissions, accepted
            )
            VALUES ($1, $2, $3, $4, $5)
            ",
            team_member_id.0,
            team_id.0,
            creator.id.0,
            team_member_permissions,
            true
        )            
        .execute(conn)
        .await?;

        Ok(team_id)
    }

    pub async fn get_members(
        &self, 
        conn: &SqlitePool
    ) -> Result<Vec<TeamMember>, sqlx::Error> {
        let users =sqlx::query!(
            "
            SELECT id, team_id, user_id, permissions, accepted 
            FROM team_members 
            WHERE team_id = ?
            ",
            self.id.0
        )
        .fetch_many(conn)
        .try_filter_map(|e| async {
            Ok(e.right().map(|m| TeamMember {
                id: TeamMemberId(m.id),
                team_id: TeamId(m.team_id),
                user_id: UserId(m.user_id),
                permissions: Permissions::from_bits(m.permissions as u64).unwrap_or_default(),
                accepted: m.accepted,
            }))
        })
        .try_collect::<Vec<TeamMember>>()
        .await?;

        Ok(users)
    }

    pub async fn invite_member(
        &self, 
        conn: &SqlitePool
    ) ->  Result<(), sqlx::error::Error> {
        Ok(())
    }
}

bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct Permissions: u64 {
        const MANAGE_TASKS = 1 << 0;
        const MANAGE_PROJECT = 1 << 1;
        const MANAGE_TEAM = 1 << 2;
        const ALL = Self::MANAGE_TASKS.bits 
            | Self::MANAGE_PROJECT.bits
            | Self::MANAGE_TEAM.bits;
    }
}

impl Default for Permissions {
    fn default() -> Permissions {
        Permissions::MANAGE_TASKS
    }
}

pub struct TeamMember {
    pub id: TeamMemberId,
    pub team_id: TeamId,
    pub user_id: UserId,
    pub permissions: Permissions,
    pub accepted: bool,
}

impl TeamMember {    
    pub fn of(user: User) -> Self {
        unimplemented!()
    }
}