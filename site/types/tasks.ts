export interface TaskGroup {
    id: String,
    project_id: String,
    name: String,
    position: number,
}

export interface Task {
    id: String,
    project_id: String,
    task_group_id: String,
    name: String,
    information?: String,
    creator: String,
    due?: number,
    primary_colour: String,
    accent_colour: String,
    sub_tasks?: SubTask[]
    created: number,
}

export interface SubTask {
    id: String,
    task_id: String,
    assignee: String,
    body: String,
    completed: Boolean
}