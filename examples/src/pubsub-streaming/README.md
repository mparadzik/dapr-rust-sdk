# Pub/Sub Streaming Example

This example demonstrates Dapr's pub/sub capabilities using **gRPC bidirectional streaming**. Unlike the [standard pub/sub example](../pubsub/), which requires implementing an `AppCallback` gRPC server that Dapr calls back into, the streaming approach lets the subscriber act as a **client** that opens a persistent stream to Dapr — no server needed.

## Comparison

| | Standard Pub/Sub | Streaming Pub/Sub (this example) |
|---|---|---|
| Subscriber role | gRPC server (`AppCallback`) | gRPC client (stream to Dapr) |
| Requires `appPort` | Yes | No |
| Complexity | Must implement server traits | Simple loop over stream |

## Prerequisites

- [Dapr CLI](https://docs.dapr.io/getting-started/install-dapr-cli/)
- [Dapr initialized](https://docs.dapr.io/getting-started/install-dapr-selfhost/)
- Redis running locally (default Dapr installation includes this):
  ```bash
  docker ps  # verify redis is running
  ```

## Running

1. Build the examples:

```bash
cargo build --examples
```

2. Run the multi-app run template:

```bash
dapr run -f .
```

The streaming subscriber will receive and print `Order` events published by the publisher.

3. Stop with `ctrl + c`
