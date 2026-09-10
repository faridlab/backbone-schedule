-- Hand-authored (user-owned). Not regenerated.
--
-- Strip every company-fence artifact from the schedule tables (ADR-0029): the module is
-- tenant-agnostic; org scoping is installed by the COMPOSING service's tenancy decorator,
-- never by the module. Dropped here, per table: the company-leading index, the
-- <table>_company_isolation RLS policy, and the company_id column itself.
--
-- Tables: schedules, schedule_organizations, and schedule_employees.
-- schedule_weekdays never carried company_id (it is scoped via its schedule_id) and is
-- not touched. schedule_employees carries no company-leading index — its only declared
-- index (employee_id, schedule_date) is tenant-free and stays.
--
-- Ordering guard (the decorator must run FIRST on any database with data): the module
-- never moves tenancy data. A table is safe to strip when EITHER
--   a) it carries org_unit_id with no NULLs — the decorator backfilled it from company_id —
--      or b) it is empty (a fresh database: the earlier chain files created it empty).
-- Otherwise the strip RAISEs, naming the decorator step, rather than dropping a column
-- that still holds the only tenancy key. The file is re-runnable (every drop is IF EXISTS
-- and the tracker has no checksums), so a failed run retries cleanly after the decorator
-- lands.
--
-- RLS enable/force flags are deliberately NOT touched: the decorator owns those now.
-- This module declares no uniques at all, so there is no unique to split into a
-- domain (tenant-free) form and a per-unit decorator form.

DO $$
DECLARE
    t text;
    has_org boolean;
    org_nulls bigint;
    total bigint;
    offenders text := '';
BEGIN
    FOREACH t IN ARRAY ARRAY['schedules', 'schedule_organizations', 'schedule_employees']
    LOOP
        IF to_regclass(format('schedule.%I', t)) IS NULL THEN
            CONTINUE; -- chain not fully applied on this database; nothing to strip
        END IF;

        SELECT EXISTS (
                   SELECT 1 FROM information_schema.columns
                   WHERE table_schema = 'schedule' AND table_name = t AND column_name = 'org_unit_id'
               )
        INTO has_org;

        EXECUTE format('SELECT count(*) FROM schedule.%I', t) INTO total;

        IF has_org THEN
            EXECUTE format(
                'SELECT count(*) FROM schedule.%I WHERE org_unit_id IS NULL', t)
            INTO org_nulls;
        ELSE
            org_nulls := total; -- no org column: every row's only tenancy key is company_id
        END IF;

        IF has_org AND org_nulls = 0 THEN
            CONTINUE; -- decorator backfilled: safe
        END IF;
        IF total = 0 THEN
            CONTINUE; -- empty table (fresh database): safe
        END IF;
        offenders := offenders || format(' schedule.%s (%s rows, %s rows not covered by org_unit_id);', t, total, org_nulls);
    END LOOP;

    IF offenders <> '' THEN
        RAISE EXCEPTION 'refusing to strip company_id — these tables are not yet covered by the tenancy decorator:%. Apply the composing service''s tenancy decorator (it backfills org_unit_id from company_id) and re-run; it is the only step that moves tenancy data.', offenders;
    END IF;
END $$;

-- ── schedules ─────────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS schedule.idx_schedules_company_id;
DROP POLICY IF EXISTS schedules_company_isolation ON schedule.schedules;
ALTER TABLE schedule.schedules DROP COLUMN IF EXISTS company_id;

-- ── schedule_organizations ────────────────────────────────────────────────────
DROP INDEX IF EXISTS schedule.idx_schedule_organizations_company_id;
DROP POLICY IF EXISTS schedule_organizations_company_isolation ON schedule.schedule_organizations;
ALTER TABLE schedule.schedule_organizations DROP COLUMN IF EXISTS company_id;

-- ── schedule_employees ────────────────────────────────────────────────────────
DROP POLICY IF EXISTS schedule_employees_company_isolation ON schedule.schedule_employees;
ALTER TABLE schedule.schedule_employees DROP COLUMN IF EXISTS company_id;
