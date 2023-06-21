<div style="text-align: center">

# Nine-Yards

<img style="width: 10em;" src="site/public/svg/base/fern.svg" alt="fern"/>

Nine Yards (As in "going the whole nine yards") is a self hosted, free, and open source team and project management tool. Like Trello or Asana just you dont spend your life savings on the utilities your team wants needs and everything inbetween.

</div>

---

## Features

TODO: Waffle about fancy features with some pretty pictures as example

---

## Getting started with Nine Yards

Nine Yards can be ran in multiple configurations so you'll need to make some choices before getting started. 

If you dont already have an idea of where you can host your own Nine Yards instance look [here](#what-if-i-dont-have-a-server).

#### Picking a database

Nine Yards fully supports both SQLite and Postgres functionality wise you'll see no difference. By default SQLite is used for ease of setup but if you are expecting many concurrent users often, or for any other reason you may want to use postgres instead.

#### Hosting the site

Nine Yards' front end can either be served statically from the backend or can be ran seperately allowing server side rendering to be utilised. If you dont have multiple servers it's likely best to serve the site statically; although, you can run both concurrently and route incoming traffic using software like Nginx as a reverse proxy.

### Using Docker (Reccomended)

TODO

### Building From Source

To build Nine Yards from source you need some tools first. For the backend install [Rust through Rustup](https://www.rust-lang.org/tools/install) and for the front end [Node.js](https://nodejs.org/en) aswell as [pnpm](https://pnpm.io/installation#using-npm) which can be installed using npm which is bundled with Node.js.

#### Getting the source

```bash
# Clone this repository
git clone https://github.com/Rabbitminers/Nine-Yards
cd Nine-Yards
```

#### Building the frontend

```bash
cd site
pnpm i # Install packages for the project

# For seperate backend and frontend 
pnpm nuxt build # Can now be ran like so `node .output/server/index.mjs`

# or

# For static hosting 
pnpm nuxi generate # (build files are moved for you)

# Return to repository root
cd ..
```

#### Building the backend

```bash
cd backend

# For SQLite
cargo build --release --features=sqlite

# or 

# For Postgres
cargo build --release --features=postgres
```
---

## What if I don't have a server?

There are many great options for hosting Nine Yards both cloud and localy. If you are looking for something free Oracle Web Infrastructure's free trial is permanent and more than powerful enough, a full specification list can be found [here](https://www.oracle.com/cloud/free/). 

As well as this they offer splitting your resources between multiple instances which can be useful if you are intending on running Nine Yards split allowing you to take advantage of SSR to improve site performance and allowing for the front end and backend to be scaled seperately if needed with minimal difficulty

---

## Tech Stack

For contributors or anyone interrested in how Nine Yards works

- Frontend - [Nuxt](https://nuxt.com/), [Vue](https://vuejs.org/), [Sass](https://sass-lang.com/), [Pug](https://pugjs.org/api/getting-started.html)
- Backend - [Rust](https://www.rust-lang.org/), [Actix Web](https://actix.rs/), [sqlx](https://github.com/launchbadge/sqlx)
- Database - [Sqlite](https://www.sqlite.org/index.html) / [Postgres]("https://www.postgresql.org/")

---

## Licenses

Nine Yards is free and open source, and is licensed under [GNU GPLv3](./LICENSE) a summary of what this means can be found [here](https://choosealicense.com/licenses/gpl-3.0/).