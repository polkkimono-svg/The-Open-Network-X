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
