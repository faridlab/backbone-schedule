-- Down: drop the declared rule constraints for schedule module
ALTER TABLE IF EXISTS schedule.schedule_weekdays DROP CONSTRAINT IF EXISTS ck_schedule_weekdays_schedule_id_required__required;
ALTER TABLE IF EXISTS schedule.schedule_organizations DROP CONSTRAINT IF EXISTS ck_schedule_organizations_name_max_length__max_length;
ALTER TABLE IF EXISTS schedule.schedule_employees DROP CONSTRAINT IF EXISTS ck_schedule_employees_employee_id_required__required;
ALTER TABLE IF EXISTS schedule.schedules DROP CONSTRAINT IF EXISTS ck_schedules_name_max_length__max_length;
ALTER TABLE IF EXISTS schedule.schedules DROP CONSTRAINT IF EXISTS ck_schedules_name_required__required;
