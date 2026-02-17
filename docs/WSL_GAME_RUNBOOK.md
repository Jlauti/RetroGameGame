# RetroGameGame WSL Runbook

This runbook covers:
- Pulling the latest branch
- Building/running from WSL
- Required WSL/Linux setup
- WSL-specific tweaks used by this project

## 1. One-Time WSL Setup

Run these once in your Ubuntu WSL distro.

### 1.1 Install Rust toolchain

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/rustup-init.sh
sh /tmp/rustup-init.sh -y --profile default
. "$HOME/.cargo/env"
rustup default stable
```

### 1.2 Install native Linux dependencies (Bevy/Winit/Audio/X11/Wayland)

```bash
sudo apt-get update
sudo apt-get install -y \
  pkg-config lld clang \
  libasound2-dev libudev-dev \
  libwayland-dev libxkbcommon-dev libxkbcommon-x11-0 \
  libx11-dev libxcursor-dev libxi-dev libxrandr-dev libxinerama-dev \
  libgl1-mesa-dev
```

### 1.3 Verify WSL GUI env

```bash
echo "DISPLAY=$DISPLAY"
echo "WAYLAND_DISPLAY=$WAYLAND_DISPLAY"
```

## 2. Daily Pull + Launch Routine

From your project directory:

```bash
cd /mnt/c/Users/jlaut/git/RetroGameGame__codex-integration-hitl-20260216
. "$HOME/.cargo/env"
```

### 2.1 Pull latest branch

```bash
git fetch --all --prune
git checkout <branch-name>
git pull --ff-only
```

### 2.2 Build

```bash
cargo check --bin retro-game-game
```

### 2.3 Run

```bash
cargo run --bin retro-game-game
```

or run the executable directly:

```bash
./target/debug/retro-game-game
```

## 3. WSL-Specific Runtime Fallbacks

If you see Wayland socket errors like `Io error: Broken pipe` / `Connection reset by peer`, force X11 for that run:

```bash
WAYLAND_DISPLAY= WINIT_UNIX_BACKEND=x11 ./target/debug/retro-game-game
```

If you use `cargo run`:

```bash
WAYLAND_DISPLAY= WINIT_UNIX_BACKEND=x11 cargo run --bin retro-game-game
```

## 4. WSL-Specific Project Tweaks Already Applied

These changes are in this repo state:

- UI background references now use `assets/ui/main_menu_bg_v2.png`:
  - `src/ui/menu.rs`
  - `src/ui/timeline.rs`
  - `src/ui/era_select.rs`
- Added missing thumbnail placeholder:
  - `assets/ui/thumbnails/nebula_bouncer.png`
- WSL startup backend hint in app entrypoint:
  - `src/main.rs`
  - Sets `WINIT_UNIX_BACKEND=x11` and sets `WAYLAND_DISPLAY` to an empty value when running in WSL and backend is not explicitly set.

## 5. Performance Notes (WSL)

- WSL CPU/memory limits come from `C:\Users\<you>\.wslconfig`.
- If compile feels slow, increase `processors` and memory there, then run:

```powershell
wsl --shutdown
```

- For best compile speed, keeping repos under Linux filesystem (`~/...`) is usually faster than `/mnt/c/...`.

## 6. Crash Debug Capture (Optional)

If gameplay crashes, capture logs/backtrace for later debugging:

```bash
RUST_BACKTRACE=1 RUST_LOG=info cargo run --bin retro-game-game 2>&1 | tee /tmp/retrogame_crash.log
```
