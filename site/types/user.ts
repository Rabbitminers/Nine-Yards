export interface User {
    id: String,
    username: String,
    email: String,
}

export interface Login {
    username_or_email: String,
    password: String,
}