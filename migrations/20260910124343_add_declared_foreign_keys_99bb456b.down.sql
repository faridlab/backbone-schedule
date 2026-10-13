-- Down: drop the declared foreign keys for schedule module
ALTER TABLE IF EXISTS schedule.schedule_weekdays DROP CONSTRAINT IF EXISTS fk_schedule_weekdays_schedule_id;
