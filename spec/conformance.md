# UAP Conformance

A UAF implementation claims a conformance **level** and a **role set**.

## Levels

| Level | Name | Requirements |
| --- | --- | --- |
| **L0** | Cognition | Run the cognition loop (perceiver → planner → reflector) to an accepted plan and emit a text-only `result`. No generation tools required. |
| **L1** | Single-modality generation | L0 + dispatch `generate` steps to at least one tool producing one non-text modality. |
| **L2** | Any-to-any | L1 + support all four modalities (text, image, audio, video) and run reflective verification during deliberation. |

## Mandatory behavior (all levels)

A conformant participant:

1. **MUST** populate every required envelope field (§3 of the protocol) and emit
   valid `ts` (RFC 3339) and `id` (UUID) values.
2. **MUST** reject any entry whose `uap` MAJOR version differs from its own.
3. **MUST** ignore — not error on — unknown optional envelope fields and unknown
   `body` keys, to preserve forward compatibility.
4. **MUST** treat all inbound `body` content as untrusted and validate it against
   the relevant schema before acting.
5. **MUST NOT** invoke other participants directly; all coordination is via
   workspace entries.
6. **MUST** enforce the mandatory sanity check: no `phase_transition` to
   deliberation without an accepted reflector plan check, and no final `result`
   without a passed reflector output check. A skipped/failed/time-boxed check
   **MUST** be recorded as a signed entry, never silently omitted.

## Kernel (host) additional requirements

A conformant **kernel**:

1. **MUST** assign a strictly increasing `seq` per session and never reorder.
2. **MUST** enforce the phase state machine, including `max_rounds` in cognition.
3. **MUST** capability-gate tool dispatch: only route a `tool_call` to a tool
   that advertised a matching capability via `hello`.
4. **MUST** make the workspace fully replayable (deterministic reconstruction
   from the ordered entry log).

## Test vectors

Conformance fixtures live in [`examples/`](./examples/). An implementation is
expected to consume `examples/session.jsonl` and reproduce the same terminal
`result` given equivalent deterministic tools.
