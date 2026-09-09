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
  **Accepted 2026-09-10; see that day's entry below.**

### 2026-09-10

- **ADR-0008** — Consensus, Validator Lifecycle, BFT Quorum, and Finality
  Rules. Added `docs/specification/consensus.md` (resolves **ONX-ARCH-005**).
- Enforced `data-structures.md` §5 rule 3 ("Address Out of Range") in
  `crates/onx-data-structures` (PR #57): added `Message::validate_against_shard`
  (and its `validate_source_shard`/`validate_dest_shard` helpers), which call
  the existing `ShardIdent::contains_account` check and raise
  `DataStructureError::AddressShardMismatch` on a mismatch, dispatching on
  `msg_type` — previously declared but never constructed.
- Enforced `state-model.md` §5 rule 3 ("Balance Underflow") in
  `crates/onx-state-model` (PR #54): added
  `AccountState::validate_transition_with_delta`, which applies a balance
  delta and raises `StateModelError::BalanceUnderflow` on a negative result —
  previously declared but never constructed.
- Resolved the divergent `mlc_config.json` copies: deleted the unused root
  copy, whose blanket `["^https?://"]` ignore pattern would have silently
  disabled the entire markdown-link-check job had anything pointed at it.
  `.github/workflows/docs-lint.yml` already pointed at the narrower,
  domain-specific `.github/workflows/mlc_config.json`, which remains
  canonical; no `ci.yml` change was needed, since it never referenced
  `mlc_config.json`.
- **ADR-0015 accepted** (Apache-2.0): added the canonical license text as
  `LICENSE`, set `license = "Apache-2.0"` in every crate's `Cargo.toml`
  (previously the unaccepted placeholder `"MIT"`), and updated README's
  License section to point at the accepted ADR instead of "pending review."
- Added `crates/onx-transactions` implementing `docs/specification/transactions.md`
  (ADR-0005): external/internal/external-outbound message admission rules
  (including tentative execution of External Inbound messages under
  `MAX_TENTATIVE_GAS`), `extra_currencies` sort/uniqueness validation, the
  output-queue-only delivery model with per-`(src, dest)` FIFO ordering
  (`OutputQueue`), and double-delivery prevention via `processed_msg_hashes`
  (`ProcessedMessageTracker`). Also corrected this roadmap's stale claim that
  `crates/onx-data-structures`' `Message` type still needed an `extra_currencies`
  field — it already has one; the actual gap was the semantic validation
  rules the `TODO(ONX-ARCH-004)` comment above it flagged, which this crate
  now implements. Hypercube routing (§3.4) is not implemented here and
  remains open work, including the fast-path deferral tracked as
  **ONX-ARCH-011**.
- **ADR-0016** — Merge Block Second Parent Reference. Resolves **ONX-ARCH-013**:
  amends `docs/specification/data-structures.md` §4.4 to add a fixed
  `prev_ref_hash_2` field and a `MERGE_RESULT` flag bit to `BlockHeader`,
  rather than a variable-length trailer, so a merge block can carry both of
  its two parents' hashes while `BlockHeader` stays a single fixed-length
  structure. Also updates `docs/specification/blocks.md` §3.2/§3.4/§4.2 and
  `docs/specification/architecture.md`'s ONX-ARCH-013 row to reflect the
  resolution. Specification and ADR only — `crates/onx-data-structures`'
  `BlockHeader` implementation still needs updating to the amended 242-byte
  layout before block-validity code can depend on it (tracked below).
- Added `crates/onx-blocks` implementing `docs/specification/blocks.md` for
  the non-split/non-merge case: ordinary-successor structural validity,
  masterchain coupling/canonicality with the `Masterchain Block Extra`
  structure, and the split/merge flag bit layout's own well-formedness
  rules. Merge-block validation is not implemented, since it depends on the
  `BlockHeader` code update ADR-0016 (above) still requires.
- Fixed `docs/whitepaper-page-header-review.md`'s false "no open cases
  remain" claim: one injected page header (page 20, inside §2.3.6 "Hashmap
  type") was still unresolved, sitting where a dropped display equation
  belonged. Verified the original wording against the source PDF
  (`https://ton.org/whitepaper.pdf`) and restored the same three-blank-line
  gap convention `whitepaper.md` uses for every other equation this
  PDF-to-Markdown conversion drops. `scripts/fix_whitepaper_page_headers.py`
  now reports 0 remaining cases.
- **ADR-0017** — TVM Instruction Set (Basic Workchain). Added
  `docs/specification/tvm-instruction-set.md`, the concrete-opcode/gas-price
  artifact `docs/specification/execution.md` §3.1 deferred to, further
  resolving **ONX-ARCH-006**. A 46-opcode stack machine over five value
  kinds (`Integer`/`Bytes`/`Cell`/`Slice`/`Builder`), with all
  bytecode-well-formedness faults mapped onto the existing closed
  `ExceptionKind` set (as `MalformedCell`) rather than adding a sixth kind,
  and the `AbsentNode` pruned-branch boundary drawn at content access
  (`CTOS`) rather than reference-passing or hashing. Unblocks implementing
  `onx-execution` in code.
- Updated `README.md`'s status diagram and repository-structure tree, which
  had fallen behind this session's own crate additions (`onx-transactions`,
  `onx-blocks`) and the ADR-0015 license resolution — the diagram still
  colored the transactions/blocks nodes `specOnly` and the file tree and
  license-pending checklist item hadn't been touched since those changes.

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
        `Message` type didn't implement it — **the field itself was added
        separately; see "Next" below for the semantic validation rules
        that were still missing.**
      - External messages ("messages from nowhere", §2.4.6) and tentative execution
        admission rules.
      - Output-queue-only model (§2.4.16–§2.4.17) and per-account FIFO delivery order.
      - Hypercube routing ("slow path", §2.4.19) adoption and fast path deferral (**ONX-ARCH-011**).
      - Double-delivery prevention via tracking processed message hashes (§2.4.23).

### Next

- [x] **Implement the transactions/messages layer in code**
      (`crates/onx-transactions`, per `docs/specification/transactions.md` and
      ADR-0005). Note: `crates/onx-data-structures`' `Message` type already had
      the `extra_currencies` field before this work started — the actual gap
      was the semantic validation rules (admission, sorting, ordering,
      double-delivery) the `TODO(ONX-ARCH-004)` comment above it flagged,
      which are what this crate implements. Hypercube routing (§3.4) is not
      covered and remains open, including the **ONX-ARCH-011** fast-path
      deferral.
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
      question requiring its own `data-structures.md` amendment. **Resolved
      2026-09-10 by ADR-0016; see that day's entry below.**
- [x] **Implement `docs/specification/blocks.md` in code (non-split/non-merge case).**
      Added `crates/onx-blocks`: ordinary-successor structural validity
      (§3.1, §5 rules 1-6), masterchain coupling/canonicality and the
      `Masterchain Block Extra` shard-configuration commitment (§3.3, §4.1,
      §5 rules 7-8), and the split/merge announcement flag bit layout and
      its own well-formedness checks (§3.4, §4.2, §5 rule 5). Split
      successors are covered by the same ordinary-successor check, since
      each split child still has exactly one parent. **Merge-block
      validation is deliberately not implemented**: it needs `BlockHeader`'s
      `prev_ref_hash_2`/`MERGE_RESULT` fields from **ADR-0016**, which
      `crates/onx-data-structures`' `BlockHeader` struct still does not
      implement — that update, and the merge two-parent structural check it
      unblocks, remain open work.
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
- [x] **Write the "TVM Instruction Set" specification**
      (`docs/specification/tvm-instruction-set.md` + ADR-0017), the concrete
      opcodes/stack-model/gas-price artifact `docs/specification/execution.md`
      §3.1 deferred to. Defines a 46-opcode stack machine (`Integer`/`Bytes`/
      `Cell`/`Slice`/`Builder` values, no first-class continuations) across
      stack, arithmetic, conversion, bit-string, cell-access, cryptographic,
      and control-flow opcodes; maps every bytecode-well-formedness fault
      (bad opcode, stack underflow, out-of-range reference) onto the existing
      closed `ExceptionKind` set as `MalformedCell` rather than adding a
      sixth kind; and draws the `AbsentNode`/pruned-branch boundary at
      content access (`CTOS`) rather than reference-passing (`LDREF`) or
      hashing (`HASHCELL`), so Merkle-proof shape/hash stay usable on pruned
      branches per `whitepaper.md` §5.1.9. Gas prices are an explicit first
      baseline, cross-checked against `transactions.md`'s
      `MAX_TENTATIVE_GAS = 10,000` for plausibility, not calibrated final
      pricing. Unblocks implementing `onx-execution` in code (below).
- [ ] Implement `docs/specification/execution.md` +
      `docs/specification/tvm-instruction-set.md` in code (`onx-execution`
      crate): the `execute(code, data, message, context) -> ...` contract,
      the 46-opcode interpreter and gas accounting, and the `AbsentNode`
      pruned-branch path against `onx-state-model`'s existing `Cell.is_special`
      flag. Depends on `onx-transactions` (for `Message`/gas-limit inputs)
      and `onx-state-model` (for `Cell`/`AccountState`), both of which exist.

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
- [x] ~~Decide and record a license~~ — done: **ADR-0015** accepted
      Apache-2.0, added as `LICENSE`, and set as the `license` field in every
      crate's `Cargo.toml`; see [License](README.md#license).
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
