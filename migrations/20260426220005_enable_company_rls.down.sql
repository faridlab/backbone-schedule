-- Down: remove the company RLS fence for schedule module

-- Reverse the company RLS fence for schedule.schedules
DROP POLICY IF EXISTS schedules_company_isolation ON schedule.schedules;
ALTER TABLE schedule.schedules NO FORCE ROW LEVEL SECURITY;
ALTER TABLE schedule.schedules DISABLE ROW LEVEL SECURITY;

-- Reverse the company RLS fence for schedule.schedule_employees
DROP POLICY IF EXISTS schedule_employees_company_isolation ON schedule.schedule_employees;
ALTER TABLE schedule.schedule_employees NO FORCE ROW LEVEL SECURITY;
ALTER TABLE schedule.schedule_employees DISABLE ROW LEVEL SECURITY;

-- Reverse the company RLS fence for schedule.schedule_organizations
DROP POLICY IF EXISTS schedule_organizations_company_isolation ON schedule.schedule_organizations;
ALTER TABLE schedule.schedule_organizations NO FORCE ROW LEVEL SECURITY;
ALTER TABLE schedule.schedule_organizations DISABLE ROW LEVEL SECURITY;

