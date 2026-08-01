-- Down: drop schedule.schedules table
DROP TABLE IF EXISTS schedule.schedules CASCADE;
DROP FUNCTION IF EXISTS schedule.schedules_audit_timestamp() CASCADE;
