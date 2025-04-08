# qingque-sr

A minimal private server for simulating Memory of Chaos through Calyx.

## Project Overview
### License
MIT

### Crates
- **configs:** Utility for parsing config files
- **game-server, sdk-server, sr-proto:** Should be obvious
- **srtools-manager:** An app to help with managing config.jsons(s) from https://srtools.pages.dev/
- **uni-server:** game-server & sdk-server as one binary

## Tutorial

This tutorial assumes you have basic knowledge of a terminal and proxies.

### Prerequisites

#### Proxy Setup (For Windows)
- Download the `FireflySR.Tool.Proxy` from [here](https://git.xeondev.com/YYHEggEgg/FireflySR.Tool.Proxy/releases/download/v2.0.0/FireflySR.Tool.Proxy_win-x64.zip). No need to configure anything, you can just extract and run. When asked about root certificate, let it install.

- If you experience internet connectivity issues after playing, manually disable the proxy in your Windows settings.

#### Proxy Setup (For Linux)
- Install `mitmproxy` and their certificate. Set HTTP_PROXY and HTTPS_PROXY env var of the game to the `mitmproxy` server. Run it with this script:

```python
from mitmproxy import http

def request(flow: http.HTTPFlow) -> None:
    target_hosts = [
        ".hoyoverse.com",
        ".mihoyo.com",
        ".bhsr.com",
        ".starrails.com",
        ".mob.com",
        ".hyg.com"
    ]

    if any(flow.request.pretty_host.endswith(host) for host in target_hosts):
        flow.request.scheme = "http"
        flow.request.host = "127.0.0.1"
        flow.request.port = 21000
```

---

### Installation Options

#### Option 1: Prebuilt (Linux/Windows)

1. Download the prebuilt version that matches your SR version from [here](https://github.com/yuvlian/qingque-sr/releases).

2. Extract the ZIP file.

3. Edit configuration files in the `_configs_` folder as needed (refer to the README in that folder). The server will fallback to default config when a file in `_configs_` is invalid or missing.

4. Run `uni-server`

5. Make sure game traffic is being redirected by your proxy, launch the game and finally, have fun.

- If you want to see the logs when the server panics, run the binary through terminal.

- You can use `srtools-manager` to easily manage the config.json(s) from https://srtools.pages.dev/

#### Option 2: Build from Source

1. Install the following tools:
   - [Rust](https://www.rust-lang.org/)
   - `protoc` (Protocol Buffers compiler)

2. Clone the repository:
   ```bash
   git clone --recursive https://github.com/yuvlian/qingque-sr
   ```
3. cd into `qingque-sr`

4. Edit configuration files in the `_configs_` folder as needed (refer to the README in that folder). You can do this after compiling, doesn't matter.

6. Build and run the server:
   ```bash
   cargo run --release --bin game-server
   ```
   ```bash
   cargo run --release --bin sdk-server
   ```
   Or if you prefer a single binary,
   ```bash
   cargo run --release --bin uni-server
   ```

7. Build and run SRTools Manager (optional):
   ```bash
   cargo run --release --bin srtools-manager
   ```

8. Make sure game traffic is being redirected by your proxy, launch the game and finally, have fun.
