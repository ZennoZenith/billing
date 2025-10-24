-- root user (at serial_id = 0)
insert into "users" 
    (serial_id, user_id, user_type_serial_id, name, email, ctime, mtime) values 
    (0, 'root', (select ut.serial_id from user_type ut where ut.typ = 'Sys'), 'root','root@example.com', now(), now());

-- root password: super-secure-password
insert into "password_auth"
    (user_serial_id, pwd, pwd_salt, ctime, mtime) values 
    (
        0,
        '#02#$argon2id$v=19$m=19456,t=2,p=1$MAIY793WSiO3YGWIzAEPtg$BmgFaGPwgS5zzpfnVmyeRems6+aQQiY3ZN5h5OMntC4',
        '300218ef-ddd6-4a23-b760-6588cc010fb6',
        now(),
        now()
    );

-- User demo1
-- password: welcome
insert into "users" 
    (user_id, name, email, ctime, mtime) values 
    ('demo1', 'demo1', 'demo1@example.com', now(), now());
insert into "password_auth" 
    (user_serial_id, pwd, pwd_salt, ctime, mtime) values 
    (
        (select serial_id from users where user_id = 'demo1' limit 1),
        '#02#$argon2id$v=19$m=19456,t=2,p=1$X0rT4G7dR4iwt5GvVm8mbg$6/Yrgluppw4SFrszzByiXd04cl2DHmlb1XCHhuDMBJM',
        '5f4ad3e0-6edd-4788-b0b7-91af566f266e',
        now(),
        now()
    );
