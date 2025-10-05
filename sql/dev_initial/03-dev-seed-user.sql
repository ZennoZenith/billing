-- root user (at serial_id = 0)
INSERT INTO "users" 
    (serial_id, user_id,   typ,    name,  email,             cid, ctime, mid, mtime) VALUES 
    (0,         'root', 'Sys',  'root','root@test.com', 0,   now(), 0,   now());

-- root password: super-secure-password
INSERT INTO "auth" 
    (user_serial_id, pwd, pwd_salt, cid, ctime, mid, mtime) VALUES 
    (
        0,
        '#02#$argon2id$v=19$m=19456,t=2,p=1$MAIY793WSiO3YGWIzAEPtg$BmgFaGPwgS5zzpfnVmyeRems6+aQQiY3ZN5h5OMntC4',
        '300218ef-ddd6-4a23-b760-6588cc010fb6',
        0,
        now(),
        0,
        now()
    );

-- User demo1
-- password: welcome
INSERT INTO "users" 
    (user_id, name,    email,              cid, ctime, mid, mtime) VALUES 
    ('demo1', 'demo1', 'demo1@test.com', 0,   now(), 0,   now());
INSERT INTO "auth" 
    (user_serial_id, pwd, pwd_salt, cid, ctime, mid, mtime) VALUES 
    (
        (SELECT serial_id FROM users WHERE user_id = 'demo1' LIMIT 1),
        '#02#$argon2id$v=19$m=19456,t=2,p=1$X0rT4G7dR4iwt5GvVm8mbg$6/Yrgluppw4SFrszzByiXd04cl2DHmlb1XCHhuDMBJM',
        '5f4ad3e0-6edd-4788-b0b7-91af566f266e',
        0,
        now(),
        0,
        now()
    );
