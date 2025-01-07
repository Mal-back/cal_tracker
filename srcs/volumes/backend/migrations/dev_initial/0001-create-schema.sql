-- User table
CREATE TABLE "user" (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  username VARCHAR(128) UNIQUE NOT NULL,
  password VARCHAR(255),
  password_salt UUID NOT NULL DEFAULT gen_random_uuid(),
  token_salt UUID NOT NULL DEFAULT gen_random_uuid()

);

CREATE TABLE public_user (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  owner BIGINT UNIQUE NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
  age INT NOT NULL,
  size_cm INT NOT NULL,
  weight REAL NOT NULL
);

-- tasks table
CREATE TABLE meal (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  name VARCHAR(128) NOT NULL,
  kcal INT NOT NULL,
  carbs INT NOT NULL,
  lipids INT NOT NULL,
  proteins INT NOT NULL,
  owner BIGINT NOT NULL REFERENCES public_user(owner)
);
