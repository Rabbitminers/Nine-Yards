export interface Project {
    id: String,
    name: String,
    owner: String,
    icon_url: String,
    public: Boolean
}

export interface ProjectMember {
    id: String,
    project_id: String,
    user_id: String,
    permissions: String,
    accepted: Boolean,
}