# Canonical Real-World Agents

The canonical catalog is UAF's "standard library" of agents: reference
participants that bind the abstract protocol roles to **real-world tasks**. Each
canonical agent is a conformance-tested implementation that (a) demonstrates a
role or tool, (b) targets a stated conformance level, and (c) runs against
realistic inputs — not toy fixtures.

These exist so that adopters start from working, idiomatic examples rather than
from the bare protocol.

## Cognition agents (role-conditioned MLLM variants)

| Canonical id | Role | Real-world job |
| --- | --- | --- |
| `uaf.perceiver.document` | perceiver | Ground PDFs/scans/screenshots into structured text (layout, tables, entities). |
| `uaf.perceiver.scene` | perceiver | Ground photos/video frames into objects, attributes, and relations. |
| `uaf.planner.taskgraph` | planner | Decompose a goal into a dependency-ordered, tool-annotated plan. |
| `uaf.reflector.verifier` | reflector | Critique plans and verify deliberation outputs against the goal. |

## Deliberation tools (generation / understanding)

| Canonical id | Kind | In → Out |
| --- | --- | --- |
| `uaf.tool.caption` | understand | image/video → text |
| `uaf.tool.image.diffusion` | generate | text(+image) → image |
| `uaf.tool.tts` | generate | text → audio |
| `uaf.tool.video.synth` | generate | text(+image) → video |
| `uaf.tool.asr` | understand | audio → text |

## Composite "real-world" agents

Whole pipelines assembled from the above, each tied to a concrete use case:

| Canonical id | Level | Use case |
| --- | --- | --- |
| `uaf.app.doc-to-brief` | L1 | Read a document, produce a narrated one-page brief (text + audio). |
| `uaf.app.scene-to-card` | L1 | Caption an image and render a matching caption card (the worked example). |
| `uaf.app.story-to-clip` | L2 | Turn a prompt into an illustrated, narrated short video (text+image+audio+video). |

## Catalog contract

Every canonical agent MUST:

1. Declare its `role`/capabilities via a `hello` exactly as specified.
2. State its target **conformance level** and pass that level's test vectors.
3. Be **provider-agnostic**: the MLLM/diffusion backend is injected, never
   hard-wired, so the agent is portable across platforms and vendors.
4. Ship a minimal real-input fixture and an expected terminal `result`.

The catalog is intentionally small and curated — additions go through review so
"canonical" keeps meaning *reference-quality*, not *every possible agent*.
