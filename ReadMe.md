## Getting Started

First, install the dependencies:

```
cargo
```

Make sure you have created .env and set all environment variables.
You need to create(or use existing) a project on https://console.cloud.google.com/apis/credentials?project={your-project-name}
to have Google envs values.

To start the app locally run:

```
cargo run
```

To have the app in docker continer use docker-compose.

to build the app run:

```
docker-compose build
```

to start the app run:

```
docker-compose up
```