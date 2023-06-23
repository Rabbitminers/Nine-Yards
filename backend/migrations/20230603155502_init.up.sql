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
    icon_url TEXT NOT NULL,
    public BOOLEAN NOT NULL
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
    information TEXT,
    creator TEXT REFERENCES users NOT NULL,
    due DATETIME,
    primary_colour TEXT NOT NULL,
    accent_colour TEXT NOT NULL,
    position INTEGER NOT NULL,
    created DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS lables (
    id TEXT PRIMARY KEY NOT NULL,
    task_id TEXT REFERENCES tasks NOT NULL,
    project_member_id TEXT REFERENCES project_members NOT NULL
);

CREATE TABLE IF NOT EXISTS sub_tasks (
    id TEXT PRIMARY KEY NOT NULL,
    task_id TEXT REFERENCES tasks NOT NULL,
    project_id TEXT REFERENCES projects NOT NULL,
    assignee TEXT REFERENCES project_members,
    body TEXT NOT NULL,
    weight INTEGER,
    position INTEGER NOT NULL,
    completed BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS audit_log (
    id TEXT PRIMARY KEY NOT NULL,
    auditor TEXT REFERENCES project_members NOT NULL,
    project_id TEXT REFERENCES projects NOT NULL,
    body TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS notifications (
    id TEXT PRIMARY KEY NOT NULL,
    recipient TEXT REFERENCES users NOT NULL,
    body TEXT NOT NULL,
    notification_type TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);