export interface User {
	id: String;
	username: String;
	email: String;
}

export interface Login {
	username_or_email: String;
	password: String;
}

export interface Register {
	username: String;
	email: String;
	password: String;
}

export type Token = string | null;
