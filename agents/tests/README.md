# Policy Test Scenarios

These scenarios map directly to the Nebula Agent Loop v3 requirements.

## Automated Policy Smoke Test

Run:

```bash
bash /home/jl/git/RetroGameGame/agents/scripts/policy_smoke_test.sh
```

It validates:

1. Critical-path ticket in `JULES` lane is rejected.
2. `JULES` lane without eligibility is rejected.
3. One-ticket WIP violation is detected.
4. Local ticket shape passes validation.
5. QA signoff gate blocks non-`PASS` results.

## Release Readiness Tests

Run:

```bash
python3 -m unittest discover -s /home/jl/git/RetroGameGame/agents/tests -p 'test_*.py' -v
```

It validates:

1. Ticket state reconciliation scenarios (`READY_FOR_QA`, `READY_FOR_MERGE`, `MERGED`, `BLOCKED`, `STALE_METADATA`).
2. Release board ordering policy (critical-path first, then oldest completed, then non-critical).
