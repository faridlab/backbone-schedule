-- Down: drop schedule.schedule_weekdays table
DROP TABLE IF EXISTS schedule.schedule_weekdays CASCADE;
DROP FUNCTION IF EXISTS schedule.schedule_weekdays_audit_timestamp() CASCADE;
