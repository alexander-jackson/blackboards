-- Update some tables to have sensible defaults and add NOT NULL constraints
ALTER TABLE sessions ALTER COLUMN title SET NOT NULL;
ALTER TABLE sessions ALTER COLUMN start_time SET NOT NULL;
ALTER TABLE sessions ALTER COLUMN remaining SET NOT NULL;

ALTER TABLE registrations ALTER COLUMN session_id SET NOT NULL;
ALTER TABLE registrations ALTER COLUMN warwick_id SET NOT NULL;

ALTER TABLE attendances ALTER COLUMN session_id SET NOT NULL;
ALTER TABLE attendances ALTER COLUMN warwick_id SET NOT NULL;

ALTER TABLE personal_bests ALTER COLUMN show_pl SET NOT NULL;
ALTER TABLE personal_bests ALTER COLUMN show_pl SET DEFAULT false;
ALTER TABLE personal_bests ALTER COLUMN show_wl SET NOT NULL;
ALTER TABLE personal_bests ALTER COLUMN show_wl SET DEFAULT false;
