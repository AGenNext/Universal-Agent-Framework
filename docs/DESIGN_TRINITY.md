# Design Trinity

## Core principle

UAF MVP is built on a trinity of design principles:

```text
Edge-native
Runtime core
Kernel-native
```

Together, they define the shape of the system.

```text
Edge-native = where it runs
Runtime core = what it is
Kernel-native = how small and trusted it stays
```

## 1. Edge-native

Edge-native means UAF can run close to the builder, the data, the device, the organization, or the operational boundary.

It should not require a central cloud service to be useful.

MVP implications:

- local-first CLI,
- file-based JSONL workspace,
- offline replay,
- no database required,
- no managed service required,
- OCI-packaged,
- Kubernetes Job optional,
- air-gapped path possible later,
- artifacts portable as bundles.

Edge-native does not mean anti-cloud. It means cloud is optional, not mandatory.

## 2. Runtime core

Runtime core means UAF is the accountable execution substrate.

It accepts intent, records entries, applies constraints, emits evidence, and reconstructs state from replay.

MVP responsibilities:

```text
accept task
validate entry
append workspace
emit evidence
produce result
replay session
generate report
```

The runtime core is not the whole enterprise platform. It is the smallest working loop that everything else can compose around.

## 3. Kernel-native

Kernel-native means the trusted core remains small, deterministic, inspectable, and embeddable.

The kernel does not own business meaning, UI, marketplace, compliance certification, or model intelligence.

It owns only:

```text
validate
append
order
gate
transition
emit evidence
replay
```

The kernel should be small enough to audit and stable enough to embed.

## Trinity relationship

```text
Edge-native gives deployment freedom.
Runtime core gives execution accountability.
Kernel-native gives trust and minimality.
```

Or:

```text
Run anywhere.
Record everything.
Keep the trusted core small.
```

## What this excludes from MVP

The design trinity excludes premature expansion into:

```text
web console
marketplace
federation service
managed cloud
multi-tenant control plane
custom Kubernetes operator
large framework adapters
enterprise dashboard builder
certification system
```

Those can come later as tools, adapters, packs, or commercial layers.

## MVP success test

The MVP respects the trinity when this works:

```bash
uaf run --task examples/tasks/basic-task.json --workspace .uaf/session.jsonl
uaf replay --workspace .uaf/session.jsonl
uaf report --workspace .uaf/session.jsonl --out reports/out/session-summary.md
```

And no cloud service is required.

## Final rule

UAF should be edge-native in deployment, runtime-core in purpose, and kernel-native in trust.

Everything else is a tool around that core.