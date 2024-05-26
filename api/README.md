## What is my purpose?
You save the results from votes.

### Config

HIERARCHICAL LOADING! In the following order:
toml -> dotenv -> env variables
**No need to use one or the other**

**toml**
```
[server]
address = "0.0.0.0"
port = 8080
resources_path = "client"

[log]
level = "trace"
```

**dotenv/env vars**
```
API_SERVER_ADDRESS="0.0.0.0"
API_SERVER_PORT=8080
API_SERVER_RESOURCES_PATH="client"
API_LOG_LEVEL="trace"
```