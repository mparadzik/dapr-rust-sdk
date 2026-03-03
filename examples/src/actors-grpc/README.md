# Combined Actor + gRPC Server Example

This example demonstrates running both a Dapr actor HTTP server and a gRPC service on the **same port** in a single binary. The gRPC service is invocable via Dapr gRPC proxying.

The key technique is to extract the axum `Router` from `DaprHttpServer::build_router()`, convert it to tonic `Routes`, add the gRPC service, and serve with tonic's `Server` which natively handles both HTTP/1.1 (for actor callbacks) and HTTP/2 (for gRPC).

## Prerequisites

- [Dapr CLI](https://docs.dapr.io/getting-started/install-dapr-cli/)
- Redis running locally (for actor state store):
  ```bash
  docker run -d -p 6379:6379 redis
  ```

## Running

1. Build the examples:

<!-- STEP
name: Build
background: false
sleep: 30
timeout: 60
-->

```bash
cargo build --examples
```

<!-- END_STEP -->

2. Run with Dapr multi-app:

<!-- STEP
name: Run Multi-App
output_match_mode: substring
match_order: none
expected_stdout_lines:
  - '== APP - actors-grpc-server == Combined actor + gRPC server listening on:'
  - '== APP - actors-grpc-server == doing stuff with test'
  - '== APP - actors-grpc-client == Actor response: Ok('
  - '== APP - actors-grpc-client == gRPC response: HelloReply'
background: true
sleep: 30
timeout_seconds: 30
-->

```bash
dapr run -f .
```

<!-- END_STEP -->

### What the multi-run step effectively runs:

1. Start the combined actor + gRPC server (h2c protocol on port 50051):
```bash
dapr run --app-id actors-grpc-server --app-protocol h2c --app-port 50051 -- cargo run --example actors-grpc-server
```

2. Start the client that tests both services:
```bash
dapr run --app-id actors-grpc-client --dapr-grpc-port 3502 -- cargo run --example actors-grpc-client
```
