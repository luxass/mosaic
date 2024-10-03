ALTER TABLE mosaic_repositories
    ALTER COLUMN github_id SET DATA TYPE VARCHAR
    USING github_id::VARCHAR;
