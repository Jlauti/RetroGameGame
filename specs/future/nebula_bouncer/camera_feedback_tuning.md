# NB-CX-005 Camera Feedback Tuning Note

## Profile Table

| Profile | Shake Threshold (damage) | Shake Scale | Wall Damage Factor | Shake Cap | Shake Decay | Hit-Stop Threshold (damage) | Hit-Stop Scale | Hit-Stop Min | Hit-Stop Max |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| safe | 14.0 | 0.12 | 0.25 | 10.0 | 13.0 | 16.0 | 0.0030 | 0.03 | 0.10 |
| normal | 11.0 | 0.18 | 0.30 | 16.0 | 10.0 | 12.0 | 0.0045 | 0.04 | 0.16 |
| intense | 8.0 | 0.24 | 0.40 | 24.0 | 7.5 | 8.0 | 0.0060 | 0.05 | 0.22 |

## Intended Gameplay Usage

- `safe`: Accessibility-first mode for motion-sensitive playtests and long sessions.
- `normal`: Default balancing target for general gameplay and routine QA.
- `intense`: High-feedback mode for stress tests and trailer/impact feel checks.

## Runtime Controls

- `F10`: Toggle camera shake ON/OFF.
- `F11`: Cycle profile (`safe -> normal -> intense -> safe`).
- `F9`: Feedback telemetry snapshot (cooldown-limited).

## Residual Risks

- Very high bounce damage values can still hit profile caps quickly, causing sustained strong feedback in `intense`.
- Single-camera assumptions remain in telemetry and shake application.
- Hit-stop and shake both derive from impact damage; future gameplay changes to damage scaling may require retuning these constants.
