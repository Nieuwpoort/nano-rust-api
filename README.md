# ifenpay Server API

**A Rust-based Payment Gateway API for Nano Cryptocurrency**

A secure, high-performance REST API for processing Nano payments, managing wallets, and verifying transactions.

## Features

✅ **Fully Implemented:**
- HTTP REST API with Axum framework
- Nano RPC client for communication with Nano node
- Complete block signing with Ed25519 cryptography
- **Send Nano** - Create and sign send blocks
- **Receive Nano** - Receive pending blocks (including account opening)
- Balance checking endpoint
- Transaction history endpoint
- Pending blocks (receivable) endpoint
- Health check endpoint
- **API Key Authentication** - Secure endpoint access
- **Rate Limiting** - 100 requests/minute per API key
- **In-Memory Caching** - Fast responses with 5-10s TTL
- **Webhooks** - Instant payment notifications with retry logic 🆕
- **Metadata** - Custom JSON data support (max 4KB) 🆕
- **PostgreSQL Persistence** - Paid transactions audit trail 🆕
- Structured logging with tracing
- Comprehensive error handling and validation

## Endpoints

### GET /health
Check the status of the server and Nano node connection.

**Response:**
```json
{
  "status": "healthy",
  "nano_node": "connected",
  "timestamp": "2026-11-16T12:00:00Z",
  "version": "0.1.0"
}
```

### GET /balance/:address
Get the balance of a Nano account.

**Authentication:** Required (X-API-Key header)

**Example:**
```bash
curl -H "X-API-Key: ifenpay_demo_key_12345" http://localhost:3000/balance/nano_1abc...
```

**Response:**
```json
{
  "account": "nano_1abc...",
  "balance": "100.000000",
  "balance_raw": "100000000000000000000000000000000",
  "pending": "1.500000",
  "pending_raw": "1500000000000000000000000000000"
}
```

### GET /history/:address
Get the transaction history of an account.

**Authentication:** Required (X-API-Key header)

**Example:**
```bash
curl -H "X-API-Key: ifenpay_demo_key_12345" http://localhost:3000/history/nano_1abc...
```

**Response:**
```json
{
  "account": "nano_1abc...",
  "history": [
    {
      "transaction_type": "receive",
      "account": "nano_1sender...",
      "amount": "10.000000",
      "amount_raw": "10000000000000000000000000000000",
      "hash": "ABC123...",
      "timestamp": "1700000000",
      "height": "5",
      "confirmed": true
    }
  ]
}
```

### GET /pending/:address
Get pending (receivable) blocks for an account.

**Authentication:** Required (X-API-Key header)

**Example:**
```bash
curl -H "X-API-Key: ifenpay_demo_key_12345" http://localhost:3000/pending/nano_1abc...
```

**Response:**
```json
{
  "account": "nano_1abc...",
  "pending_blocks": [
    {
      "hash": "DEF456...",
      "amount": "1.500000",
      "amount_raw": "1500000000000000000000000000000",
      "source": "nano_1sender..."
    }
  ],
  "total_pending": "1.500000",
  "total_pending_raw": "1500000000000000000000000000000"
}
```

### POST /send
Send Nano to another account.

**Authentication:** Required (X-API-Key header)

**Request:**
```json
{
  "from": "nano_1sender...",
  "to": "nano_1receiver...",
  "amount": "1.5",
  "private_key": "your_private_key_hex",
  "representative": "nano_1rep..." // optional
}
```

**Response:**
```json
{
  "success": true,
  "block_hash": "ABC123...",
  "new_balance": "98.500000",
  "new_balance_raw": "98500000000000000000000000000000"
}
```

**Example:**
```bash
curl -X POST -H "X-API-Key: ifenpay_demo_key_12345" \
  -H "Content-Type: application/json" \
  -d '{"from":"nano_1...","to":"nano_3...","amount":"1.5","private_key":"abcd..."}' \
  http://localhost:3000/send
```

### POST /receive
Receive pending Nano blocks.

**Authentication:** Required (X-API-Key header)

**Request:**
```json
{
  "account": "nano_1receiver...",
  "private_key": "your_private_key_hex",
  "representative": "nano_1rep...", // optional
  "max_blocks": 10 // optional, default 10
}
```

**Response:**
```json
{
  "success": true,
  "received_blocks": [
    {
      "block_hash": "DEF456...",
      "amount": "1.500000",
      "amount_raw": "1500000000000000000000000000000",
      "from": "nano_1sender..."
    }
  ],
  "new_balance": "101.500000",
  "new_balance_raw": "101500000000000000000000000000000"
}
```

**Example:**
```bash
curl -X POST -H "X-API-Key: ifenpay_demo_key_12345" \
  -H "Content-Type: application/json" \
  -d '{"account":"nano_1...","private_key":"abcd..."}' \
  http://localhost:3000/receive
```

## Setup

### Requirements
- Rust 1.70+
- Access to a Nano RPC node (local or remote)

### Installation

1. Clone the project:
```bash
git clone <repository>
cd ifenpay_server_api
```

2. Configure Nano node URL (optional):
```bash
# Windows PowerShell
$env:NANO_NODE_URL = "http://localhost:7076"

# Linux/Mac
export NANO_NODE_URL="http://localhost:7076"
```

3. Build the project:
```bash
cargo build --release
```

4. Run the server:
```bash
cargo run --release
```

The server starts on `http://localhost:3000`

## Development

### Project Structure
```
src/
  main.rs           - Main entry point and routing
  mod.rs            - Module exports
  handlers.rs       - HTTP endpoint handlers
  rpc_client.rs     - Nano RPC client
  crypto.rs         - Ed25519 signing and Nano address utilities
  auth.rs           - API key authentication
  rate_limiter.rs   - Rate limiting per API key
  cache.rs          - In-memory caching with Moka
  structs/
    mod.rs          - Struct module exports
    account.rs      - Account-related types
    block.rs        - Block structures
    rpc.rs          - RPC request/response types
    api.rs          - API request/response types
```

### Build and run
```bash
# Development build
cargo build

# Run with logging
RUST_LOG=info cargo run

# Run tests
cargo test

# Check code without building
cargo check
```

## Configuration

Environment variables:
- `NANO_NODE_URL` - URL of the Nano RPC node (default: `http://localhost:7076`)
- `RUST_LOG` - Log level (default: `info`, options: `trace`, `debug`, `info`, `warn`, `error`)

## Authentication

All endpoints except `/health` require API key authentication.

**Header:**
```
X-API-Key: your_api_key_here
```

**Default API Key (for testing):**
```
ifenpay_demo_key_12345
```

In production, generate secure API keys and manage them through environment variables or a configuration file.

## Rate Limiting

Rate limiting is enforced per API key:
- **Limit:** 100 requests per minute
- **Response on exceeded:** HTTP 429 Too Many Requests

## Caching

In-memory caching with automatic TTL:
- **Balance cache:** 5 seconds
- **Account info cache:** 10 seconds
- Cache is automatically invalidated on send/receive operations
- No database required - fast in-memory access

## Nano Node Setup

For local development you need a Nano node. Download and install:
- [Nano Node](https://docs.nano.org/running-a-node/overview/)

Or use a public RPC node (not recommended for production).

## Security Warnings

⚠️ **IMPORTANT:**
- Private keys are sent in API requests - **always use HTTPS in production**
- Never use production private keys in development/testing
- The default demo API key (`ifenpay_demo_key_12345`) should be replaced in production
- Generate strong, unique API keys for each client
- Consider implementing additional security layers (IP whitelisting, 2FA, etc.)
- Store API keys securely (environment variables, secrets manager)
- Rate limiting is enabled (100 req/min per key) but monitor for abuse

## Roadmap

- [x] Basic HTTP server
- [x] Nano RPC client
- [x] Balance endpoint
- [x] History endpoint
- [x] Pending endpoint
- [x] Health check
- [x] Block signing implementation
- [x] Send functionality
- [x] Receive functionality
- [x] API key authentication
- [x] Rate limiting
- [x] In-memory caching
- [ ] Work caching/pre-generation
- [ ] WebSocket support for real-time updates
- [ ] Prometheus metrics
- [ ] Multi-account wallet support
- [ ] Batch operations

## License

MIT

## Contributing

Contributions are welcome! Open an issue or pull request.
