-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
	password TEXT NOT NULL,
    email TEXT NOT NULL,
    icon_url TEXT,
	login_session TEXT
);

CREATE TABLE IF NOT EXISTS login_history (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT REFERENCES users NOT NULL,
    login_timestamp TEXT
);

CREATE TABLE IF NOT EXISTS project_members (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects NOT NULL,
    user_id TEXT REFERENCES users NOT NULL,
    permissions INTEGER NOT NULL,
    accepted BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    owner TEXT REFERENCES users NOT NULL,
    icon_url TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS task_groups (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects NOT NULL,
    name TEXT NOT NULL,
    position INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects NOT NULL,
    task_group_id TEXT REFERENCES task_groups NOT NULL,
    name TEXT NOT NULL,
    information TEXT NOT NULL,
    creator TEXT REFERENCES users NOT NULL,
    assignee TEXT REFERENCES users NOT NULL,
    position INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS notifications (
    id TEXT PRIMARY KEY NOT NULL,
    recipient TEXT REFERENCES users NOT NULL,
    body TEXT NOT NULL,
    notification_type TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);