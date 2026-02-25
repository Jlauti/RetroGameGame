# QA Checklist Template

## Ticket Information
- Ticket ID: {{TICKET_ID}}
- Agent: {{AGENT_ID}}
- Status: PENDING_FINAL_GATE

## Preflight Checks
- [ ] Code compiles (cargo check)
- [ ] Tests pass (cargo test)
- [ ] Code format is correct (cargo fmt -- --check)
- [ ] Required report/deliverable is present and reviewed
- [ ] No regression in core gameplay loops

## Evidence Expectations
- [ ] Log output from successful `cargo test`
- [ ] Screenshot or log trace of the specific feature/fix in action
- [ ] Confirmation of no lint/format errors

## Blockers
- [ ] Blocker: {{BLOCKER_ID}} (Status: {{BLOCKER_STATUS}})

## Signoff
- [ ] QA Gatekeeper PASS/FAIL
- Date: 
- Note: 
