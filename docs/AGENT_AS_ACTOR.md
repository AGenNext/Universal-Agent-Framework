# Agent as an Actor

In UAF an **agent is an actor** — deliberately in *both* senses, and they
reinforce each other:

1. **The actor model** (computation): an isolated unit of state and behavior that
   communicates only by asynchronous messages.
2. **The accountable actor** (social / legal): an identified party that bears
   agency, capabilities, duties, and rights, and is answerable for what it does.

UAF's coordination model already *is* the actor model, and its identity and
governance models already make agents *accountable actors* — this document names
the unification.

## The actor model

| Actor property | How UAF realizes it |
| --- | --- |
| **Isolated state** | each agent owns its private state; there is **no shared mutable memory** — the only shared thing is the append-only workspace, which is *messages*, not state |
| **Message-passing** | agents interact **solely** by authoring and consuming [entries](../spec/PROTOCOL.md); never by direct calls (`coordinate by sharing, not by calling`) |
| **Asynchronous** | entries are appended and reacted to; no synchronous coupling between participants |
| **Location transparency** | an actor is addressed by its [UDCI](./IDENTIFIERS.md) and reached via [uniform access](./ACCESS.md) — in-process, out-of-process, remote, or federated, identically |
| **Supervision** | actors are [lifecycle-managed](./AXIOMS.md) and supervised (the kernel / [K8s operator](./CLOUD_NATIVE.md)); the replayable log makes restart and self-healing safe |
| **Concurrency** | many actors progress independently; the workspace's total order (`seq`) reconciles their messages |

This is why the blackboard model and the actor model are the same thing here: the
workspace is the message bus, entries are the messages, and agents are the actors.

## The accountable actor

An agent is also an actor in the worldly sense — a first-class party in the
[World Model](./WORLD_MODEL.md) and its social-economic fabric:

| Accountability facet | Realized by |
| --- | --- |
| **Identity** | a self-sovereign [UDCI](./IDENTIFIERS.md); no anonymous actors |
| **Agency / capability** | declared capabilities (via `hello`), scoped by trust |
| **Duties & rights** | first-class [governance](./GOVERNANCE.md) bound to the actor's role/identity |
| **Answerability** | every action is a signed entry — attributable, auditable, sanity-checked |
| **Standing** | actors relate, delegate, and transact (the Relation primitive) as organizations and individuals do |

So an agent is not a function call site; it is a **party** that holds capabilities,
owes duties, claims rights, and answers for its actions.

## Agent as a tool — the actor/tool duality

An agent is an actor internally; externally it can present a **tool face**. The
two are dual:

- **Invocable like any tool.** An agent advertises capabilities (`hello`) and is
  called via the [uniform `INVOKE`](./ACCESS.md) / `tool_call` — the caller need
  not know the "tool" is itself a whole multi-agent actor.
- **Recursive composition.** Agents call agents call tools; an entire
  [solution](./AXIOMS.md) composed of many agents is itself invocable as a single
  tool/product. This nesting is unbounded — the engine of
  [composability](./DESIGN_PRINCIPLES.md).
- **Guarantees preserved.** Across the boundary it is a normal gate; inside, the
  sub-agent remains an accountable actor with its own identity, governance, and
  signing. Wrapping an actor as a tool hides its internals, not its accountability.

So *tool* and *agent* are the same kind of thing seen from two sides — which is why
any actor (or solution) can be packaged as a callable product.

## Why both senses matter

- The **computational** actor gives **isolation, concurrency, resilience, and
  location transparency** — the systems properties.
- The **accountable** actor gives **identity, agency, duty, right, and
  answerability** — the governance properties.
- Together: a UAF agent is an **isolated, message-passing, supervised unit of
  computation that is also an identified, capable, accountable party** — which is
  exactly what you need for autonomous agents acting in the real world under
  governance.

## Relationships

| This builds on | In |
| --- | --- |
| message-passing coordination | [Protocol §1](../spec/PROTOCOL.md) |
| identity & answerability | [Identifiers](./IDENTIFIERS.md), [Data Security](./DATA_SECURITY.md) |
| duties & rights | [Governance](./GOVERNANCE.md) |
| supervision & lifecycle | [Cloud-Native](./CLOUD_NATIVE.md), [Axioms](./AXIOMS.md) |
| location-transparent access | [Uniform Access](./ACCESS.md) |

> Status: design direction. "Agent as actor" is a conceptual unification of the
> existing coordination, identity, and governance models — it adds no wire-format
> fields.
