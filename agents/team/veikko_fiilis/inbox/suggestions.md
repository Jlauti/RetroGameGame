# Suggestions Inbox

Append your suggestions below after each completed task. The principal engineer will review and curate.

## Format

```markdown
### Suggestion — YYYY-MM-DD
- **Type**: memory | mandate | observation
- **Summary**: <one-line summary>
- **Detail**: <what you learned and why it matters>
- **Proposed Action**: <what should be added to memory.md or team policy>
```

---

<!-- Append suggestions below this line -->

### Suggestion — 2026-02-25
- **Type**: observation
- **Summary**: Bevy 0.18 Event API and serialization mismatches.
- **Detail**: `EventReader` and `add_event` are no longer in the standard prelude/App in 0.18. Also, `ron` was not in the dependencies. Switched to `Res::is_changed()` and `serde_json`.
- **Proposed Action**: Update migration notes to recommend change detection for settings over custom events to reduce API friction. Add `ron` to `Cargo.toml` if it's the preferred format.

