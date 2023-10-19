-- Your SQL goes here
CREATE TABLE IF NOT EXISTS tasks
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    rewardsat INTEGER NOT NULL,
    link TEXT NOT NULL,
    creator_name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS users
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    address TEXT NOT NULL,
    balance INTEGER NOT Null,
    badge TEXT NOT NULL
    
);