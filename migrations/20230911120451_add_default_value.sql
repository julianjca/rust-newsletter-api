-- Update Default Value of subscribed_at Column
ALTER TABLE subscriptions
ALTER COLUMN subscribed_at SET DEFAULT NOW();