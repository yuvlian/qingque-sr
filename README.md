# QingqueSR - 3.0.0

Ever wanted a private server that is quick to update when the game updates, but doesn't sacrifice too much features?

A server that's modular enough and feature complete-ish for battle simulation?

QingqueSR got you covered.

## Tutorial

This tutorial assumes you have basic knowledge of terminal usage and traffic redirection via a proxy.

### Prerequisites

#### Proxy Setup (For Windows)
- Download the proxy tool from [here](https://git.xeondev.com/YYHEggEgg/FireflySR.Tool.Proxy/releases/download/v2.0.0/FireflySR.Tool.Proxy_win-x64.zip).

- If you experience internet connectivity issues after playing, manually disable the proxy in your Windows settings.

This server will fallback to default config when a file in `_cfg` is invalid or missing.

---

### Installation Options

#### Option 1: Prebuilt (Windows Only)

1. Download the prebuilt version that matches your SR version from [here](https://github.com/f2pqingque/sr/releases).
2. Extract the ZIP file.
3. Edit configuration files in the `_cfg` folder as needed (refer to the README in that folder).
4. Run the following executables:
   - `game-server.exe`
   - `sdk-server.exe`
5. Make sure game traffic is being redirected and have fun.

NOTE: If you want to see the logs when the server panics, run the binary through cmd.

#### Option 2: Build from Source

1. Install the following tools:
   - [Rust](https://www.rust-lang.org/)
   - `protoc` (Protocol Buffers compiler)

2. Clone the repository:
   ```bash
   git clone https://github.com/f2pqingque/sr.git
   ```
3. Place your `proto` and `cmdid` files in the `sr-proto` folder, and adjust the `build.rs` file accordingly.

4. Edit configuration files in the `_cfg` folder as needed (refer to the README in that folder).

5. Build and run the game server:
   ```bash
   cargo run --release --bin game-server
   ```

6. Build and run the SDK server:
   ```bash
   cargo run --release --bin sdk-server
   ```

---

## Credits

- **QingqueSR Developer**: Yulian
- **SRTools Author**: Amizing25
- **Protobufs**: Amizing25 & Lukopa

---
