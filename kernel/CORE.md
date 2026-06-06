# UAF Kernel Core

## Definition

The UAF kernel core is the smallest trusted runtime that makes agent execution accountable.

It is not an agent framework, model runtime, workflow engine, UI, marketplace, registry, or cloud platform.

It owns only:

```text
validate -> append -> order -> gate -> transition -> emit evidence -> replay
```

## Kernel responsibilities

The kernel core MUST:

1. accept UAP entries,
2. validate the entry envelope,
3. assign monotonic `seq`,
4. append entries to the workspace log,
5. preserve append-only ordering,
6. enforce the phase state machine,
7. detect blocking constraints,
8. require policy verdicts before gated actions,
9. emit evidence entries for control-relevant events,
10. replay a workspace log into deterministic session state.

## Kernel non-responsibilities

The kernel core MUST NOT own:

- LLM prompting strategy
- model routing
- tool implementation
- UI/dashboard rendering
- compliance certification
- federation
- marketplace
- identity provider
- payment
- long-term storage product
- Kubernetes controller logic
- business workflow definitions

Those are participants, adapters, extensions, or enterprise layers.

## Core data types

The MVP kernel needs only these internal concepts:

```text
Entry
Workspace
Session
Phase
Constraint
PolicyVerdict
Evidence
ReplayState
KernelError
```

## Entry lifecycle

```text
submitted entry
  -> parse JSON
  -> validate envelope
  -> validate phase allowance
  -> check constraints
  -> require policy verdict if gated
  -> assign seq
  -> append to workspace
  -> emit evidence if needed
  -> return accepted entry
```

## Phase machine

MVP phases:

```text
cognition -> deliberation -> complete
```

Allowed minimum flow:

```text
task
plan
critique(accept)
phase_transition(deliberation)
tool_call
tool_result
result
```

The kernel MUST reject:

- `tool_call` before deliberation,
- `result` before required tool results,
- `phase_transition` to deliberation without accepted critique,
- gated action without `policy_verdict.allow` or valid `approval`,
- entries with duplicate or out-of-order assigned sequence on replay.

## Workspace

The MVP workspace is JSONL.

Each line is one accepted UAP entry.

Rules:

- append-only
- newline-delimited JSON
- ordered by `seq`
- replayable without external database
- portable as an evidence bundle

## Evidence emission

The kernel SHOULD emit `evidence` entries for:

- task accepted
- entry rejected
- constraint registered
- policy verdict observed
- gated action allowed
- gated action denied
- phase transitioned
- result emitted
- replay verified

Evidence entries are part of the workspace, not a side log.

## OPA boundary

The kernel does not embed business policy.

It calls or receives a policy verdict and records it as data:

```text
constraint + target entry + ctx -> policy_verdict
```

The MVP can implement policy in one of two ways:

1. shell out to OPA/Conftest-compatible evaluation, or
2. accept precomputed `policy_verdict` entries.

The kernel decision surface remains:

```text
allow | deny | revise | needs_human
```

## Rust module boundary

Recommended MVP modules:

```text
src/main.rs        CLI entrypoint
src/entry.rs       UAP entry structs and validation
src/workspace.rs   JSONL append/read
src/session.rs     session state and phase machine
src/policy.rs      policy verdict handling
src/evidence.rs    evidence entry helpers
src/replay.rs      deterministic replay
src/error.rs       kernel errors
```

## CLI boundary

Required MVP commands:

```bash
uaf run --task examples/tasks/basic-task.json --workspace .uaf/session.jsonl
uaf validate --workspace .uaf/session.jsonl
uaf replay --workspace .uaf/session.jsonl
uaf report --workspace .uaf/session.jsonl --out reports/out/session.md
```

## Build target

The kernel core is done when this works:

```bash
cargo test
cargo run -- run --task examples/tasks/basic-task.json --workspace .uaf/session.jsonl
cargo run -- replay --workspace .uaf/session.jsonl
```

And replay reconstructs:

```text
session id
phase history
entries total
constraints
policy verdicts
evidence entries
tool calls
result
final state
```

## Core invariant

If it cannot be replayed, it did not happen.

If it was not recorded as an entry, it is not part of UAF.

If the kernel must understand business meaning to proceed, the design is too coupled.