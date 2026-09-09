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
