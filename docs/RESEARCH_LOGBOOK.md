# Research Question Logbook

This logbook maintains a running chain of research and development questions for Open Network X (ONX).

## Rules for Contributors

Every contributor making a pull request to ONX must participate in the Research Question Logbook:

1. Locate the latest entry in this logbook (`docs/RESEARCH_LOGBOOK.md`).
2. Add a new entry numbered sequentially (`Entry #N`).
3. Label your answer to the previous question clearly using the keyword `[ANSWER]`.
4. Label your new question regarding ONX development clearly using the keyword `[QUESTION]`.
5. Ensure your entry contains both `[ANSWER]` and `[QUESTION]` sections. CI will fail if these keywords/labels are omitted in new logbook entries.

---

## Logbook Entries

### Entry #1

#### [ANSWER]
This is the initial entry establishing the ONX Research Question Logbook. There is no preceding question to answer.

#### [QUESTION]
How should cross-shard message queueing handle gas metering and timeouts deterministically across independent validator sets in ONX?

### Entry #2

#### [ANSWER]
Cross-shard message queueing in ONX ensures deterministic gas metering and timeout processing by decoupling initial outbound message credit reservation from destination execution fees. The sending shard deducts a fixed routing fee and locks an explicit gas limit in nanocoins into the message header upon block inclusion. Destination shard validators process messages in strictly increasing order of destination logical time (`created_lt`). If a destination shard experiences processing delays exceeding the message's defined expiration logical time window, the destination validator set deterministically generates a bounce transaction refunding remaining gas minus transit fees back to the source address, ensuring consistent state across independent validator sets.

#### [QUESTION]
How should account storage fees be calculated and deducted during state transitions, and under what exact conditions should an active account transition to frozen or destroyed state?

### Entry #3

#### [ANSWER]
`docs/specification/state-model.md` already ties the answer's shape to two fields every `Active` account carries: `storage_stat` (`cell_count`, `byte_count`) and `last_trans_lt`. A deterministic storage fee accrues as a function of `storage_stat.byte_count` and the elapsed logical time since `last_trans_lt` (`new_lt - last_trans_lt`), charged as part of the same balance-delta check `AccountState::validate_transition_with_delta` already performs on every transition — so storage fees don't need a separate code path, only a deterministic formula (bytes × elapsed `lt` × a per-byte-per-`lt` rate, itself a `docs/specification/economics.md` parameter, not invented here) feeding the same `delta` that transaction/gas fees already feed. The freeze condition is exactly the case `validate_transition`'s `Active -> Frozen` arm already accepts structurally: an `Active` account transitions to `Frozen` when the account's balance would go negative *after* the storage-fee delta is applied (`BalanceUnderflow` on the storage-only debit, distinct from a `BalanceUnderflow` on a message-driven debit, which instead bounces the message per Entry #2's answer) — at that point storage is pruned to `storage_hash` and no further code execution is permitted until the account is topped up and unfrozen. `Destroyed` is reserved for a `Frozen` account whose balance remains at or below zero past a further, separately-specified grace window (not yet chosen; illustrative TON-derived numbers are exactly the kind of figure `INSTRUCTIONS.md` §7 says isn't binding until an ONX specification or ADR sets it), since collapsing `Frozen -> Destroyed` immediately on the first zero-balance block would give an account no chance to be topped up before its state is discarded.

#### [QUESTION]
Now that `docs/specification/tvm-instruction-set.md` (ADR-0017) prices cell-access opcodes (`NEWC`/`STBITS`/`STREF`/`CTOS`/`LDU`/`LDI`/`LDREF`, each 10 gas) but says nothing about storage itself, how should the per-byte-per-logical-time storage fee rate this entry's answer assumes be integrated into that gas model — as a separate charge `onx-execution` applies once per transaction, outside the opcode gas table `execute()` accounts for during a run, or as part of `ExecutionContext` (per `execution.md` §3.2) so contract code can read its own accruing storage cost mid-execution?

### Entry #4

#### [ANSWER]
Storage fee accrual in ONX is evaluated outside the opcode loop prior to VM execution during account state transition validation (`AccountState::validate_transition_with_delta`), where the storage fee delta accrued over the elapsed logical time (`new_lt - last_trans_lt`) is debited from the account's balance before message execution begins. However, the current storage fee rates (`storage_fee_per_byte_per_lt`) are exposed as protocol-committed fields in `ExecutionContext` (per `docs/specification/execution.md` §3.2), enabling contract execution to inspect the environment parameters without mixing state storage fee debits into the opcode step gas accounting table.

#### [QUESTION]
How should cross-workchain transaction fee conversion rates and message queue expiration logical-time bounds be calibrated and enforced across workchains with heterogeneous VM execution models and block generation frequencies?

### Entry #5

#### [ANSWER]
Cross-workchain transaction fee conversion rates and message queue expiration bounds in ONX are governed by masterchain-committed exchange rate feeds and normalized logical time (`created_lt` / `expire_at_lt`) scaling factors as specified in ADR-0018. Each workchain header committed to the masterchain reports its local gas unit rate in nanocoins and target block period. The masterchain computes and signs canonical cross-workchain exchange matrices during epoch transitions. Message queue expiration bounds are evaluated relative to normalized logical time deltas rather than local block numbers or wall-clock timestamps, guaranteeing deterministic message bounce generation across workchains with heterogeneous execution speeds and gas accounting models.

#### [QUESTION]
How should state migration during shard splits and merges preserve atomic cross-shard message delivery order and state proof validity when accounts move between shard trees under high transaction load?

Entry #5
[ANSWER]
State migration during shard splits and merges in ONX guarantees atomic cross-shard message delivery order and state proof validity by utilizing the masterchain block sequence as the strict coordination layer. When a shard split or merge condition triggers (per ADR-0012), the affected shard blocks flag a pending state transition over a deterministic epoch boundary defined in docs/specification/sharding.md.
During the migration epoch, outbound messages bound for migrating accounts are held in the source shard's output queue using the old ShardIdent prefix routing until the masterchain commits the new shard block headers containing the finalized account state Merkle proofs. Cross-shard message delivery order is preserved because destination shards enforce sequential processing based on the created_lt of the source messages, which remains immutable across the split/merge boundary. State proof validity is maintained because the masterchain guarantees the new shard state roots are cryptographically linked to the pre-migration state root before unlocking the inbound message queues for the newly formed shards.
[QUESTION]
How does the onx-consensus crate handle validator equivocation penalties and Byzantine fault proofs during a 2/3 quorum failure when a subset of validators attempt to finalize competing block candidates on the masterchain?



### Entry #6

#### [ANSWER]
Shard splits and merges in ONX only execute at masterchain-designated epoch boundaries (`docs/specification/shard-tree.md`, ADR-0019), never mid-block, so the logical-time ordering this logbook already depends on (Entry #2's `created_lt`, Entry #5's normalized cross-workchain lt deltas) carries over cleanly instead of resetting. A split/merge triggers when a shard's queue-depth or tx-count metric from the prior epoch crosses a masterchain-committed threshold, but it's gated behind a **freeze window**: one full block period before the boundary, the shard stops admitting new inbound messages that would need re-routing across the new prefix boundary, while continuing to drain its existing outbound queue under Entry #2's ordering rule. That bounds how much in-flight state the split has to reason about, instead of splitting a queue that's still growing under load.

At the boundary: for a split, every account's `storage_stat` and `last_trans_lt` (Entry #3) carry over byte-for-byte into whichever child shard its address prefix resolves to — no re-execution or fee re-basis, since Entry #3's formula only depends on elapsed lt, not shard identity. Queued messages are re-homed by destination prefix, so each child inherits a strict sub-sequence of the parent's `created_lt`-ordered queue; relative order within a prefix is preserved automatically. A merge interleaves two queues by `created_lt` rather than concatenating them, since concatenation would break the strictly-increasing invariant for whichever queue got appended second.

State proofs are the harder problem: a Merkle proof against a pre-split root becomes unverifiable the instant the root is replaced. Rather than forcing every outstanding proof to be reissued instantly, the masterchain block finalizing the split also commits a **migration certificate** — a signed mapping from old sub-tree hashes to their unchanged position under whichever new root now holds them (a split repartitions the tree, it doesn't rehash unmoved leaves). A verifier can present an old proof alongside the certificate for a bounded grace window — left as a masterchain parameter, not invented here, per Entry #3's precedent — after which it must fetch a fresh proof.

One sharp edge this doesn't resolve: a bounce transaction's gas-refund proof (Entry #2) can be in flight across exactly the block where its source shard splits, and `docs/specification/shard-tree.md` doesn't yet say whether the certificate mechanism covers it.

#### [QUESTION]
Should a migration certificate's grace window be a fixed lt-delta set once in `docs/specification/economics.md` (mirroring ADR-0018's static exchange-matrix cadence), or computed per-split from the freeze-window drain rate observed just before the boundary — and if the latter, how does a light client, which doesn't run shard validation itself, learn which formula and inputs applied to a certificate it's holding?
