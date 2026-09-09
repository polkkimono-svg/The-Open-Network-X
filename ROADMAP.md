# ONX Changelog & Development Roadmap

This document tracks what has been built so far and lays out, in priority
order, what should be worked on next. It follows the build-incrementally
sequence from `INSTRUCTIONS.md` §23 and the specification sequence from
`docs/specification/architecture.md`: each protocol layer's specification
should exist (with an ADR recording any interpretation or open question)
before its code is implemented, and code for a layer should not start until
the layers below it are specified.

Anything in the [Up for grabs](#up-for-grabs) checklist is unclaimed and open
to contribute. If you start on one, open a draft PR or issue early so others
don't duplicate the work.

## Changelog

### 2026-09-08

- Added `INSTRUCTIONS.md` and the reference white paper (`whitepaper.md`).
- **ADR-0001** — Preserved the masterchain/workchain/shardchain distinction
  as a required boundary; added `docs/specification/architecture.md`
  (architecture baseline, required boundaries, specification sequence, and
  the ONX-ARCH open-questions list).
- **ADR-0002** — Standardized cryptographic primitives and canonical binary
  serialization (SHA-256, Ed25519, big-endian integers, mandatory
  domain-separation tags). Added `docs/specification/protocol-primitives.md`
  and `docs/specification/data-structures.md` (resolves ONX-ARCH-001 and
  ONX-ARCH-002).
- **ADR-0003** — State model and account lifecycle. Added
  `docs/specification/state-model.md` (resolves ONX-ARCH-003).
- **ADR-0004** — Selected Rust as the implementation language and
  toolchain. Added the Cargo workspace and the `crates/onx-primitives`
  crate, implementing `docs/specification/protocol-primitives.md` in full:
  canonical integer/byte-string encoding, domain-separated SHA-256 hashing,
  and Ed25519 signing/verification, with tests covering the specification's
  §6 test plan (NIST and RFC 8032 vectors, boundary values, and adversarial
  truncated/trailing/non-canonical-input rejection).
- Added `crates/onx-data-structures` implementing `docs/specification/data-structures.md`
  in full: workchain identifiers, account IDs, full addresses, ShardIdent bitwise prefix encoding,
  message structures, and block headers with domain-separated SHA-256 hashing.
- Added `.github/workflows/ci.yml` running `cargo fmt`, `cargo clippy`, `cargo build`,
  and `cargo test` on pull requests.
- Reformatted `README.md` with proper Markdown structure and updated status diagram.
- Added `CONTRIBUTING.md` codifying the spec-before-code workflow, per-layer crate
  structure, malformed-input and domain-separation conventions, ADR expectations, and
  PR checklist this roadmap and `INSTRUCTIONS.md` assume.
- Read `whitepaper.md` in full and expanded the "Up for grabs" checklist below with
  concrete white-paper section references for every still-unwritten spec item,
  added a wholly untracked **Payment channels (TON Payments)** item
  (`whitepaper.md` §5) with a new open question **ONX-ARCH-009**, added a new open
  question **ONX-ARCH-010** for the previously-unquestioned Networking item
  and split its description into ADNL/DHT/overlay sub-components (`whitepaper.md`
  §3.1–§3.3), and added an explicit "Out of scope for now" note for `whitepaper.md`
  §4 (TON Services and Applications).
- Added `crates/onx-state-model` implementing `docs/specification/state-model.md`.
- **ADR-0005** — Transactions, Messaging, Hypercube Routing, and Multi-Currency Model.
  Added `docs/specification/transactions.md` (resolves **ONX-ARCH-004**, introduces
  **ONX-ARCH-011** and **ONX-ARCH-012**).
- **ADR-0006** — Block Validity, Parent References, and Masterchain Coupling.
  Added `docs/specification/blocks.md`, separating structural block validity
  from consensus/reliability (deferred to ONX-ARCH-005) and from split/merge
  trigger conditions (deferred to ONX-ARCH-007), while defining the
  `Masterchain Block Extra` shard-hash commitment structure and the
  split/merge announcement flag bit layout now. Introduces **ONX-ARCH-013**
  (merge blocks need a second parent reference `BlockHeader` doesn't have).
- **ADR-0007** — Execution Model, Resource Accounting, and Merkle-Proof VM
  Primitive Reservation. Added `docs/specification/execution.md`, partially
  resolving **ONX-ARCH-006** and **ONX-ARCH-009**. Scopes "instruction
  semantics" to the execution contract and required semantic categories
  (no bytecode ISA exists to draw from), and **accepts** reserving a
  Merkle-proof/pruned-branch VM primitive now rather than deferring it,
  per the white paper's own warning that VM semantics are hard to retrofit
  post-deployment.

### 2026-09-09

- **ADR-0009** — Peer identity, transport, and reliable datagram transport
  (ADNL and RLDP). Added `docs/specification/networking-adnl.md` (partially
  resolves **ONX-ARCH-010**).
- **ADR-0010** — Distributed Hash Table for peer and service discovery. Added
  `docs/specification/networking-dht.md` (partially resolves **ONX-ARCH-010**).
- **ADR-0011** — Overlay networks and block propagation multicast. Added
  `docs/specification/networking-overlay.md` (completes resolution of
  **ONX-ARCH-010**).
- **ADR-0012** — Dynamic Sharding, shard-tree invariants, split/merge
  lifecycle, state migration, and routing continuity. Added
  `docs/specification/sharding.md` (resolves **ONX-ARCH-007**).
- **ADR-0013** — Economics, Onyx supply, denomination, fee/reward structures,
  staking, and slashing penalties. Added `docs/specification/economics.md`
  (partially resolves **ONX-ARCH-008**).
- **ADR-0014** — Payment Channels and off-chain payment networks. Added
  `docs/specification/payment-channels.md` (partially resolves **ONX-ARCH-009**).
- **ADR-0015** — Project License Proposal. Added
  `docs/decisions/ADR-0015-project-license.md` proposing Apache-2.0 adoption.

### 2026-09-10

- **ADR-0008** — Consensus, Validator Lifecycle, BFT Quorum, and Finality
  Rules. Added `docs/specification/consensus.md` (resolves **ONX-ARCH-005**).

## Up for grabs

Ordered by priority — earlier items unblock more of what follows. Items marked
**(whitepaper.md §N)** cite the white-paper section that most directly informs the
task; per `INSTRUCTIONS.md` §1 and CONTRIBUTING.md, none of the numeric values
or mechanisms cited from `whitepaper.md` become ONX rules until a separate ONX
specification or ADR says so — they're the starting research material, not
the answer.

### Now (unblocks the most)

- [x] **Implement `docs/specification/state-model.md` in code.**
      A crate for the account state record layout, cell binary serialization,
      domain-separated cell hashing, and Merkle proof structures. Depends on
      `onx-primitives` and `onx-data-structures`.
- [x] **Write the Transactions and Messages specification**
      (`docs/specification/transactions.md` + ADR-0005). Resolves **ONX-ARCH-004**
      (cross-shard message order, replay, and failure semantics).
      - Message value model as `(currency_id, value)` pairs (§2.4.5) integrated
        with `extra_currencies`, closing the gap flagged in the PR #8 review
        where `data-structures.md` listed this field but `crates/onx-data-structures`'
        `Message` type didn't implement it — **the crate itself still needs updating
        to match; see "Next" below.**
      - External messages ("messages from nowhere", §2.4.6) and tentative execution
        admission rules.
      - Output-queue-only model (§2.4.16–§2.4.17) and per-account FIFO delivery order.
      - Hypercube routing ("slow path", §2.4.19) adoption and fast path deferral (**ONX-ARCH-011**).
      - Double-delivery prevention via tracking processed message hashes (§2.4.23).

### Next

- [ ] Implement the transactions/messages layer in code now that
      `docs/specification/transactions.md` (ADR-0005) exists. Includes updating
      `crates/onx-data-structures`' `Message` type to add the `extra_currencies`
      field the spec now formalizes (§4.1) — its wire layout currently omits it.
- [x] **Write the Blocks and masterchain coupling specification**
      (`docs/specification/blocks.md` + ADR-0006) — architecture sequence
      item 5. Separates structural block validity (deterministic, defined
      now) from BFT/consensus validity and reliability (deferred to the
      future Consensus spec, ONX-ARCH-005). Defines the `Masterchain Block
      Extra` shard-hash commitment structure, and assigns the four
      split/merge announcement flags (`SPLIT_PREPARE`/`SPLIT_COMMIT`/
      `MERGE_PREPARE`/`MERGE_COMMIT`) bit positions within `BlockHeader.flags`
      — but explicitly defers their load-based trigger conditions and
      validator task-group reassignment to the future Dynamic Sharding spec
      (ONX-ARCH-007), so neither spec silently assumes the other owns them.
      Surfaced a real gap as **ONX-ARCH-013**: `data-structures.md`'s
      `BlockHeader` has only one `prev_ref_hash` field, but a merge block
      needs two parent references — not resolved here, tracked as an open
      question requiring its own `data-structures.md` amendment.
- [ ] Implement `docs/specification/blocks.md` in code once ONX-ARCH-013
      (merge-block parent references) is resolved — the non-split/non-merge
      structural validity rules could reasonably be implemented sooner,
      since they don't depend on that open question.
- [x] **Write the Execution (virtual machine) specification**
      (`docs/specification/execution.md` + ADR-0007) — architecture sequence
      item 6. Resolves part of **ONX-ARCH-006** (VM rules per workchain).
      Defines the execution contract (inputs/outputs/gas/exceptions),
      required arithmetic/data semantic categories, and a closed five-member
      exception set — but explicitly defers concrete opcode-level
      instruction encoding and gas pricing to a future "TVM Instruction Set"
      artifact, since no bytecode-level ISA exists in the reference material
      to draw from.
      **Decision recorded: accept** the Merkle-proof/pruned-branch VM
      primitive now (§3.5) — assigns a meaning to `state-model.md`'s
      previously-undefined `Cell.is_special_flag` bit and reserves an
      `AbsentNode` exception, unblocking **ONX-ARCH-009** (Payment channels)
      from the Execution side without committing to payment-channel
      semantics themselves.
- [ ] Write the "TVM Instruction Set" specification (concrete opcodes and
      gas price table) that `docs/specification/execution.md` §3.1 defers to
      — required before Execution can be implemented in code.

### Later (depend on consensus existing)

- [x] **Write the Consensus and validator operation specification**
      (`docs/specification/consensus.md` + ADR-0008) — architecture sequence
      item 7. Resolves **ONX-ARCH-005** (validator lifecycle, stake-weighted election,
      nominators and fishermen roles, rotating shard task groups, block candidate
      propagation, BFT quorum thresholds, signature depth/decay, relative vs. recursive
      block reliability, and explicit finality window).
- [ ] Implement `docs/specification/consensus.md` in code.
- [x] **Write the Networking specifications**
      (`docs/specification/networking-adnl.md`, `networking-dht.md`,
      `networking-overlay.md` + ADR-0009–ADR-0011) — architecture sequence item 8.
      Resolves **ONX-ARCH-010** across three sub-layer specifications: ADNL peer identity
      and RLDP transport, DHT peer/service discovery, and per-shard overlay propagation.
- [ ] Implement the Networking specifications in code.
- [x] **Write the Dynamic sharding specification**
      (`docs/specification/sharding.md` + ADR-0012) — architecture sequence item 9.
      Resolves **ONX-ARCH-007** and the remainder of **ONX-ARCH-006** (shard-tree invariants,
      split/merge announcements, validator task-group drift bounds, and load-driven triggers).
- [ ] Implement `docs/specification/sharding.md` in code.
- [x] **Write the Economics specification**
      (`docs/specification/economics.md` + ADR-0013) — architecture sequence item 10.
      Resolves **ONX-ARCH-008** (Onyx nano-denomination, fee schedule shape, reward rules,
      and parameter deferral).
- [ ] Implement `docs/specification/economics.md` in code.
- [x] **Write the Payment channels specification**
      (`docs/specification/payment-channels.md` + ADR-0014) — resolves **ONX-ARCH-009**
      (point-to-point channels, asynchronous two-workchain channels, conditional promises,
      and multi-hop network mechanics).
- [ ] Implement `docs/specification/payment-channels.md` in code once the execution engine's
      reserved Merkle-proof VM primitive is concretely implemented by the instruction-set artifact.

### Out of scope for now

Not tasks — an explicit note, per `INSTRUCTIONS.md` §22–23 ("build
incrementally," "do not optimize prematurely"), that **whitepaper.md §4 (TON
Services and Applications)** — TON DNS, TON Storage as a general file-hosting
service, TON Proxy, and ton-services/ton-sites/ton-browser — describes an
application/ecosystem layer built on top of the blockchain, network, and
payments layers above. None of it should be started before Consensus,
Networking, Sharding, Payments, and Economics exist; adding it earlier would
be designing for requirements this project isn't at yet. This is distinct
from the archival storage of blocks and state already implied by the
existing Networking item (architecture.md's "Networking and storage" domain)
— that's node-operational storage, not the TON Storage *service*.

### Project infrastructure (can be picked up any time, independent of the above)

- [x] ~~Set up CI (`cargo build`, `cargo test`, `cargo clippy`, `cargo fmt --check`)~~ —
      done in `.github/workflows/ci.yml` targeting `main`.
- [ ] Decide and record a license (`README.md` currently says this is
      still pending — see [License](README.md#license)).
- [x] ~~Add a `CONTRIBUTING.md` codifying the workflow this document assumes~~ — done,
      see [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Notes on prioritization

- **Spec before code, always.** Per `INSTRUCTIONS.md` §1 and §23, a layer's
  specification (and ADR, where an interpretation was required) must exist
  before its implementation is started. The ordering above reflects that:
  data-structures and state-model already have specs and are therefore ready
  for code now, while transactions/blocks/execution/consensus/networking/
  sharding/economics need their specs written first.
- **Consensus-critical layers are sequenced by dependency, not by
  difficulty.** Execution depends on state and transactions being defined;
  consensus depends on execution's resource accounting; networking is
  deliberately kept separate from consensus validity per the architecture
  baseline's required boundaries; payment channels depend on execution
  supporting Merkle-proof verification (whitepaper.md §5.1.9); economics is last
  because it depends on the resource-accounting and consensus model being
  settled (`INSTRUCTIONS.md` §18).
- This ordering is a recommendation, not a rule enforced anywhere in code.
  If you have a good reason to reorder something, open an issue or PR
  discussing it — per project philosophy, deviations should be explicit and
  documented, not silent.
