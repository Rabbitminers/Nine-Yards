<p align="center">
<img style="width: 15em;" src="display/fern.svg" alt="fern"/>
<h1 align="center"> Nine Yards API </h1>
</p>

## Overview

If you would like to help develop Nine Yards or make your own extension app for your Nine Yards instance this document can be used as a reference for the API so you dont need to go digging through the source code to work out what does what.

This document may out of date. If you find an inaccuracy you can report issues [here](https://github.com/Rabbitminers/Nine-Yards/issues). Aswell as find the source code for all of these routes [here](https://github.com/Rabbitminers/Nine-Yards/tree/master/backend/src/routes).

Got any other questions? You can also find us on discord [here](https://discord.gg/GJsQadv9Mc)

---

### Organisation

For the most features there are two seperate paths for there routes which we will call the generic routes and specific routes. For example `/tasks/{task_id}/sub-tasks` is a generic route for getting all of the sub tasks on a specific task, whereas if you would want to get a specific sub task you would call the sub task specific scope `/sub-tasks/{sub_task_id}`. You will find many examples below.

In this documentation. Endpoints are generally given by scope then length, shortest first, for example Project routes will be listed before task routes as they have a wider scope. Routes will be organised by their scope then results will be listed by method.

For example:

> `api/v1/some-scope`
>
>   `METHOD`
>
>  Explanation of the method
>
>  ```json 
>   The body of the method (if present)
>   ```
>
>  ```json 
>   The OK response of the method (if relevant)
>   ```
>  ...

---

### Terms, Names & Other Jargon

- Projects: The widest scope of organisation, where teams are organised etc.
- Task Groups: Groups of tasks, for example the individual columns of the kanban board in your project.
- Tasks: The individual "cards" in the kanban board, that contain collections of short term goals.
- Sub-tasks: The individual goal on each tasks. Usually a tick box and what team members can be assigned to.
- Projects member: Slightly different from users. Essentailly a wrapper around a user indicating their membership containing information about their role and permission in  aproject.
- Public Projects: Projects that can be viewed without a membership, however a membership is still needed to interact with the project, i.e complete or create a task.

---

### Endpoints

#### Authentication

`/api/v1/account`

`POST`

Edits the account of the supplied bearer token. All fields are optional and an example body containing all editable fields are optional can be found below:

```json
{
    "username": "A new username", // Must be unique
    "password": "Some new password", // If a mail server is provided confirmation will be requested.
    "profile_picture": "Some image url"
}
```

`DELETE`

Removes the account of the supplied bearer token. If a mail server is provided in the setup of your Nine Yards instance, for security, an email will be sent to the user confirming this removal.

`/api/v1/account/register`

`POST`

Creates a new user, requires no existing authentication unless the Nine-Yards instance is private.

All data should be supplied in the body as JSON like so:

```json
{
    "username": "Some username", // Must be between 3 - 30 Charachters (configurable in setup)
    "password": "Some password",
    "email": "Some e-mail address" // Depending on setup only format may be checked.
}
```

`/api/v1/account/login`

`POST`

Creates a new login session for the given user, returing a bearer token to be used for authentication. Do not share this with anyone.

If you already have an existing login session it will be overwritten disabling the existing token.

Example Body:

```json
{
    "username_or_email": "Either your accounts email or password",
    "password": "Corresponding password"
}
```

Example Response:

```json
{
    "message": ...,
    "body": {
        "token": "Some bearer token"
    }
}
```

This can then be used for authenticated routes by attaching the token to the header of your requests, example for cURL:

```bash
curl -X POST \
    -H 'Content-Type: application/json'  \
    -H 'Authorization: bearer TOKEN' \ # Replace TOKEN with your bearer token
    -i 'https://...'
```

`/api/v1/account/logout`

`POST`

Removes the current login session, requires a bearer token.

#### Projects

`/api/v1/projects/{id}/`

`GET`

Fetches information about the related project, requires authorized bearer token unless the project is public

Example Response

```json
{
    "message": ...,
    "data": {
        "id": "The Project's id",
        "naem": "The name of the project",
        "owner": "The Project owner's id",
        "icon_url": "The projects icon",
        "public": boolean
    }
}
```

`DELETE`

Removes the given project, requires an authorized bearer token even if the project is public, and for the given user to be owner of the project.

Also removes all attached sub-tasks, tasks, task-groups, project memberships, task-labels, invitation notifications, audits etc...

`POST`

Edits the project, requires an authorized bearer token and for the given member to have permission to modify the project. An example body changing all editable fields can be found below: 

Example Body: (All fields are optional)

```json
{
    "name": "The name of the project", // Must be between 3 - 30 charachters
    "icon_url": "Some image url",
    "owner": "The new owner's membership id", // Must be owner & new owner must be in project team.
    "public": boolean
}
```

`/api/v1/projects/{id}/audit-log`

`GET`

Retrives the recent audits to the given project, requires an authorized bearer token unless the project is public

Example Response

```json
{
    "message": ...
    "data": [
        {
            "id": "The audit's id",
            "auditor": "The auditor's id"
            "project_id": "The project's id",
            "body": "The audit body",
            "timestamp": "The time the audit was created"
        }
        ...
    ]
}
```
`/api/v1/projects/{id}/members/`

`GET`

Retrieves a full list of the projects members, aswell as their roles and permissions. Requires a bearer token unless the project is public. This will not retrive information about the users aside from their id.

Example Response

```json
{
    "message": ...
    "data": [
        {
            "id": "The member's id",
            "project_id": "The project's id",
            "user_id": "The user's id",
            "permissions": "The user's permission level",
            "accepted": boolean
        },
        ...
    ]
}
```

`/api/v1/projects/{id}/tasks`

`GET`

Fetches all tasks in the project, requires an authorized bearer token unless the project is public

Example Response

```json
{
    "message": ...
    "data": [
        {  
            "id": "The task's id",
            "project_id": "The task's project's id",
            "task_group_id": "The task's group's id",
            "name": "The tasks name",
            "information": "Optional - Information about the task",
            "creator": "The creator of the tasks id",
            "due": "Optional - The date the task should be completed by",
            "primary_colour": "The primary colour of the task",
            "accent_colour": "The accent colour of the task",
            "position": integer, // The location of thet task in it's group
            "sub_tasks": [
                {
                    "id": "The sub task's id",
                    "task_id": "The task's id",
                    "project_id": "The project's id",
                    "assignee": "Optional - The member assigned the task's id",
                    "body": "The text describing the task",
                    "weight": integer, // Optional - How much this sub task should affect the progress
                    "position": integer, // The location of thet sub task in it's task
                    "completed": bool,
                }
                ...
            ],
            "created": "The creation timestamp"
        },
        ...
    ]
}
```

`/api/v1/projects/{id}/task-groups`

`GET`

Gets all the task groups in the project, requires an authorized bearer token unless the project is public

Example Response:

```json
{
    "message": ...,
    "data": [
        {
            "id": "The task group's id",
            "project_id": "The task groups parent project's id",
            "name": "The name of the task group",
            "position": integer // The location of the task group in the project
        },
        ...
    ]
}
```


`POST`

Creates a new task group in the project, requires an authorized bearer token and for the given member to have task management permissions.

Takes a JSON body to the request of the given name for example: 

```json
{
    "name": "My task group" // Must be between 3 - 30 charachters and must contain charachters other than whitespace
}
```

#### Task Groups

`/api/v1/task-groups/{id}/`

`GET`

Fetches information about the given task group. Requires an authorized bearer token unless the project is public

Example Response:

```json
{   
    "message": ...,
    "data": {
        "id": "The id of the task group",
        "project_id": "The parent project's id",
        "name": "The task group's name",
        "position": integer, // The position of the task group in the project
    }
}
```

`POST`

This will edit the given task group. This requires an authorized bearer token even if the project is public and for the given member to have task management permissions. 

```json
{
    "name": "The task group's name",
    "position": integer, // The position of the task group in the project
}
```

`REMOVE`

Removes the specified task group, requires an authorized bearer token even if the project is public and for the given member to have task management permissions.

This will also remove all attached tasks, sub-tasks, task-labels etc and update the position of other task-groups in the same group

`/api/v1/task-groups/{id}/tasks`

`GET`

Fetches all tasks in the project, requires an authorized bearer token unless the project is public.

Example Response

```json
{
    "message": ...
    "data": [
        {  
            "id": "The task's id",
            "project_id": "The task's project's id",
            "task_group_id": "The task's group's id",
            "name": "The tasks name",
            "information": "Optional - Information about the task",
            "creator": "The creator of the tasks id",
            "due": "Optional - The date the task should be completed by",
            "primary_colour": "The primary colour of the task",
            "accent_colour": "The accent colour of the task",
            "position": integer, // The location of thet task in it's group
            "sub_tasks": [
                {
                    "id": "The sub task's id",
                    "task_id": "The task's id",
                    "project_id": "The project's id",
                    "assignee": "Optional - The member assigned the task's id",
                    "body": "The text describing the task",
                    "weight": integer, // Optional - How much this sub task should affect the progress
                    "position": integer, // The location of thet sub task in it's task
                    "completed": bool,
                }
                ...
            ],
            "created": "The creation timestamp"
        },
        ...
    ]
}
```

#### Tasks

`/api/v1/tasks/{id}/`

`GET`

Fetches information about the specified task, requires an authorized bearer token unless the project is public.

Example Response:

```json
{
    "message": ...
    "data": {
        "id": "The task's id",
        "project_id": "The task's project's id",
        "task_group_id": "The task's group's id",
        "name": "The tasks name",
        "information": "Optional - Information about the task",
        "creator": "The creator of the tasks id",
        "due": "Optional - The date the task should be completed by",
        "primary_colour": "The primary colour of the task",
        "accent_colour": "The accent colour of the task",
        "position": integer, // The location of thet task in it's group
        "sub_tasks": [
            {
                "id": "The sub task's id",
                "task_id": "The task's id",
                "project_id": "The project's id",
                "assignee": "Optional - The member assigned the task's id",
                "body": "The text describing the task",
                "weight": integer, // Optional - How much this sub task should affect the progress
                "position": integer, // The location of thet sub task in it's task
                "completed": bool,
            }
            ...
        ],
        "created": "The creation timestamp"
    },
}
```

`POST`

Edits the specified task, requires an authorized bearer token even if the project is public and for the given member to have task management permissions. The below example body contains all editable fields.

Example Body: (All fields are optional)

```json
{
    "name": "The tasks name",
    "information": "Information about the task",
    "due": integer // The timestamp the task should be completed by, when editing this must be in the future
    "primary_colour": "The primary colour of the task",
    "accent_colour": "The accent colour of the task",
    "position": integer, // The location of thet task in it's group
}
```

`REMOVE`

Removes the specified task, requires an authorized bearer token even if the project is public and for the given member to have task management permissions.

This will also remove all attached sub-tasks and update the position of other tasks in the same group

`/api/v1/tasks/{id}/sub-tasks`

`GET`

Fetches all sub tasks of the given task, requires an authorized bearer token unless the project is public.

Example Response:

```json
{
    "message": ...,
    "data": [
        {
            "id": "The sub-task's id",
            "task_id": "The parent task's id",
            "project_id": "The parent project's id",
            "assignee": "Optional - The assigned project member's id",
            "body": "A description of the sub-task",
            "weight": integer, // Optional - the influence of the sub-task on overall completion.
            "position": integer, // The position of the sub task in the parent task.
            "completed": boolean
        }
        ...
    ]
}
```

#### Sub-tasks

`/api/v1/sub-tasks/{id}/`

`GET`

Fetches the relevant information about the given sub task individually. Requires an authorized bearer token to be provided unless the given project is public.

Example Response:

```json
{   
    "message": ...,
    "data": {
        "id": "The sub-task's id",
        "task_id": "The parent task's id",
        "project_id": "The parent project's id",
        "assignee": "Optional - The assigned project member's id",
        "body": "A description of the sub-task",
        "weight": integer, // Optional - the influence of the sub-task on overall completion.
        "position": integer, // The position of the sub task in the parent task.
        "completed": boolean
    }
}
```

`POST`

Edits the given sub-task, requires an authorized bearer token even if the project to be public, and for the related project member to have task management permissions. The below example body contains all editable fields.

Example Body: (All fields are optional)
```json
{
    "assignee": "The assigned project member's id", // Must be a valid project member
    "body": "A description of the sub-task",
    "weight": integer, // Optional - the influence of the sub-task on overall completion.
    "position": integer, // The position of the sub task in the parent task.
    "completed": boolean
}
```

`DELETE`

Removes the given sub task. Requires an authorized bearer token even if the project is public and for the related project member to have task management permissions.

This will also update the location of other sub-tasks to fille any created gaps.

#### File Hosting (If enabled)

UNIMPLEMENTED

`/api/v1/files/{id}`

#### Notifications

(ENDPOINTS) UNIMPLEMENTED

`/api/v1/notifications`