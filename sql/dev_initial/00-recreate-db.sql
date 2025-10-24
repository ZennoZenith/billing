-- DEV ONLY - Brute Force DROP DB (for local dev and unit test)
select pg_terminate_backend(pid) from pg_stat_activity where
 usename = 'example_user' or datname = 'example_db';
drop database if exists example_db;
drop user if exists example_user;

-- DEV ONLY - Dev only password (for local dev and unit test).
create user example_user password 'dev_only_pwd';
create database example_db owner example_user encoding = 'UTF-8';