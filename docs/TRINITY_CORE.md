# Trinity Core

## Core

```text
Identity
Execution
Evidence
```

## Identity

Defines the acting context.

```text
who
why
scope
choice
consent
```

## Execution

Defines the work path.

```text
input
action
state
result
```

## Evidence

Defines the proof path.

```text
record
verify
replay
report
```

## Stack mapping

```text
Enterprise Core -> Identity
Runtime Core    -> Execution
Canonical Core  -> Evidence
```

## Contract mapping

```text
Identity Contract  -> should this exist and under what scope?
Execution Contract -> what happened?
Evidence Contract  -> can it be proven?
```

## Build mapping

```text
src/entry.rs      -> Evidence
src/workspace.rs  -> Evidence
src/replay.rs     -> Evidence
src/runtime.rs    -> Execution
src/report.rs     -> Evidence presentation
src/main.rs       -> Touchpoint dispatch
```

## Rule

Expand through adapters and runtime.
Protect evidence at the core.
