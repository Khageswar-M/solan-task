## Environment Setup and Execution from Scratch

### Prerequisites

Install the following software on your host machine:

1. Docker Desktop
2. Visual Studio Code
3. VS Code Dev Containers extension

No local Rust installation or Substreams CLI installation is required. These tools are configured inside the Docker development container.

### Step 1: Extract the Project

Download and extract `solana-transfer-extractor.zip`.

Open the extracted `solana_transfer_extractor` folder in VS Code.

### Step 2: Start the Development Container

Make sure Docker Desktop is running.

In VS Code:

1. Open the Command Palette (`Ctrl + Shift + P`).
2. Select **Dev Containers: Reopen in Container**.
3. Wait for Docker to build the development environment and initialize the container.

The development environment installs and configures Rust, the WebAssembly target, Buf, and the Substreams CLI.

### Step 3: Verify the Environment

Open the VS Code integrated terminal and run:

```bash
rustc --version
cargo --version
substreams --version
buf --version
```

### Step 4: Configure the Pinax API Key

Set your Pinax API key inside the container:

```bash
export SUBSTREAMS_API_KEY="YOUR_PINAX_API_KEY"
```

You must set this again when opening a new terminal session unless you configure it through another secure environment-variable mechanism.

### Step 5: Build the Substreams Package

From the project root, execute:

```bash
substreams build
```

This compiles the Rust module and generates the SPKG package.

### Step 6: Execute the Substreams

Run:

```bash
substreams run \
  -e solana.substreams.pinax.network:443 \
  solana-transfer-extractor-v0.1.0.spkg \
  map_my_data \
  -s 385870151 \
  -t +6
```

The command processes blocks 385870151 through 385870156.

The extracted transfer records are displayed in the terminal.

### Step 7: Verify the Output

Check that each extracted transfer contains:

- `from`: Sender wallet address
- `to`: Recipient wallet address
- `amount`: Transfer amount in the smallest unit of the asset

Multiple transfers are represented as separate transfer records.
