# NB-A2-014 Runtime Notes

Date: 2026-03-08
Ticket: `NB-A2-014`
Mode: `Nebula-first`

## BRP Launch Notes

- Successful Windows BRP launch came from starting the built binary through `cmd.exe` so the environment variables reached the `.exe` process:
  - `RETRO_DEV_BOOT=nebula`
  - `BEVY_BRP_ENABLE=1`
  - `BEVY_BRP_PORT=15702`
- The repo build exposes BRP at `http://127.0.0.1:15702`, but this build does not implement the custom `__brp_probe__` helper. Success was confirmed by live JSON-RPC responses from `world.query` and `world.insert_components`.
- Practical shell note: Git Bash-style inline env assignment did not propagate reliably to the Windows `.exe` in this environment. `cmd.exe` launch did.

## Ticket-Scoped Runtime Validation

### 1. Enemy motion stayed world-relative across player forward/back changes

Validation used BRP `world.insert_components` to force player Y changes equivalent to forward/back throttle shifts, then `world.query` to inspect a live enemy already in the attack loop.

- Enemy entity: `4294964892`
- Baseline:
  - player Y: `-200.0`
  - enemy Y: `81.9651`
  - enemy velocity Y: `3.6065`
  - enemy state: `Firing`
  - enemy locked anchor Y: `90.0`
- Player forced forward:
  - player Y: `100.0`
  - enemy Y: `81.6672`
  - enemy velocity Y: `-1.2008`
  - enemy state: `Firing`
  - enemy locked anchor Y: `90.0`
- Player forced back:
  - player Y: `-350.0`
  - enemy Y: `81.7472`
  - enemy velocity Y: `3.0806`
  - enemy state: `Firing`
  - enemy locked anchor Y: `90.0`

Result: the enemy kept the same captured attack anchor (`90.0`) through the player Y changes instead of retargeting directly to the player’s latest depth.

### 2. Shoulder enemies did not sit beside the player as same-speed traffic

Validation sampled a shoulder enemy while the player remained neutral.

- Enemy entity: `42949672751`
- Start Y: `-553.9701`
- End Y after ~1.1s: `-682.8763`
- Delta Y: `-128.9063`
- End velocity Y: `-110.0000`
- End state: `Firing`

Result: the sampled shoulder enemy continued advancing and transitioned into the attack loop instead of idling beside the player as convoy-style traffic.

### 3. Telegraphing and firing enemies faced the player during attack setup and release

Validation queried live enemy transforms and compared their actual Z yaw against the expected yaw from the enemy-to-player vector.

- Telegraphing sample:
  - enemy entity: `8589932054`
  - actual yaw: `-9.7982069 deg`
  - expected yaw: `-9.7982124 deg`
  - delta: `0.0000055 deg`
- Firing sample:
  - enemy entity: `4294964892`
  - actual yaw: `58.5879951 deg`
  - expected yaw: `58.5879992 deg`
  - delta: `-0.0000041 deg`

Result: both telegraphing and firing samples matched the player-facing bearing within effectively zero visible error.

## BRP/MCP Documentation Request

Principal engineer should update the control-plane docs that agents actually read for Nebula runtime work so this BRP/MCP path is discoverable without trial and error.

Recommended docs to update:

- `AGENTS.md`
- `agents/team/pekka_kone/brief.md`

Requested content:

- the Windows-safe BRP launch pattern
- the expected BRP URL/port
- the fact that `world.query` and `world.insert_components` are the reliable verification methods in this repo
- the note that `__brp_probe__` is not available in this build
- one short Nebula example for querying player/enemy transforms and mutating test state
