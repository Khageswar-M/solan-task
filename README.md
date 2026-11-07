# Solana Transfer Extractor

## Project Overview

A Rust-based Substreams module that processes Solana blockchain transactions and extracts transfer instructions.

## Assessment Requirements

- Blockchain: Solana Mainnet
- Start block: `385870151`
- End block: `385870156`
- Output fields:
  - `from`: Sender wallet address (Base58)
  - `to`: Recipient wallet address (Base58)
  - `amount`: Transfer amount

## Technology Stack

- Rust
- Substreams
- Solana
- Protocol Buffers
- Docker
- VS Code Dev Containers
- Pinax Substreams Provider

## Project Structure

- `src/lib.rs` – Rust transfer extraction logic
- `proto/mydata.proto` – Protobuf output definitions
- `substreams.yaml` – Substreams module manifest
- `Cargo.toml` – Rust dependencies
- `Cargo.lock` – Locked dependency versions
- `buf.gen.yaml` – Protobuf code generation configuration
- `solana-transfer-extractor-v0.1.0.spkg` – Compiled Substreams package

## Environment Setup

1. Install Docker Desktop.
2. Install Visual Studio Code.
3. Install the VS Code Dev Containers extension.
4. Open this project in VS Code.
5. Select **Reopen in Container** when prompted.

Rust and the Substreams CLI are configured inside the development container.

## API Configuration

A Pinax API key is required to access the Solana Substreams endpoint.

Set the key inside the development container:

```bash
export SUBSTREAMS_API_KEY="YOUR_PINAX_API_KEY"
```

Do not commit or share your actual API key.

## Build the Package

Run:

```bash
substreams build
```

The command generates:

`solana-transfer-extractor-v0.1.0.spkg`

## Execute the Substreams

Run the module against the six required blocks:

```bash
substreams run \
  -e solana.substreams.pinax.network:443 \
  solana-transfer-extractor-v0.1.0.spkg \
  map_my_data \
  -s 385870151 \
  -t +6
```

## Supported Transfer Instructions

The current implementation handles:

- Native SOL transfers
- SPL Token transfers
- Token-2022 transfers
- Supported transfers in inner instructions (CPI)

Failed transactions are ignored.

For SPL token transfers, wallet owners are resolved using transaction token-balance metadata.

## Output

Each extracted transfer contains the sender address, recipient address, and integer amount.

The amount is represented in the smallest unit of the respective asset:
- SOL: lamports
- SPL tokens: raw token units

## Compiled Package

`solana-transfer-extractor-v0.1.0.spkg`

The examiner can rebuild the package using the source code and the build command above.