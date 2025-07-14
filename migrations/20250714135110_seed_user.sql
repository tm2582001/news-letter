-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    '96ab743d-6c22-4bb6-b97d-253b871a0197',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$OEx/rcq+3ts//'
'WUDzGNl2g$Am8UFBA4w5NJEmAtquGvBmAlu92q/VQcaoL5AyJPfc8'
)