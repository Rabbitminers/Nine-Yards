export interface TaskGroup {
	id: string;
	project_id: string;
	name: string;
	position: number;
}

export interface Task {
	id: string;
	project_id: string;
	task_group_id: string;
	name: string;
	information?: string;
	creator: string;
	position: number;
	due?: number;
	primary_colour: string;
	accent_colour: string;
	sub_tasks?: SubTask[];
	created: number;
}

export interface SubTask {
	id: string;
	task_id: string;
	assignee?: string;
	body?: string;
	completed: Boolean;
}
