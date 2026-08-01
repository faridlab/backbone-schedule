-- Down: drop schedule.schedule_organizations table
DROP TABLE IF EXISTS schedule.schedule_organizations CASCADE;
DROP FUNCTION IF EXISTS schedule.schedule_organizations_audit_timestamp() CASCADE;
