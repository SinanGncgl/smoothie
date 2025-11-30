// Window service - manage window positions and states

use crate::{
    db::Database,
    error::{Result, SmoothieError},
};
use serde::Serialize;
use uuid::Uuid;

/// Window DTO for API responses
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowDto {
    pub id: String,
    pub profile_id: String,
    pub app_id: String,
    pub monitor_id: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub is_maximized: bool,
    pub state: String,
}

/// Helper to parse UUID from string
fn parse_uuid(s: &str) -> Result<Uuid> {
    Uuid::parse_str(s).map_err(|_| SmoothieError::ValidationError(format!("Invalid UUID: {}", s)))
}

pub struct WindowService;

impl WindowService {
    pub async fn create_window(
        db: &Database,
        profile_id: &str,
        app_id: &str,
        monitor_id: &str,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        is_maximized: bool,
        state: String,
    ) -> Result<WindowDto> {
        let id = Uuid::new_v4();
        let _profile_uuid = parse_uuid(profile_id)?;
        let _app_uuid = parse_uuid(app_id)?;
        let _monitor_uuid = parse_uuid(monitor_id)?;

        sqlx::query(
            "INSERT INTO windows (id, profile_id, app_id, monitor_id, x, y, width, height, is_maximized, state) 
             VALUES ($1, $2::uuid, $3::uuid, $4::uuid, $5, $6, $7, $8, $9, $10)"
        )
        .bind(id)
        .bind(profile_id)
        .bind(app_id)
        .bind(monitor_id)
        .bind(x)
        .bind(y)
        .bind(width)
        .bind(height)
        .bind(is_maximized)
        .bind(&state)
        .execute(db.pool())
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(WindowDto {
            id: id.to_string(),
            profile_id: profile_id.to_string(),
            app_id: app_id.to_string(),
            monitor_id: monitor_id.to_string(),
            x,
            y,
            width,
            height,
            is_maximized,
            state,
        })
    }

    pub async fn get_windows(db: &Database, profile_id: &str) -> Result<Vec<WindowDto>> {
        let _profile_uuid = parse_uuid(profile_id)?;
        
        let rows = sqlx::query_as::<_, (String, String, String, String, i32, i32, i32, i32, bool, String)>(
            "SELECT id::text, profile_id::text, app_id::text, monitor_id::text, x, y, width, height, is_maximized, state FROM windows WHERE profile_id = $1::uuid"
        )
        .bind(profile_id)
        .fetch_all(db.pool())
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|(id, profile_id, app_id, monitor_id, x, y, width, height, is_maximized, state)| {
            WindowDto {
                id,
                profile_id,
                app_id,
                monitor_id,
                x,
                y,
                width,
                height,
                is_maximized,
                state,
            }
        }).collect())
    }

    pub async fn update_window_position(
        db: &Database,
        window_id: &str,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<WindowDto> {
        let _window_uuid = parse_uuid(window_id)?;

        sqlx::query("UPDATE windows SET x = $1, y = $2, width = $3, height = $4 WHERE id = $5::uuid")
            .bind(x)
            .bind(y)
            .bind(width)
            .bind(height)
            .bind(window_id)
            .execute(db.pool())
            .await
            .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        let row = sqlx::query_as::<_, (String, String, String, String, i32, i32, i32, i32, bool, String)>(
            "SELECT id::text, profile_id::text, app_id::text, monitor_id::text, x, y, width, height, is_maximized, state FROM windows WHERE id = $1::uuid"
        )
        .bind(window_id)
        .fetch_one(db.pool())
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(WindowDto {
            id: row.0,
            profile_id: row.1,
            app_id: row.2,
            monitor_id: row.3,
            x: row.4,
            y: row.5,
            width: row.6,
            height: row.7,
            is_maximized: row.8,
            state: row.9,
        })
    }

    pub async fn delete_window(db: &Database, window_id: &str) -> Result<()> {
        let _window_uuid = parse_uuid(window_id)?;

        let result = sqlx::query("DELETE FROM windows WHERE id = $1::uuid")
            .bind(window_id)
            .execute(db.pool())
            .await
            .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(SmoothieError::NotFound("Window not found".into()));
        }

        Ok(())
    }
}
