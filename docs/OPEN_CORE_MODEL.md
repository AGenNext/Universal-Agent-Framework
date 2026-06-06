# Open Core Model

## Positioning

UAF is an open core project.

The core protocol and runtime stay open so developers, teams, enterprises, and vendors can trust, inspect, extend, and self-host the foundation.

Commercial value comes from enterprise-grade packaging, managed operations, advanced governance, dashboards, connectors, support, and compliance evidence workflows.

## Principle

Open core must not weaken trust.

The open core must be complete enough to run real workloads, emit evidence, replay sessions, and integrate with cloud-native infrastructure.

```text
Open core = usable foundation
Commercial = enterprise acceleration, operations, scale, assurance, and support
```

## Open source core

The open source core SHOULD include:

```text
Universal Agent Protocol
Reference runtime / kernel
Workspace JSONL log
Replay engine
Constraint model
OPA policy hook
Evidence entry model
Basic report generator
CLI
OCI image
Kubernetes Job manifest
Helm chart
Default examples
Conformance tests
```

The open core must be independently useful.

A user should be able to:

```bash
uaf run
uaf replay
uaf report
kubectl apply -f k8s/job.yaml
```

without buying anything.

## Commercial extensions

Commercial extensions MAY include:

```text
Managed UAF Cloud
Enterprise dashboard builder
Advanced report designer
SOC 2 / GDPR / ISO evidence packs
Policy pack marketplace
Enterprise connector packs
SSO / SCIM / directory sync
Multi-tenant admin console
Audit evidence portal
Retention and legal hold workflows
Advanced RBAC / ReBAC templates
Private registry
Hosted replay explorer
Long-term evidence storage
Customer-managed cloud deployment
Priority support
Professional services
Training and certification
```

## Boundary rule

The commercial layer must not own the protocol.

The following MUST remain open:

- UAP envelope
- entry kinds required for MVP
- workspace format
- replay semantics
- constraint model
- policy verdict model
- evidence model
- local CLI
- basic Kubernetes deployment
- basic reports

The following MAY be commercial:

- hosted UX
- managed storage
- advanced dashboard/report templates
- enterprise control mappings
- premium connectors
- enterprise policy bundles
- advanced deployment automation
- operational SLAs

## Why open core fits UAF

UAF is infrastructure. Infrastructure needs trust.

Enterprises will trust the foundation more if the protocol, runtime behavior, evidence format, and replay logic are open and inspectable.

Commercial value should be built around making that foundation easier to adopt, operate, govern, audit, and scale.

## Product packaging

### Community Edition

Open source, self-hosted, developer-first.

Includes:

- CLI
- local runtime
- JSONL workspace
- replay
- basic OPA policy check
- basic evidence entries
- basic reports
- Docker image
- Kubernetes Job
- Helm chart

### Enterprise Edition

Commercial, self-hosted or managed.

Includes:

- web console
- custom dashboards
- report designer
- evidence bundles
- advanced policy packs
- connector packs
- enterprise identity integration
- team/workspace administration
- support and SLAs
- audit-ready evidence workflows

### Managed Cloud

Commercial hosted service.

Includes:

- hosted runtime orchestration
- managed evidence storage
- hosted dashboards
- hosted report exports
- tenant administration
- enterprise support
- optional customer-managed cloud deployment

## Licensing intent

The licensing model should protect the core while allowing adoption.

Recommended direction:

```text
Core: Apache-2.0 or AGPL-3.0 depending on commercial posture
Enterprise extensions: commercial license
Docs/spec: Creative Commons or Apache-2.0
Trademarks: retained by project/company
```

If maximum adoption is the priority, Apache-2.0 is simpler.

If preventing hosted competitors from taking the open core without contributing is the priority, AGPL-3.0 or a source-available license may fit better, but it can reduce enterprise adoption.

## Monetization surfaces

UAF should monetize enterprise trust and operations, not protocol lock-in.

Revenue surfaces:

- managed hosting
- support subscription
- enterprise dashboard/report packs
- compliance evidence packs
- policy packs
- private connectors
- deployment automation
- training
- implementation services
- customer-managed enterprise edition

## Anti-patterns

Do not make these commercial-only:

- replay
- evidence format
- CLI
- protocol schemas
- local policy hook
- basic report export
- Kubernetes Job manifest

If these are closed, trust drops.

## Open core promise

A developer should always be able to run UAF locally, inspect the workspace, replay a session, generate basic evidence, and deploy the runtime to Kubernetes using open components.

A paying enterprise should get speed, assurance, governance, integration, scale, support, and polished reporting — not access to the basic truth layer.