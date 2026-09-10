-- Hand-authored (user-owned). Not regenerated.
--
-- Best-effort restore sketch for the tenancy strip (ADR-0029). This is a breaking module
-- release against dev-stage databases: the down re-adds the company_id column as nullable
-- with its plain index and the company isolation policy shape, but restores NO data —
-- rows written after the strip (or after the decorator re-keyed them) carry org_unit_id
-- only. The composing service's tenancy decorator remains the live fence; treat this
-- down as a schema-shape sketch for archaeology, not a usable rollback.

ALTER TABLE schedule.schedules               ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE schedule.schedule_organizations  ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE schedule.schedule_employees      ADD COLUMN IF NOT EXISTS company_id uuid;

CREATE INDEX IF NOT EXISTS idx_schedules_company_id               ON schedule.schedules (company_id);
CREATE INDEX IF NOT EXISTS idx_schedule_organizations_company_id  ON schedule.schedule_organizations (company_id);

CREATE POLICY schedules_company_isolation ON schedule.schedules
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
CREATE POLICY schedule_organizations_company_isolation ON schedule.schedule_organizations
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
CREATE POLICY schedule_employees_company_isolation ON schedule.schedule_employees
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
