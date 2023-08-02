-- Add migration script here
CREATE TABLE users (
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
	password TEXT NOT NULL,
    email TEXT NOT NULL,
    icon_url TEXT,
    login_session TEXT
);

CREATE TABLE project_members (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects NOT NULL,
    user_id TEXT REFERENCES users NOT NULL,
    permissions INTEGER NOT NULL,
    accepted BOOLEAN NOT NULL
);

CREATE TABLE projects (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    owner TEXT REFERENCES users NOT NULL,
    icon_url TEXT NOT NULL,
    public BOOLEAN NOT NULL
);

CREATE TABLE task_groups (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects NOT NULL,
    name TEXT NOT NULL,
    position INTEGER NOT NULL
);

CREATE TABLE tasks (
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

CREATE TABLE task_edges (
    parent_task TEXT REFERENCES tasks NOT NULL,
    child_task TEXT REFERENCES tasks NOT NULL
    project_id TEXT REFERENCES projects NOT NULL,
    flow_type TEXT NOT NULL,
    CONSTRAINT valid_edge CHECK (parent_task <> child_task)
    CONSTRAINT overlap UNIQUE (parent_task, child_task)
);

CREATE TABLE sub_tasks (
    id TEXT PRIMARY KEY NOT NULL,
    task_id TEXT REFERENCES tasks NOT NULL,
    project_id TEXT REFERENCES projects NOT NULL,
    assignee TEXT REFERENCES project_members,
    body TEXT NOT NULL,
    weight INTEGER,
    position INTEGER NOT NULL,
    completed BOOLEAN NOT NULL
);

CREATE TABLE labels (
    id TEXT PRIMARY KEY NOT NULL,
    task_id TEXT REFERENCES tasks NOT NULL,
    project_id TEXT REFERENCES projects NOT NULL,
    body TEXT NOT NULL,
    colour TEXT NOT NULL
);

CREATE TABLE audit_log (
    id TEXT PRIMARY KEY NOT NULL,
    auditor TEXT REFERENCES project_members NOT NULL,
    project_id TEXT REFERENCES projects NOT NULL,
    body TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE notifications (
    id TEXT PRIMARY KEY NOT NULL,
    recipient TEXT REFERENCES users NOT NULL,
    body TEXT NOT NULL,
    notification_type TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);