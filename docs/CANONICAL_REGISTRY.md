# Canonical Registry

## Purpose

Everything important in UAF should be canonical, but not everything belongs in the canonical core.

Canonical means:

```text
named
versioned
owned
documented
bounded
replayable or evidence-linked
compatible with the contracts
```

## Rule

```text
Everything canonical.
Only unavoidable truth primitives in the core.
```

## Canonical layers

| Layer | Canonical things | Stability |
| --- | --- | --- |
| Core | entry, workspace, evidence, replay, outcome summary | highest |
| Runtime | task, report, simulation, policy hook | high |
| Enterprise | identity, consent, intent, choice, lifecycle, audit | high/contextual |
| Adapter | tool, model, API, Kubernetes, OPA, connector | medium |
| Ecosystem | industry pack, skill pack, course, marketplace artifact | variable |

## Canonical record shape

Every canonical concept SHOULD declare:

```text
id
name
layer
version
owner
status
purpose
inputs
outputs
evidence
compatibility
retirement_path
```

## Promotion rule

A concept can move inward only when it becomes more universal and stable.

```text
ecosystem -> adapter -> enterprise -> runtime -> core
```

Most concepts should never reach core.

## Core admission test

A concept may enter the canonical core only if removing it breaks replayable truth.

Questions:

1. Is it required to know what happened?
2. Is it required to replay evidence?
3. Is it independent of industry, vendor, model, and interface?
4. Can it remain backward-compatible?
5. Does it avoid network, secrets, and external service dependency?

If not, keep it outside core.

## Canonical examples

### Core

```text
uaf.entry.v0.1
uaf.workspace.jsonl.v0.1
uaf.evidence.v0.1
uaf.replay.v0.1
```

### Runtime

```text
uaf.task.v0.1
uaf.report.session_summary.v0.1
uaf.simulation.v0.1
```

### Enterprise

```text
uaf.consent.v0.1
uaf.choice.v0.1
uaf.audit_bundle.v0.1
uaf.lifecycle.v0.1
```

### Adapter

```text
uaf.adapter.opa.v0.1
uaf.adapter.kubernetes_job.v0.1
uaf.adapter.oci.v0.1
```

### Ecosystem

```text
uaf.skill.policy_author.v0.1
uaf.industry.manufacturing.v0.1
uaf.course.edge_runtime_101.v0.1
```

## Compatibility rule

Canonical records can evolve, but must not silently break older artifacts.

Prefer:

```text
new optional fields
new versions
migration notes
compatibility statements
```

Avoid:

```text
silent semantic changes
unversioned renames
breaking replay
breaking evidence meaning
```

## Final rule

Canonical does not mean centralized or frozen.

Canonical means named, governed, versioned, and compatible.

The core stays small. The world around it becomes legible.