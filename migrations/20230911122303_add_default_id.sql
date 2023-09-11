-- Add migration script here
ALTER COLUMN id SET DEFAULT gen_random_uuid(),
