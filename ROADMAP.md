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
- [ ] Write the **Execution (virtual machine)** specification — instruction
      semantics, resource accounting, exceptions, deterministic contract
      state transitions — architecture sequence item 6. Resolves part of
      **ONX-ARCH-006** (VM rules per workchain). **(whitepaper.md §2.1.20, §5.1.9)**
      Note explicitly: the white paper states that Merkle-proof operations
      inside the VM (needed for the Payment channels item below) are much
      harder to retrofit than to design in from the start (§5.1.9, echoing
      the general warning in §2.8.16 about how rigid a blockchain's "genome"
      becomes post-deployment) — whether or not ONX builds payment channels
      soon, this spec should explicitly decide whether to reserve VM
      primitives for Merkle-proof verification now, and record that decision
      (accept, defer, or reject) rather than leaving it implicit.

### Later (depend on consensus existing)

- [ ] Write the **Consensus and validator operation** specification —
      validator lifecycle, assignment, quorum rules, finality, invalid-block
      evidence — architecture sequence item 7. Resolves **ONX-ARCH-005**.
      **(whitepaper.md §2.6)** covers considerably more ground than the one-line
      summary suggests, including: validator election and stake-weighting
      (§2.6.7), nominators and fishermen as distinct non-validator roles for
      capital-provision and invalidity-reporting respectively (§2.6.3–§2.6.4,
      directly relevant to the vertical-block-correction mechanism ADR-0001
      already commits to preserving), rotating validator task groups per
      shard (§2.6.8–§2.6.9), block-candidate propagation and BFT signature
      thresholds (§2.6.10–§2.6.12), a validator signature's "depth" and
      partial/late-signature reward decay (§2.6.20–§2.6.21), and the
      relative-vs-recursive block reliability distinction with a bounded
      (e.g. two-month in the reference) challenge window before a block is
      no longer reconsidered (§2.6.26–§2.6.28) — this last point is a
      concrete finality rule ONX-ARCH-005 needs an explicit answer for.
- [ ] Write a **Networking** specification — architecture sequence item 8,
      **(whitepaper.md §3, resolves ONX-ARCH-010)**. The reference treats this as
      three distinct, separately specifiable sub-layers rather than one
      protocol, and the spec (or specs) should probably follow that split:
      - **Peer identity and transport (ADNL)** — 256-bit abstract addresses
        derived from a hashed, serialized key description, channel/tunnel
        identifiers, and the reliable large-datagram protocol built on top
        (whitepaper.md §3.1);
      - **Peer/service discovery (DHT)** — a Kademlia-like distributed hash
        table keyed by 256-bit hashes, used to locate nodes, services, and
        tunnel entry points (whitepaper.md §3.2);
      - **Overlay networks and propagation** — per-shard gossip/broadcast
        overlays, including the streaming/erasure-coded broadcast protocol
        used for block-candidate propagation described alongside consensus
        in §2.6.10 (whitepaper.md §3.3).
- [ ] Write the **Dynamic sharding** specification — shard-tree invariants,
      split/merge lifecycle, state migration, validator responsibility —
      architecture sequence item 9. Resolves **ONX-ARCH-007** and the
      remainder of **ONX-ARCH-006** (initial workchain set). **(whitepaper.md §2.7)**
      gives concrete mechanics to decide on or explicitly deviate from:
      shard configuration as masterchain state organized as a binary tree
      per workchain (§2.7.1–§2.7.2); split/merge changes announced several
      blocks in advance via header flags before being committed (§2.7.3);
      bounded distance limits on how far the active shard configuration may
      drift from the configuration a validator task group was assigned
      under, before a split/merge is simply refused (§2.7.5); and formal
      load-based trigger conditions for splitting and merging (§2.7.6,
      §2.7.8) that a deterministic implementation needs to pin down
      precisely, not leave as "when load is high enough."
- [ ] Write the **Economics** specification — Onyx supply, denomination,
      fees, rewards, staking, penalties — architecture sequence item 10.
      Resolves **ONX-ARCH-008**. Deliberately last: `INSTRUCTIONS.md` §18
      requires the consensus and resource-accounting model to be specified
      first. **(whitepaper.md Appendix A)** gives TON's own concrete reference
      figures the ONX spec must explicitly accept, adapt, or reject rather
      than silently inherit: a 10^9-unit subdivision (with a further
      2^-16 sub-unit, "specks," for gas-price rounding), an initial supply
      cap, and an inflation model tying validator rewards to a percentage
      of stake per year with slashed stakes partly burned (deflationary).
- [ ] Write a **Payment channels (TON Payments)** specification —
      **(whitepaper.md §5, resolves new ONX-ARCH-009)**. This is not currently
      tracked anywhere in this roadmap or in architecture.md, despite being
      an entire chapter of the reference: point-to-point trustless payment
      channels backed by an on-chain arbiter smart contract (§5.1.1–§5.1.4),
      an asynchronous two-workchain variant avoiding round-trip
      confirmation delay (§5.1.5), conditional transfers/"promises"
      enabling channels to be chained (§5.1.7), and a payment-channel
      network ("lightning network") for multi-hop transfers with path
      finding (§5.2). Depends on Execution existing first, since §5.1.9
      is explicit that this needs VM support for embedding and verifying
      Merkle proofs of an inner (virtual) blockchain's state transitions —
      see the note on the Execution item above.

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
      done in `.github/workflows/ci.yml`, though its `push` trigger only watches a
      `work` branch, not `main`; worth fixing.
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
