-- migrations/{timestamp}_create_subscriptions_table.sql

-- Create Subscriptions Table 
-- migrations/{timestamp}_create_subscriptions_table.sql

-- Create Subscriptions Table 
CREATE TABLE subscriptions (
  id uuid DEFAULT gen_random_uuid() NOT NULL,
  PRIMARY KEY (id),
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  subscribed_at timestamptz DEFAULT NOW() NOT NULL
);