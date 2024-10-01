CREATE TABLE IF NOT EXISTS mosaic_repositories (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  github_id INTEGER NOT NULL UNIQUE,
  name_with_owner TEXT NOT NULL UNIQUE,
  description TEXT,
  name TEXT NOT NULL,
  url TEXT NOT NULL,
  last_updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  config JSON NOT NULL DEFAULT '{}'
);

CREATE INDEX idx_mosaic_repositories_github_id ON mosaic_repositories(github_id);
CREATE INDEX idx_mosaic_repositories_name ON mosaic_repositories(name);
CREATE INDEX idx_mosaic_repositories_last_updated ON mosaic_repositories(last_updated);
