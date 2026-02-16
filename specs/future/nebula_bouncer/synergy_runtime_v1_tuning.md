# NB-CX-007 Synergy Runtime v1 Tuning

## Combo Mapping Summary

- Runtime loadout is `Element x Modifier` (4 x 4 = 16 deterministic combos).
- `F6` cycles element order: `Plasma -> Cryo -> Tesla -> Void -> Plasma`.
- `F7` cycles modifier order: `Elasticity -> Splinter -> Mass -> Velocity -> Elasticity`.
- Default neutral combo is `Plasma + Elasticity` (baseline spawn stats).
- Cryo rows apply slow timers (`cryo_slow_factor < 1.0`, timed expiry).
- Void rows apply DOT timers (`void_dot_dps > 0.0`, timed expiry).

## Clamp Policy

- Damage clamp: `4.0 ..= 80.0`
- Speed clamp: `220.0 ..= 1200.0`
- Bounce clamp: `0 ..= 8`
- Radius clamp: `3.0 ..= 14.0`
- Void DOT tick interval: `0.5s` (minimum 1 damage per resolved tick)

## Runtime Notes

- Orb spawn stats are resolved at fire-time from active loadout through the synergy matrix.
- Orb status payload carries Cryo/Void values to collision resolution.
- On enemy hit:
  - base hit damage is applied immediately,
  - Cryo status extends/strengthens slow timer,
  - Void status extends/strengthens DOT timer and DPS.
- Status effects are updated every frame with explicit expiry reset to neutral values.

## Known Follow-Up Risks

- Enemy AI movement is currently minimal, so Cryo slow has limited visible gameplay expression without richer movement behavior.
- DOT currently uses fixed cadence; future balancing may need sub-tick carryover for very short durations.
- The v1 matrix is hand-tuned and should be validated by playtest telemetry before locking progression balance.
