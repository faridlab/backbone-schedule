use chrono::{DateTime, Utc, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

/// Strongly-typed ID for Schedule
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ScheduleId(pub Uuid);

impl ScheduleId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for ScheduleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ScheduleId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for ScheduleId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<ScheduleId> for Uuid {
    fn from(id: ScheduleId) -> Self { id.0 }
}

impl AsRef<Uuid> for ScheduleId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for ScheduleId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Schedule {
    pub id: Uuid,
    pub company_id: Uuid,
    pub name: String,
    pub is_default: bool,
    pub order_number: i32,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub time_in: NaiveTime,
    pub time_out: NaiveTime,
    pub is_override_holiday: bool,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Schedule {
    /// Create a builder for Schedule
    pub fn builder() -> ScheduleBuilder {
        ScheduleBuilder::default()
    }

    /// Create a new Schedule with required fields
    pub fn new(company_id: Uuid, name: String, is_default: bool, order_number: i32, time_in: NaiveTime, time_out: NaiveTime, is_override_holiday: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            company_id,
            name,
            is_default,
            order_number,
            start_date: None,
            end_date: None,
            time_in,
            time_out,
            is_override_holiday,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> ScheduleId {
        ScheduleId(self.id)
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
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the start_date field (chainable)
    pub fn with_start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    /// Set the end_date field (chainable)
    pub fn with_end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "company_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.company_id = v; }
                }
                "name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.name = v; }
                }
                "is_default" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_default = v; }
                }
                "order_number" => {
                    if let Ok(v) = serde_json::from_value(value) { self.order_number = v; }
                }
                "start_date" => {
                    if let Ok(v) = serde_json::from_value(value) { self.start_date = v; }
                }
                "end_date" => {
                    if let Ok(v) = serde_json::from_value(value) { self.end_date = v; }
                }
                "time_in" => {
                    if let Ok(v) = serde_json::from_value(value) { self.time_in = v; }
                }
                "time_out" => {
                    if let Ok(v) = serde_json::from_value(value) { self.time_out = v; }
                }
                "is_override_holiday" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_override_holiday = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Schedule {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "Schedule"
    }
}

impl backbone_core::PersistentEntity for Schedule {
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

impl backbone_orm::EntityRepoMeta for Schedule {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("company_id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["name"]
    }
    fn company_field() -> Option<&'static str> {
        Some("company_id")
    }
}

/// Builder for Schedule entity
///
/// Provides a fluent API for constructing Schedule instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct ScheduleBuilder {
    company_id: Option<Uuid>,
    name: Option<String>,
    is_default: Option<bool>,
    order_number: Option<i32>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    time_in: Option<NaiveTime>,
    time_out: Option<NaiveTime>,
    is_override_holiday: Option<bool>,
}

impl ScheduleBuilder {
    /// Set the company_id field (required)
    pub fn company_id(mut self, value: Uuid) -> Self {
        self.company_id = Some(value);
        self
    }

    /// Set the name field (required)
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    /// Set the is_default field (default: `false`)
    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    /// Set the order_number field (default: `0`)
    pub fn order_number(mut self, value: i32) -> Self {
        self.order_number = Some(value);
        self
    }

    /// Set the start_date field (optional)
    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    /// Set the end_date field (optional)
    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
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

    /// Set the is_override_holiday field (default: `false`)
    pub fn is_override_holiday(mut self, value: bool) -> Self {
        self.is_override_holiday = Some(value);
        self
    }

    /// Build the Schedule entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Schedule, String> {
        let company_id = self.company_id.ok_or_else(|| "company_id is required".to_string())?;
        let name = self.name.ok_or_else(|| "name is required".to_string())?;
        let time_in = self.time_in.ok_or_else(|| "time_in is required".to_string())?;
        let time_out = self.time_out.ok_or_else(|| "time_out is required".to_string())?;

        Ok(Schedule {
            id: Uuid::new_v4(),
            company_id,
            name,
            is_default: self.is_default.unwrap_or(false),
            order_number: self.order_number.unwrap_or(0),
            start_date: self.start_date,
            end_date: self.end_date,
            time_in,
            time_out,
            is_override_holiday: self.is_override_holiday.unwrap_or(false),
            metadata: AuditMetadata::default(),
        })
    }
}
