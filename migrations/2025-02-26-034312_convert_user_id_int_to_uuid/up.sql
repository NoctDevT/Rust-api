ALTER TABLE users ADD COLUMN new_id UUID DEFAULT gen_random_uuid();

UPDATE users SET new_id = gen_random_uuid();  

ALTER TABLE users DROP COLUMN id;

ALTER TABLE users RENAME COLUMN new_id TO id;

ALTER TABLE users ADD PRIMARY KEY (id);
