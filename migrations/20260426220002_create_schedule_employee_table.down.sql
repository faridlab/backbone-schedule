-- Down: drop schedule.schedule_employees table
DROP TABLE IF EXISTS schedule.schedule_employees CASCADE;
DROP FUNCTION IF EXISTS schedule.schedule_employees_audit_timestamp() CASCADE;
