use chrono::{DateTime, Utc, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

/// Strongly-typed ID for ScheduleEmployee
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ScheduleEmployeeId(pub Uuid);

impl ScheduleEmployeeId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for ScheduleEmployeeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ScheduleEmployeeId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for ScheduleEmployeeId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<ScheduleEmployeeId> for Uuid {
    fn from(id: ScheduleEmployeeId) -> Self { id.0 }
}

impl AsRef<Uuid> for ScheduleEmployeeId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for ScheduleEmployeeId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ScheduleEmployee {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub schedule_date: NaiveDate,
    pub order_number: i32,
    pub time_in: NaiveTime,
    pub time_out: NaiveTime,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl ScheduleEmployee {
    /// Create a builder for ScheduleEmployee
    pub fn builder() -> ScheduleEmployeeBuilder {
        <ScheduleEmployeeBuilder as Default>::default()
    }

    /// Create a new ScheduleEmployee with required fields
    pub fn new(employee_id: Uuid, schedule_date: NaiveDate, order_number: i32, time_in: NaiveTime, time_out: NaiveTime) -> Self {
        Self {
            id: Uuid::new_v4(),
            employee_id,
            schedule_date,
            order_number,
            time_in,
            time_out,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> ScheduleEmployeeId {
        ScheduleEmployeeId(self.id)
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
                "employee_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.employee_id = v; }
                }
                "schedule_date" => {
                    if let Ok(v) = serde_json::from_value(value) { self.schedule_date = v; }
                }
                "order_number" => {
                    if let Ok(v) = serde_json::from_value(value) { self.order_number = v; }
                }
                "time_in" => {
                    if let Ok(v) = serde_json::from_value(value) { self.time_in = v; }
                }
                "time_out" => {
                    if let Ok(v) = serde_json::from_value(value) { self.time_out = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for ScheduleEmployee {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "ScheduleEmployee"
    }
}

impl backbone_core::PersistentEntity for ScheduleEmployee {
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

impl backbone_orm::EntityRepoMeta for ScheduleEmployee {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("employee_id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for ScheduleEmployee entity
///
/// Provides a fluent API for constructing ScheduleEmployee instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct ScheduleEmployeeBuilder {
    employee_id: Option<Uuid>,
    schedule_date: Option<NaiveDate>,
    order_number: Option<i32>,
    time_in: Option<NaiveTime>,
    time_out: Option<NaiveTime>,
}

impl ScheduleEmployeeBuilder {
    /// Set the employee_id field (required)
    pub fn employee_id(mut self, value: Uuid) -> Self {
        self.employee_id = Some(value);
        self
    }

    /// Set the schedule_date field (required)
    pub fn schedule_date(mut self, value: NaiveDate) -> Self {
        self.schedule_date = Some(value);
        self
    }

    /// Set the order_number field (default: `0`)
    pub fn order_number(mut self, value: i32) -> Self {
        self.order_number = Some(value);
        self
    }

    /// Set the time_in field (required)
    pub fn time_in(mut self, value: NaiveTime) -> Self {
        self.time_in = Some(value);
        self
    }

    /// Set the time_out field (required)
    pub fn time_out(mut self, value: NaiveTime) -> Self {
        self.time_out = Some(value);
        self
    }

    /// Build the ScheduleEmployee entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<ScheduleEmployee, String> {
        let employee_id = self.employee_id.ok_or_else(|| "employee_id is required".to_string())?;
        let schedule_date = self.schedule_date.ok_or_else(|| "schedule_date is required".to_string())?;
        let time_in = self.time_in.ok_or_else(|| "time_in is required".to_string())?;
        let time_out = self.time_out.ok_or_else(|| "time_out is required".to_string())?;

        Ok(ScheduleEmployee {
            id: Uuid::new_v4(),
            employee_id,
            schedule_date,
            order_number: self.order_number.unwrap_or(0),
            time_in,
            time_out,
            metadata: AuditMetadata::default(),
        })
    }
}
