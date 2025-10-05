-- DEV ONLY - Brute Force DROP DB (for local dev and unit test)
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
 usename = 'test_user' OR datname = 'test_db';
DROP DATABASE IF EXISTS test_db;
DROP USER IF EXISTS test_user;

-- DEV ONLY - Dev only password (for local dev and unit test).
CREATE USER test_user PASSWORD 'dev_only_pwd';
CREATE DATABASE test_db owner test_user ENCODING = 'UTF-8';