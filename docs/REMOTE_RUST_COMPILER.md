# Remote Rust Compiler (Docker on Windows PC)

This setup runs a Linux Rust compiler service in Docker on this Windows PC so a Linux laptop can offload `cargo` builds over LAN.

## 1. Prepare SSH key auth

On the Linux laptop:

```bash
ssh-keygen -t ed25519 -C "rust-remote-compiler"
```

Copy the laptop public key onto this repo path on the Windows PC:

`docker/rust-remote-compiler/authorized_keys`

You can start from the example file:

`docker/rust-remote-compiler/authorized_keys.example`

## 2. Start the remote compiler container (Windows PC)

From repo root (`C:\Users\jlaut\git\RetroGameGame`):

```powershell
docker compose -f docker/rust-remote-compiler/docker-compose.yml up -d --build
docker compose -f docker/rust-remote-compiler/docker-compose.yml ps
```

Optional: allow inbound LAN traffic on port `2222`:

```powershell
New-NetFirewallRule -DisplayName "Rust Remote Compiler 2222" -Direction Inbound -Action Allow -Protocol TCP -LocalPort 2222
```

## 3. Discover the Windows PC LAN IP

On the Windows PC:

```powershell
Get-NetIPAddress -AddressFamily IPv4 | Where-Object { $_.IPAddress -notlike '169.254*' -and $_.InterfaceAlias -notmatch 'Loopback|vEthernet' } | Select-Object -ExpandProperty IPAddress
```

Pick the LAN IP (example: `192.168.1.50`).

## 4. Validate connection from Linux laptop

```bash
ssh -p 2222 root@192.168.1.50 "rustc --version && cargo --version"
```

If this works, remote compilation is ready.

## 5. Build remotely from Linux laptop

Use the helper script in this repo:

```bash
chmod +x scripts/remote_compiler/remote-cargo.sh
./scripts/remote_compiler/remote-cargo.sh root@192.168.1.50 check
./scripts/remote_compiler/remote-cargo.sh root@192.168.1.50 build --release
```

The script:

- rsyncs your repo to `/work/RetroGameGame` inside the container
- runs `cargo ...` remotely
- syncs remote `target/` back to your local `target/` (default enabled)

Environment flags:

```bash
REMOTE_PORT=2222
REMOTE_PROJECT_DIR=/work/RetroGameGame
SYNC_TARGET_BACK=1
```

## 6. Stop/remove service (Windows PC)

```powershell
docker compose -f docker/rust-remote-compiler/docker-compose.yml down
```

To also remove cached volumes:

```powershell
docker compose -f docker/rust-remote-compiler/docker-compose.yml down -v
```
