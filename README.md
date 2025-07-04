# RustyMQ 🦀⚡  
**A lightweight, blazing-fast, Kubernetes-native message broker written in Rust.**  
Pub/Sub. Wildcards. Prometheus metrics. CRD-ready. Built for performance, simplicity, and cloud-native environments.

---

## 🚀 Features

- 🔁 **Publish/Subscribe** system with wildcard support (`*`, `>`)
- 💨 **High-performance** async engine powered by `tokio`
- 📊 **Prometheus-compatible metrics** endpoint (`/metrics`)
- ♻️ **Kubernetes-native design**: configurable via ENV and CRDs (soon!)
- 🔧 Configurable buffer size, ports, and wildcards support
- 🛡️ Secure and extensible foundation
- 🧪 Unit-tested matcher engine

---

## 🦀 Why Rust?

RustyMQ is designed with:
- **Zero-cost abstractions** and **no GC**
- **Memory safety** without sacrificing speed
- **Minimal binary size** (~<5MB)
- A forward-looking approach for modern infrastructure

---

## ⚙️ Quick Start

### 1. Clone & Run

```bash
git clone https://github.com/youruser/rustymq.git
cd rustymq
cargo run
```

### 2. Optional: env

```env
RUSTYMQ_PORT=8080
RUSTYMQ_BUFFER_SIZE=1000
RUSTYMQ_ENABLE_METRICS=true
RUSTYMQ_ALLOW_WILDCARDS=true
```

### 📦 Configuration (ENV Variables)

| Variable                  | Default | Description                        |
| ------------------------- | ------- | ---------------------------------- |
| `RUSTYMQ_PORT`            | `8080`  | Port for main server               |
| `RUSTYMQ_BUFFER_SIZE`     | `100`   | Max buffer per topic               |
| `RUSTYMQ_ENABLE_METRICS`  | `true`  | Enables `/metrics` endpoint        |
| `RUSTYMQ_ALLOW_WILDCARDS` | `true`  | Enables `*` and `>` topic patterns |
| `RUSTYMQ_DEBUG`           | `false` | Enables debug logging              |

### 🤖 Example: Subscribing to Topics

```rust
let mut sub = broker.subscribe("logs.*");
broker.publish(Message {
    topic: "logs.auth".into(),
    payload: "{\"status\": \"ok\"}".into(),
});
```

### 📈 Metrics

```http
GET /metrics
```

Exposes Prometheus-compatible metrics for monitoring.
- Total messages published
- Active topics
- Subscriptions count

### 🧪 Testing
Run unit tests to ensure matcher engine works as expected:

```bash
cargo test
```

## 📜 License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

