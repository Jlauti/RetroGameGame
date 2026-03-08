# Release QA Signoff Protocol

QA is release-only by default.

Use this protocol for:

- milestone review
- release candidate validation
- explicit HITL checkpoints requested by the principal engineer

## Default Gates

```powershell
cargo check
cargo fmt -- --check
```

Add a scoped test or run command only when the checkpoint explicitly requires it.

## Required Artifact Contents

Each QA artifact must include:

1. checkpoint or ticket name
2. commands run with exit codes
3. PASS or FAIL verdict
4. integrated-build notes
5. residual risks

## Rule

Do not create routine ticket-level QA artifacts unless QA was explicitly activated for that checkpoint.
