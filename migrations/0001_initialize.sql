DROP TABLE IF EXISTS projects;

CREATE TABLE IF NOT EXISTS projects (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  github_id INTEGER NOT NULL UNIQUE,
  name_with_owner TEXT NOT NULL UNIQUE,
  description TEXT,
  name TEXT NOT NULL,
  url TEXT NOT NULL,
  last_updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  config JSON NOT NULL DEFAULT '{}'
);

CREATE INDEX idx_projects_github_id ON projects(github_id);
CREATE INDEX idx_projects_name ON projects(name);
CREATE INDEX idx_projects_last_updated ON projects(last_updated);
