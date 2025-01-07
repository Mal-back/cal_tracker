-- DEV ONLY -- BRUT FORCE DROP DB

-- Kick ll users
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
  usename = 'app_user' or datname = 'app_db';

-- Drop db and user
DROP DATABASE IF EXISTS app_db; 
DROP USER IF EXISTS app_user; 

-- Recreate db and user
CREATE USER app_user WITH PASSWORD 'dev_only';
CREATE DATABASE app_db OWNER app_user ENCODING = 'UTF-8';
