# QingqueSR

MIT licensed smol PS for moc simulation.
raw protobufs are available~

## Screenshots :3
<img src="screenshots/overworld.PNG" alt="overworld" width="360"/>
<img src="screenshots/sp.PNG" alt="infinite-sp" width="360"/>

---

## Project Overview

### Packages

**cmd**  
Program entrypoint.

**config**  
Anything used for the config files.

Honestly, I liked working on this very much because of how composable it was.

Currently there is:
- `hotfix.json` - Hotfix for query_gateway
- `lx.json` - For lua "execution"
- `player.json` - Player data (uid, username, character, etc.)
- `ports.json` - Ports for the servers
- `scene.json` - Map, player position, and calyx position
- `config.json` - [SRTools](https://srtools.pages.dev/) config Popura edition

**gameserver**  
TCP. Basic. Calyx moc simulator. But atleast you can run lua nicely.

Currently there is no ingame commands yet, but I plan to add some later on for modifying the things in player.json.

You can find the handler list in [here](gameserver/handler/handler.go).

**pb**  
Protocol buffers. Feel free to take the raw protobufs from here.

**sdkserver**  
HTTP server.

Nothing special about it. Auth is hardcoded and all that. There is, however, auto hotfix.

Lua "Executor" and the SRTools Manager is also implemented here (mostly because I couldn't find an `egui` in Go).

**setup.go**  
This will automatically compile the protobufs, generate cmd id file, and compile the server.

---

# Tutorial

## Prerequisites

### Building From Source
> You can skip this part if you want to use prebuilt.

- **Go** v1.24.5
- **protoc** v31.1
- **protoc-gen-go** v1.36.6

These versions are pulled out of my ass (aka what I used while developing this). You can probably use older or newer ones.

Not making a detailed tutorial for installing these because you should know how to if you want to build from source.

### Playing With The Server
- Proxy (or a redirect patch, I guess)

#### Proxy Setup (For Windows)
1. Download the `FireflySR.Tool.Proxy` from [here](https://git.xeondev.com/YYHEggEgg/FireflySR.Tool.Proxy/releases/download/v2.0.0/FireflySR.Tool.Proxy_win-x64.zip)
2. No need to configure anything, you can just extract and run
3. When asked about root certificate, let it install
4. If you experience internet connectivity issues after playing, manually disable the proxy in your Windows settings

#### Proxy Setup (For Linux)
Install `mitmproxy` and their certificate. Set HTTP_PROXY and HTTPS_PROXY env var of the game to the `mitmproxy` server. Run it with this script:

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

You can also use process specific capture instead of using env vars, I suppose.

---

## Installation

### Prebuilt
1. Download the `{game_version}.zip` (**NOT THE SOURCE CODE!!**) from [here](https://github.com/yuvlian/qingque-sr/releases)
2. Extract the ZIP file
3. Run the `server` binary (exe, whatever)
4. Make sure the proxy is properly redirecting game traffic to the server
5. Go have fun

### Building From Source
1. Make sure you have the prerequisites
2. Clone this repository
3. Run `go mod tidy`
4. Run `go run setup.go`
5. Run `go run cmd/server/main.go` or `./cmd/server/main.exe` or whatever
6. Make sure the proxy is properly redirecting game traffic to the server
7. Go have fun

---

## Epic Notes Time!!
- **Very important:** Go's json library does NOT validate the jsons. So if your json is fucked up, it won't tell you, but there will be signs.
- You can "execute" lua through `http://127.0.0.1:21000/lua_executor`
- There's also `/srtools_manager`, so you don't have to overwrite your `config.json` manually. 
- Make sure you choose `config.json` from https://srtools.pages.dev/ and not `freesr-data.json` (I am too lazy to implement that one)
- Traces are hardcoded to 6/10/10/10 because not all e3 or e5 is the same

---

## Credits
**Me, Yulian.** I made the code, duh.

**Naruse.** For protos & helping migrate old code.

**Lukopa.** Very fun lua scripts. I'm not gonna share them :3
