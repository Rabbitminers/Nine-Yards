-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
	password TEXT NOT NULL,
    email TEXT NOT NULL,
    icon_url TEXT,
	login_session TEXT
);

CREATE TABLE IF NOT EXISTS teams (
    id TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE IF NOT EXISTS team_members (
    id TEXT PRIMARY KEY NOT NULL,
    team_id TEXT REFERENCES teams NOT NULL,
    user_id TEXT REFERENCES users NOT NULL,
    permissions INTEGER NOT NULL,
    accepted BOOLEAN NOT NULL,
    member_name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY NOT NULL,
    team_id TEXT REFERENCES teams NOT NULL,
    name TEXT NOT NULL
    icon_url TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS task_groups (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects NOT NULL
);

CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects NOT NULL,
    task_group_id TEXT REFERENCES task_groups NOT NULL,
    title TEXT NOT NULL,
    summary TEXT,
    creator TEXT REFERENCES users NOT NULL,
    assignee TEXT REFERENCES users NOT NULL
);

CREATE TABLE IF NOT EXISTS notifications (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT REFERENCES users NOT NULL
);