use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::WeekdayType;
use super::AuditMetadata;

/// Strongly-typed ID for ScheduleWeekday
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ScheduleWeekdayId(pub Uuid);

impl ScheduleWeekdayId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for ScheduleWeekdayId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ScheduleWeekdayId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for ScheduleWeekdayId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<ScheduleWeekdayId> for Uuid {
    fn from(id: ScheduleWeekdayId) -> Self { id.0 }
}

impl AsRef<Uuid> for ScheduleWeekdayId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for ScheduleWeekdayId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ScheduleWeekday {
    pub id: Uuid,
    pub schedule_id: Uuid,
    pub sun: WeekdayType,
    pub mon: WeekdayType,
    pub tue: WeekdayType,
    pub wed: WeekdayType,
    pub thu: WeekdayType,
    pub fri: WeekdayType,
    pub sat: WeekdayType,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl ScheduleWeekday {
    /// Create a builder for ScheduleWeekday
    pub fn builder() -> ScheduleWeekdayBuilder {
        ScheduleWeekdayBuilder::default()
    }

    /// Create a new ScheduleWeekday with required fields
    pub fn new(schedule_id: Uuid, sun: WeekdayType, mon: WeekdayType, tue: WeekdayType, wed: WeekdayType, thu: WeekdayType, fri: WeekdayType, sat: WeekdayType) -> Self {
        Self {
            id: Uuid::new_v4(),
            schedule_id,
            sun,
            mon,
            tue,
            wed,
            thu,
            fri,
            sat,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> ScheduleWeekdayId {
        ScheduleWeekdayId(self.id)
    }

    /// Get when this entity was created
    pub fn created_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.created_at.as_ref()
    }

    /// Get when this entity was last updated
    pub fn updated_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.updated_at.as_ref()
    }

    /// Check if this entity is soft deleted
    pub fn is_deleted(&self) -> bool {
        self.metadata.deleted_at.is_some()
    }

    /// Check if this entity is active (not deleted)
    pub fn is_active(&self) -> bool {
        self.metadata.deleted_at.is_none()
    }

    /// Get when this entity was deleted
    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.deleted_at.as_ref()
    }

    /// Get who created this entity
    pub fn created_by(&self) -> Option<&Uuid> {
        self.metadata.created_by.as_ref()
    }

    /// Get who last updated this entity
    pub fn updated_by(&self) -> Option<&Uuid> {
        self.metadata.updated_by.as_ref()
    }

    /// Get who deleted this entity
    pub fn deleted_by(&self) -> Option<&Uuid> {
        self.metadata.deleted_by.as_ref()
    }


    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "schedule_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.schedule_id = v; }
                }
                "sun" => {
                    if let Ok(v) = serde_json::from_value(value) { self.sun = v; }
                }
                "mon" => {
                    if let Ok(v) = serde_json::from_value(value) { self.mon = v; }
                }
                "tue" => {
                    if let Ok(v) = serde_json::from_value(value) { self.tue = v; }
                }
                "wed" => {
                    if let Ok(v) = serde_json::from_value(value) { self.wed = v; }
                }
                "thu" => {
                    if let Ok(v) = serde_json::from_value(value) { self.thu = v; }
                }
                "fri" => {
                    if let Ok(v) = serde_json::from_value(value) { self.fri = v; }
                }
                "sat" => {
                    if let Ok(v) = serde_json::from_value(value) { self.sat = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for ScheduleWeekday {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "ScheduleWeekday"
    }
}

impl backbone_core::PersistentEntity for ScheduleWeekday {
    fn entity_id(&self) -> String {
        self.id.to_string()
    }
    fn set_entity_id(&mut self, id: String) {
        if let Ok(uuid) = uuid::Uuid::parse_str(&id) {
            self.id = uuid;
        }
    }
    fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.created_at
    }
    fn set_created_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.created_at = Some(ts);
    }
    fn updated_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.updated_at
    }
    fn set_updated_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.updated_at = Some(ts);
    }
    fn deleted_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.deleted_at
    }
    fn set_deleted_at(&mut self, ts: Option<chrono::DateTime<chrono::Utc>>) {
        self.metadata.deleted_at = ts;
    }
}

impl backbone_orm::EntityRepoMeta for ScheduleWeekday {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("schedule_id".to_string(), "uuid".to_string());
        m.insert("sun".to_string(), "weekday_type".to_string());
        m.insert("mon".to_string(), "weekday_type".to_string());
        m.insert("tue".to_string(), "weekday_type".to_string());
        m.insert("wed".to_string(), "weekday_type".to_string());
        m.insert("thu".to_string(), "weekday_type".to_string());
        m.insert("fri".to_string(), "weekday_type".to_string());
        m.insert("sat".to_string(), "weekday_type".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for ScheduleWeekday entity
///
/// Provides a fluent API for constructing ScheduleWeekday instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct ScheduleWeekdayBuilder {
    schedule_id: Option<Uuid>,
    sun: Option<WeekdayType>,
    mon: Option<WeekdayType>,
    tue: Option<WeekdayType>,
    wed: Option<WeekdayType>,
    thu: Option<WeekdayType>,
    fri: Option<WeekdayType>,
    sat: Option<WeekdayType>,
}

impl ScheduleWeekdayBuilder {
    /// Set the schedule_id field (required)
    pub fn schedule_id(mut self, value: Uuid) -> Self {
        self.schedule_id = Some(value);
        self
    }

    /// Set the sun field (default: `WeekdayType::default()`)
    pub fn sun(mut self, value: WeekdayType) -> Self {
        self.sun = Some(value);
        self
    }

    /// Set the mon field (default: `WeekdayType::default()`)
    pub fn mon(mut self, value: WeekdayType) -> Self {
        self.mon = Some(value);
        self
    }

    /// Set the tue field (default: `WeekdayType::default()`)
    pub fn tue(mut self, value: WeekdayType) -> Self {
        self.tue = Some(value);
        self
    }

    /// Set the wed field (default: `WeekdayType::default()`)
    pub fn wed(mut self, value: WeekdayType) -> Self {
        self.wed = Some(value);
        self
    }

    /// Set the thu field (default: `WeekdayType::default()`)
    pub fn thu(mut self, value: WeekdayType) -> Self {
        self.thu = Some(value);
        self
    }

    /// Set the fri field (default: `WeekdayType::default()`)
    pub fn fri(mut self, value: WeekdayType) -> Self {
        self.fri = Some(value);
        self
    }

    /// Set the sat field (default: `WeekdayType::default()`)
    pub fn sat(mut self, value: WeekdayType) -> Self {
        self.sat = Some(value);
        self
    }

    /// Build the ScheduleWeekday entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<ScheduleWeekday, String> {
        let schedule_id = self.schedule_id.ok_or_else(|| "schedule_id is required".to_string())?;

        Ok(ScheduleWeekday {
            id: Uuid::new_v4(),
            schedule_id,
            sun: self.sun.unwrap_or(WeekdayType::default()),
            mon: self.mon.unwrap_or(WeekdayType::default()),
            tue: self.tue.unwrap_or(WeekdayType::default()),
            wed: self.wed.unwrap_or(WeekdayType::default()),
            thu: self.thu.unwrap_or(WeekdayType::default()),
            fri: self.fri.unwrap_or(WeekdayType::default()),
            sat: self.sat.unwrap_or(WeekdayType::default()),
            metadata: AuditMetadata::default(),
        })
    }
}
