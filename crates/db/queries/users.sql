--: User()

--! get_users : User
SELECT 
    id, 
    email
FROM users;

-- Cornucopia will use the above definition to generate a Rust function called 
-- get_users to access the database. Note cornucopia checks the query at code 
-- generation time against Postgres.