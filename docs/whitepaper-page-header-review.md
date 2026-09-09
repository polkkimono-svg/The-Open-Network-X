# Whitepaper page-header removal: manual review needed

62 of 62 injected page-break blocks land mid-sentence or mid-word and were NOT auto-resolved. For each, the text immediately before and after the injected `<blank>/<page number>/<blank>/<running header>/<blank>` block is shown so the original wording can be reconstructed by hand.

## Page 5 / "2.1. TON Blockchain as a Collection of 2-Blockchains" (line 88)
- Classification: mid-sentence
- Before (last line before the block):
  ```
     - Each workchain is in turn subdivided into up to 2<sup>60</sup> shard blockchains, or shardchains for short, having the same rules and block format as
  ```
- After (first line after the block):
  ```
  the workchain itself, but responsible only for a subset of accounts, depending on several first (most significant) bits of the account address. In other words, a form of sharding is built into the system (cf. 2.8.12). Because all these shardchains share a common block format and rules, the TON Blockchain is homogeneous in this respect (cf. 2.8.8), similarly to what has been discussed in one of Ethereum scaling proposals.<sup>1</sup>
  ```

## Page 6 / "2.1. TON Blockchain as a Collection of 2-Blockchains" (line 104)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  1https://github.com/ethereum/wiki/wiki/Sharding-FAQ
  ```
- After (first line after the block):
  ```
  collection of blocks of account-chains that have been assigned to this shard. Thus the “account-chains” have only a purely virtual or logical existence inside the “shardchains”.
  ```

## Page 7 / "2.1. TON Blockchain as a Collection of 2-Blockchains" (line 120)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  2.1.6. Identification of workchains. Each workchain is identified by its number or workchain identifier (workchain_id : uint 32 ), which is simply an
  ```
- After (first line after the block):
  ```
  unsigned 32-bit integer. Workchains are created by special transactions in the masterchain, defining the (previously unused) workchain identifier and the formal description of the workchain, sufficient at least for the interaction of this workchain with other workchains and for superficial verification of this workchain's blocks.
  ```

## Page 8 / "2.1. TON Blockchain as a Collection of 2-Blockchains" (line 136)
- Classification: uncertain boundary
- Before (last line before the block):
  ```
  An important feature of the TON Blockchain is that it implements dynamic sharding, meaning that the number of shards is not fixed. Instead, shard ( _w, s_ ) can be automatically subdivided into shards ( _w, s._ 0) and ( _w, s._ 1) if some formal conditions are met (essentially, if the transaction load on the original shard is high enough for a prolonged period of time). Conversely,
  ```
- After (first line after the block):
  ```
  if the load stays too low for some period of time, the shards ( _w, s._ 0) and ( _w, s._ 1) can be automatically merged back into shard ( _w, s_ ) .
  ```

## Page 11 / "2.1. TON Blockchain as a Collection of 2-Blockchains" (line 180)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  However, this is not sufficient, because the overall state of the system (TON Blockchain) turns out to be invalid because of the invalid shardchain block previously committed. This invalid block must be replaced by a newer
  ```
- After (first line after the block):
  ```
  valid version.
  ```

## Page 12 / "2.1. TON Blockchain as a Collection of 2-Blockchains" (line 196)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  Once the “history rewriting” ripples reach the most recent blocks, the new shardchain blocks are generated in one version only, being successors of the newest block versions only. This means that they will contain references to
  ```
- After (first line after the block):
  ```
  the correct (most recent) vertical blocks from the very beginning.
  ```

## Page 13 / "2.1. TON Blockchain as a Collection of 2-Blockchains" (line 218)
- Classification: uncertain boundary
- Before (last line before the block):
  ```
  - TVM represents all data as a collection of (TVM) cells (cf. 2.3.14). Each cell contains up to 128 data bytes and up to 4 references to other cells. As a consequence of the “everything is a bag of cells” philosophy
  ```
- After (first line after the block):
  ```
     - (cf. 2.5.14), this enables TVM to work with all data related to the TON Blockchain, including blocks and blockchain global state if necessary.
  ```

## Page 16 / "2.2. Generalities on Blockchains" (line 294)
- Classification: uncertain boundary
- Before (last line before the block):
  ```
  > 4https://core.telegram.org/mtproto/TL
  ```
- After (first line after the block):
  ```
  A collection of constructor and type definitions is called a TL-scheme. It is usually kept in one or several files with the suffix .tl.
  ```

## Page 17 / "2.2. Generalities on Blockchains" (line 322)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  can be derived from ev_trans. It takes a block _B_ : Block and the previous blockchain state _s_ : State (which might include the hash of the previous
  ```
- After (first line after the block):
  ```
  block) and computes the next blockchain state _s_<sup>_′_</sup> = ev_block ( _B_ )( _s_ ) : State, which is either a true state or a special value _⊥_ indicating that the next state cannot be computed (i.e., that the block is invalid if evaluated from the starting state given—for example, the block includes a transaction trying to debit an empty account.)
  ```

## Page 18 / "2.3. Blockchain State, Accounts and Hashmaps" (line 348)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  However, this is not so convenient for the proofs. If (8) is used at most _N_ times in a proof with 2<sup>_−k_</sup> _N < ϵ_ for some small _ϵ_ (say, _ϵ_ = 10<sup>_−_18</sup> ), we can
  ```
- After (first line after the block):
  ```
  reason as if (7) were true, provided we accept a failure probability _ϵ_ (i.e., the final conclusions will be true with probability at least 1 _− ϵ_ ).
  ```

## Page 20 / "2.3. Blockchain State, Accounts and Hashmaps" (line 402)
- Classification: uncertain boundary
- Before (last line before the block):
  ```
  
  ```
- After (first line after the block):
  ```
  or
  ```

## Page 24 / "2.3. Blockchain State, Accounts and Hashmaps" (line 474)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  This raw value format may be used to implement arbitrary sum-product algebraic types. In this case, the value would contain a raw byte first, describing the “constructor” being used (from the perspective of a high-level
  ```
- After (first line after the block):
  ```
  language), and then other “fields” or “constructor arguments”, consisting of raw bytes and references to other structures depending on the constructor chosen (cf. 2.2.5). However, TVM does not know anything about the correspondence between constructors and their arguments; the mixture of bytes and references is explicitly described by certain descriptor bytes.<sup>8</sup>
  ```

## Page 25 / "2.3. Blockchain State, Accounts and Hashmaps" (line 496)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  > 10Logically; the “bag of cells” representation described in 2.5.5 identifies all duplicate
  ```
- After (first line after the block):
  ```
  the smart-contract description. If necessary, a Merkle tree hash of this entire persistent storage is recursively computed, starting from the leaves and then simply replacing all references in a cell with the recursively computed hashes of the referenced cells, and subsequently computing the hash of the byte string thus obtained.
  ```

## Page 30 / "2.4. Messages Between Shardchains" (line 580)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  2.4.6. External messages, or “messages from nowhere”. Some messages arrive into the system “from nowhere”—that is, they are not generated by an account (smart contract or not) residing in the blockchain. The most
  ```
- After (first line after the block):
  ```
  typical example arises when a user wants to transfer some funds from an account controlled by her to some other account. In this case, the user sends a “message from nowhere” to her own account, requesting it to generate a message to the receiving account, carrying the specified value. If this message is correctly signed, her account receives it and generates the required outbound messages.
  ```

## Page 31 / "2.4. Messages Between Shardchains" (line 596)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  2.4.8. Interaction with off-chain services and external blockchains. These external input and output messages can be used for interacting with
  ```
- After (first line after the block):
  ```
  off-chain services and other (external) blockchains, such as Bitcoin or Ethereum. One might create tokens or cryptocurrencies inside the TON Blockchain pegged to Bitcoins, Ethers or any ERC-20 tokens defined in the Ethereum blockchain, and use “messages from nowhere” and “messages to nowhere”, generated and processed by scripts residing on some third-party off-chain servers, to implement the necessary interaction between the TON Blockchain and these external blockchains.
  ```

## Page 37 / "2.5. Global Shardchain State. “Bag of Cells” Philosophy." (line 674)
- Classification: mid-word (hyphenated split)
- Before (last line before the block):
  ```
  Messages cannot be automatically routed through the masterchain. A message with workchain_id = _−_ 1 ( _−_ 1 being the special workchain_id indi-
  ```
- After (first line after the block):
  ```
  cating the masterchain) cannot be delivered to the masterchain. In principle, one can create a message-forwarding smart contract inside the masterchain, but the price of using it would be prohibitive.
  ```

## Page 39 / "2.5. Global Shardchain State. “Bag of Cells” Philosophy." (line 714)
- Classification: uncertain boundary
- Before (last line before the block):
  ```
  2.5.5. Low-level perspective: “bag of cells”. There is a “low-level” description of the account-chain or shardchain state as well, complementary to the “high-level” description given above. This description is quite important, because it turns out to be pretty universal, providing a common basis for representing, storing, serializing and transferring by network almost all data used by the TON Blockchain (blocks, shardchain states, smart-contract storage, Merkle proofs, etc.). At the same time, such a universal “low-level”
  ```
- After (first line after the block):
  ```
  description, once understood and implemented, allows us to concentrate our attention on the “high-level” considerations only.
  ```

## Page 44 / "2.6. Creating and Validating New Blocks" (line 778)
- Classification: mid-word (hyphenated split)
- Before (last line before the block):
  ```
  All this means that the validator does not get its money “for nothing”. Indeed, it must keep track of the states of all or some shardchains (each validator is responsible for validating and creating new blocks in a certain subset of shardchains), perform all computations requested by smart con-
  ```
- After (first line after the block):
  ```
  tracts in these shardchains, receive updates about other shardchains and so on. This activity requires considerable disk space, computing power and network bandwidth.
  ```

## Page 46 / "2.6. Creating and Validating New Blocks" (line 808)
- Classification: uncertain boundary
- Before (last line before the block):
  ```
  Because a validator needs to submit new (collated) block candidates to obtain some (“mining”) rewards, it makes sense to pay some part of the reward to a collator willing to provide suitable block candidates. In this way,
  ```
- After (first line after the block):
  ```
  a validator may free itself from the necessity of watching the state of the neighboring shardchains, by outsourcing it to a collator.
  ```

## Page 48 / "2.6. Creating and Validating New Blocks" (line 848)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  When a new shardchain block needs to be generated, the shard task group validator selected to create this block is normally the first one with respect to this rotating “priority” order. If it fails to create the block, the second or third validator may do it. Essentially, all of them may suggest their block candidates, but the candidate suggested by the validator having the highest
  ```
- After (first line after the block):
  ```
  priority should win as the result of Byzantine Fault Tolerant (BFT) consensus protocol.
  ```

## Page 49 / "2.6. Creating and Validating New Blocks" (line 862)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  If the “multicast mesh” (overlay network) remains connected after removing all “bad” nodes (recall that up to one-third of nodes are allowed to be
  ```
- After (first line after the block):
  ```
  bad in a Byzantine way, i.e., behave in arbitrary malicious fashion), this algorithm will propagate the block candidate as quickly as possible.
  ```

## Page 53 / "2.6. Creating and Validating New Blocks" (line 918)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  For example, imagine that the cell expected to contain the original balance of an account accessed from a transaction committed into a block turns out to have zero raw bytes instead of the expected 8 or 16. Then the original
  ```
- After (first line after the block):
  ```
  balance simply cannot be retrieved from the cell, and an “unhandled exception” happens while trying to process the block. In this case, the validator should not sign such a block on pain of being punished.
  ```

## Page 54 / "2.6. Creating and Validating New Blocks" (line 932)
- Classification: uncertain boundary
- Before (last line before the block):
  ```
  2.6.25. Decentralization of the system. One might suspect that a Proofof-Stake system such as the TON Blockchain, relying on _T ≈_ 1000 validators to create all shardchain and masterchain blocks, is bound to become “too centralized”, as opposed to conventional Proof-of-Work blockchains like Bitcoin or Ethereum, where everybody (in principle) might mine a new block,
  ```
- After (first line after the block):
  ```
  without an explicit upper limit on the total number of miners.
  ```

## Page 57 / "2.7. Splitting and Merging Shardchains" (line 968)
- Classification: uncertain boundary
- Before (last line before the block):
  ```
  The shard configuration—i.e., this shard binary tree, or the collection of all active ( _w, s_ ) for a given _w_ (corresponding to the leaves of the shard binary tree)—is part of the masterchain state and is available to everybody
  ```
- After (first line after the block):
  ```
  #### who keeps track of the masterchain.<sup>22</sup>
  ```

## Page 59 / "2.7. Splitting and Merging Shardchains" (line 1002)
- Classification: uncertain boundary
- Before (last line before the block):
  ```
  2.7.6. Determining the necessity of split operations. The split operation for a shardchain is triggered by certain formal conditions (e.g., if for 64 consecutive blocks the shardchain blocks are at least 90% full). These conditions are monitored by the shardchain task group. If they are met,
  ```
- After (first line after the block):
  ```
  first a “split preparation” ag is included in the header of a new shardchain block (and propagated to the masterchain block referring to this shardchain block). Then, several blocks afterwards, the “split commit” ag is included in the header of the shardchain block (and propagated to the next masterchain block).
  ```

## Page 63 / "2.8. Classification of Blockchain Projects" (line 1064)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  The Proof-of-Stake approach is more natural in the respect that it incentivizes the validators (which replace PoW miners) to perform useful computation (needed to check or create new blocks, in particular, by performing all transactions listed in a block) instead of computing otherwise useless hashes. In this way, validators would purchase hardware that is better adapted to processing user transactions, in order to receive rewards associated with these transactions, which seems quite a useful investment from the perspective of
  ```
- After (first line after the block):
  ```
  the system as a whole.
  ```

## Page 64 / "2.8. Classification of Blockchain Projects" (line 1088)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  While there seem to be 2<sup>4</sup> possible classes of PoS algorithms depending on the answers to these questions, the distinction in practice boils down to two
  ```
- After (first line after the block):
  ```
  major approaches to PoS. In fact, most modern PoS algorithms, designed to be used in scalable multi-chain systems, answer the first two questions in the same fashion: only validators can produce new blocks, and they guarantee block validity without requiring all full nodes to check the validity of all blocks by themselves.
  ```

## Page 66 / "2.8. Classification of Blockchain Projects" (line 1116)
- Classification: uncertain boundary
- Before (last line before the block):
  ```
  On the other hand, DPOS might be a good choice for a “loosely-coupled”
  ```
- After (first line after the block):
  ```
  multi-chain system, where fast interaction between blockchains is not required e.g., if each blockchain (“workchain”) represents a separate distributed exchange, and inter-blockchain interaction is limited to rare transfers of tokens from one workchain into another (or, rather, trading one altcoin residing in one workchain for another at a rate approaching 1 : 1 ). This is what is actually done in the BitShares project, which uses DPOS quite successfully.
  ```

## Page 68 / "2.8. Classification of Blockchain Projects" (line 1146)
- Classification: mid-word (hyphenated split)
- Before (last line before the block):
  ```
  2.8.10. Heterogeneous systems with several workchains having the same rules, or confederations. In some cases, several blockchains (work-
  ```
- After (first line after the block):
  ```
  chains) with the same rules can be present in a heterogeneous system, but the interaction between them is the same as between blockchains with different rules (i.e., their similarity is not exploited explicitly). Even if they appear to use “the same” cryptocurrency, they in fact use different “altcoins” (independent incarnations of the cryptocurrency). Sometimes one can even have certain mechanisms to convert these altcoins at a rate near to 1 : 1 . However, this does not make the system homogeneous in our view; it remains heterogeneous. We say that such a heterogeneous collection of workchains with the same rules is a confederation.
  ```

## Page 69 / "2.8. Classification of Blockchain Projects" (line 1160)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  2.8.12. Sharding support. Some blockchain projects (or systems) have native support for sharding, meaning that several (necessarily homogeneous; cf. 2.8.8) blockchains are thought of as shards of a single (from a highlevel perspective) virtual blockchain. For example, one can create 256 shard
  ```
- After (first line after the block):
  ```
  blockchains (“shardchains”) with the same rules, and keep the state of an account in exactly one shard selected depending on the first byte of its account_id .
  ```

## Page 71 / "2.8. Classification of Blockchain Projects" (line 1188)
- Classification: uncertain boundary
- Before (last line before the block):
  ```
  Of course, building a “loosely-coupled” system is much simpler; however,
  ```
- After (first line after the block):
  ```
  fast and efficient sharding (cf. 2.8.12) requires the system to be “tightlycoupled”.
  ```

## Page 72 / "2.9. Comparison to Other Blockchain Projects" (line 1212)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  2.8.16. Complications of changing the “genome” of a blockchain project. The above classification defines the “genome” of a blockchain project. This genome is quite “rigid”: it is almost impossible to change it once the project is deployed and is used by a lot of people. One would need a series of hard forks (which would require the approval of the majority of the
  ```
- After (first line after the block):
  ```
  community), and even then the changes would need to be very conservative in order to preserve backward compatibility (e.g., changing the semantics of the virtual machine might break existing smart contracts). An alternative would be to create new “sidechains” with their different rules, and bind them somehow to the blockchain (or the blockchains) of the original project. One might use the blockchain of the existing single-blockchain project as an external masterchain for an essentially new and separate project.<sup>27</sup>
  ```

## Page 75 / "2.9. Comparison to Other Blockchain Projects" (line 1269)
- Classification: uncertain boundary
- Before (last line before the block):
  ```
  > 30https://blog.ethereum.org/2015/08/01/introducing-casper-friendly-ghost/
  ```
- After (first line after the block):
  ```
  successfully created the BitShares and SteemIt projects, demonstrating the strong points of the DPoS consensus algorithm. Scalability will be achieved by creating specialized workchains for projects that need it (e.g., a distributed exchange might use a workchain supporting a special set of optimized transactions, similarly to what BitShares did) and by creating multiple workchains with the same rules (confederations in the sense described in 2.8.10). The drawbacks and limitations of this approach to scalability have been discussed in loc. cit. Cf. also 2.8.5, 2.8.12, and 2.8.14 for a more detailed discussion of DPoS, sharding, interaction between workchains and their implications for the scalability of a blockchain system.
  ```

## Page 76 / "2.9. Comparison to Other Blockchain Projects" (line 1289)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  2.9.9. Universa; https://universa.io. The only reason we mention this
  ```
- After (first line after the block):
  ```
  unusual blockchain project here is because it is the only project so far to make in passing an explicit reference to something similar to our Infinite Sharding Paradigm (cf. 2.1.2). Its other peculiarity is that it bypasses all complications related to Byzantine Fault Tolerance by promising that only trusted and licensed partners of the project will be admitted as validators, hence they will never commit invalid blocks. This is an interesting decision; however, it essentially makes a blockchain project deliberately centralized, something blockchain projects usually want to avoid (why does one need a blockchain at all to work in a trusted centralized environment?).
  ```

## Page 77 / "2.9. Comparison to Other Blockchain Projects" (line 1299)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  2.9.11. Specialized blockchain projects. There are also some specialized blockchain projects, such as FileCoin (a system that incentivizes users to offer their disk space for storing the files of other users who are willing to pay for it), Golem (a blockchain-based platform for renting and lending computing power for specialized applications such as 3D-rendering) or SONM (another similar computing power-lending project). Such projects do not introduce anything conceptually new on the level of blockchain organization; rather, they are particular blockchain applications, which could be implemented by smart contracts running in a general-purpose blockchain, provided it can
  ```
- After (first line after the block):
  ```
  deliver the required performance. As such, projects of this kind are likely to use one of the existing or planned blockchain projects as their base, such as EOS, PolkaDot or TON. If a project needs “true” scalability (based on sharding), it would better use TON; if it is content to work in a “confederated” context by defining a family of workchains of its own, explicitly optimized for its purpose, it might opt for EOS or PolkaDot.
  ```

## Page 78 / "2.9. Comparison to Other Blockchain Projects" (line 1311)
- Classification: uncertain boundary
- Before (last line before the block):
  ```
  Let us consider “uploading Facebook into a blockchain” as a thought experiment; any other project of similar scale might serve as an example as well. Once Facebook is uploaded into a blockchain, all operations currently done by Facebook's servers will be serialized as transactions in certain blockchains (e.g., TON's shardchains), and will be performed by all validators of these blockchains. Each operation will have to be performed, say, at least twenty times, if we expect every block to collect at least twenty validator signatures (immediately or eventually, as in DPOS systems). Similarly, all data kept by
  ```
- After (first line after the block):
  ```
  Facebook's servers on their disks will be kept on the disks of all validators for the corresponding shardchain (i.e., in at least twenty copies).
  ```

## Page 81 / "3.1. Abstract Datagram Network Layer" (line 1351)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  In this case, the sender simply augments the datagram to be sent by its ECC signature (done with its private key) and its source address (or the preimage of the source address, if the receiver is not known to know that
  ```
- After (first line after the block):
  ```
  preimage yet). The result is encrypted with the recipient's public key, embedded into a UDP datagram and sent to the known IP and port of the recipient. Because the first 256 bits of the UDP datagram contain the recipient's abstract address, the recipient can identify which private key should be used to decrypt the remainder of the datagram. Only after that is the sender's identity revealed.
  ```

## Page 82 / "3.1. Abstract Datagram Network Layer" (line 1367)
- Classification: mid-word (hyphenated split)
- Before (last line before the block):
  ```
  3.1.6. Channel as a tunnel identifier. In general, a “channel”, or “channel identifier” simply selects a way of processing an inbound UDP datagram, known to the receiver. If the channel is the receiver's abstract address, the processing is done as outlined in 3.1.3 or 3.1.4; if the channel is an estab-
  ```
- After (first line after the block):
  ```
  lished point-to-point channel discussed in 3.1.5, the processing consists in decrypting the datagram with the aid of the shared secret as explained in loc. cit., and so on.
  ```

## Page 83 / "3.2. TON DHT: Kademlia-like Distributed Hash Table" (line 1385)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  > 31https://geti2p.net/en/docs/how/garlic-routing
  ```
- After (first line after the block):
  ```
  special queries sent to the already known nodes.
  ```

## Page 84 / "3.2. TON DHT: Kademlia-like Distributed Hash Table" (line 1405)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  3.2.1. Keys of the TON DHT. The keys of the TON DHT are simply 256bit integers. In most cases, they are computed as sha256 of a TL-serialized object (cf. 2.2.5), called preimage of the key, or key description. In some cases, the abstract addresses of the TON Network nodes (cf. 3.1.1) can also
  ```
- After (first line after the block):
  ```
  be used as keys of the TON DHT, because they are also 256-bit, and they are also hashes of TL-serialized objects. For example, if a node is not afraid of publishing its IP address, it can be found by anybody who knows its abstract address by simply looking up that address as a key in the DHT.
  ```

## Page 85 / "3.2. TON DHT: Kademlia-like Distributed Hash Table" (line 1425)
- Classification: uncertain boundary
- Before (last line before the block):
  ```
  3.2.5. Kademlia-like DHTs and the TON DHT. We say that a distributed hash table (DHT) with 256-bit keys and 256-bit node addresses is a
  ```
- After (first line after the block):
  ```
  Kademlia-like DHT if it is expected to keep the value of key _K_ on _s_ Kademlianearest nodes to _K_ (i.e., the _s_ nodes with smallest Kademlia distance from their addresses to _K_ .)
  ```

## Page 88 / "3.2. TON DHT: Kademlia-like Distributed Hash Table" (line 1475)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  3.2.11. Fall-back keys. Most of the “key types” described so far have an extra 32-bit integer field in their TL description, normally equal to zero. However, if the key obtained by hashing that description cannot be retrieved from or updated in the TON DHT, the value in this field is increased, and
  ```
- After (first line after the block):
  ```
  a new attempt is made. In this way, one cannot “capture” and “censor” a key (i.e., perform a key retention attack) by creating a lot of abstract addresses lying near the key under attack and controlling the corresponding DHT nodes.
  ```

## Page 89 / "3.3. Overlay Networks and Multicasting Messages" (line 1495)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  The most natural way would be to use the same private key that controls the account in the TON Blockchain to sign and publish updates in the TON DHT about the abstract addresses associated with that account. This is done almost in the same way as described in 3.2.12; however, the DHT key employed would require a special key description, containing only the
  ```
- After (first line after the block):
  ```
  account_id itself, equal to sha256 of the “account description”, which contains the public key of the account. The signature, included in the value of this DHT key, would contain the account description as well.
  ```

## Page 90 / "3.3. Overlay Networks and Multicasting Messages" (line 1517)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  Therefore, the need to build arbitrary overlay subnetworks, open to any nodes willing to participate, arises. Special gossip protocols, built upon ADNL, will be run in these overlay networks. In particular, these gossip
  ```
- After (first line after the block):
  ```
  protocols may be used to propagate (broadcast) arbitrary data inside such a subnetwork.
  ```

## Page 91 / "3.3. Overlay Networks and Multicasting Messages" (line 1539)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  3.3.3. Private and public overlay networks. Some overlay networks are public, meaning that any node can join them at will. Other are private, meaning that only certain nodes can be admitted (e.g., those that can prove
  ```
- After (first line after the block):
  ```
  their identities as validators.) Some private overlay networks can even be unknown to the “general public”. The information about such overlay networks is made available only to certain trusted nodes; for example, it can be encrypted with a public key, and only nodes having a copy of the corresponding private key will be able to decrypt this information.
  ```

## Page 93 / "3.3. Overlay Networks and Multicasting Messages" (line 1573)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  3.3.9. The overlay network is a random subgraph. In this way, the overlay network becomes a random subgraph inside the ADNL network. If the degree of each vertex is at least three (i.e., if each node is connected to at least three neighbors), this random graph is known to be connected with a probability almost equal to one. More precisely, the probability of a random graph with _n_ vertices being disconnected is exponentially small, and this probability can be completely neglected if, say, _n ≥_ 20 . (Of course, this does not apply in the case of a global network partition, when nodes on different sides of the partition have no chance to learn about each other.) On the
  ```
- After (first line after the block):
  ```
  other hand, if _n_ is smaller than 20, it would suffice to require each vertex to have, say, at least ten neighbors.
  ```

## Page 94 / "3.3. Overlay Networks and Multicasting Messages" (line 1591)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  3.3.13. More sophisticated broadcast protocols. Some applications may warrant more sophisticated broadcast protocols. For instance, for broadcasting messages of substantial size, it makes sense to send to the neighbors not the newly-received message itself, but its hash (or a collection of hashes
  ```
- After (first line after the block):
  ```
  of new messages). The neighbor may request the message itself after learning a previously unseen message hash, to be transferred, say, using the reliable large datagram protocol (RLDP) discussed in 3.1.9. In this way, the new message will be downloaded from one neighbor only.
  ```

## Page 95 / "3.3. Overlay Networks and Multicasting Messages" (line 1605)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  This protocol has already been outlined in 2.6.10: the new (large) broadcast message is split into, say, _N_ one-kilobyte chunks; the sequence of these chunks is augmented to _M ≥ N_ chunks by means of an erasure code such as the Reed–Solomon or a fountain code (e.g., the RaptorQ code [9] [14]), and these _M_ chunks are streamed to all neighbors in ascending chunk number order. The participating nodes collect these chunks until they can recover the original large message (one would have to successfully receive at least _N_ of the chunks for this), and then instruct their neighbors to stop sending new chunks of the stream, because now these nodes can generate the subsequent chunks on their own, having a copy of the original message. Such nodes continue to generate the subsequent chunks of the stream and send them to
  ```
- After (first line after the block):
  ```
  their neighbors, unless the neighbors in turn indicate that this is no longer necessary.
  ```

## Page 96 / "3.3. Overlay Networks and Multicasting Messages" (line 1623)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  3.3.17. Overlay networks within overlay networks. Another interesting case arises in the implementation of TON Payments (a “lightning network” for instant off-chain value transfers; cf. 5.2). In this case, first an overlay network containing all transit nodes of the “lightning network” is constructed. However, some of these nodes have established payment channels in the blockchain; they must always be neighbors in this overlay network, in
  ```
- After (first line after the block):
  ```
  addition to any “random” neighbors selected by the general overlay network algorithms described in 3.3.6, 3.3.7 and 3.3.8. These “permanent links” to the neighbors with established payment channels are used to run specific lightning network protocols, thus effectively creating an overlay subnetwork (not necessarily connected, if things go awry) inside the encompassing (almost always connected) overlay network.
  ```

## Page 100 / "4.1. TON Service Implementation Strategies" (line 1669)
- Classification: uncertain boundary
- Before (last line before the block):
  ```
  4.1.7. Example: keeping files off-chain; TON Storage. An example of such a service is given by TON Storage. In its simplest form, it allows users to store files off-chain, by keeping on-chain only a hash of the file to be stored, and possibly a smart contract where some other parties agree to keep the file in question for a given period of time for a pre-negotiated fee. In fact, the file may be subdivided into chunks of some small size (e.g., 1 kilobyte), augmented by an erasure code such as a Reed–Solomon or a fountain code, a
  ```
- After (first line after the block):
  ```
  Merkle tree hash may be constructed for the augmented sequence of chunks, and this Merkle tree hash might be published in the smart contract instead of or along with the usual hash of the file. This is somewhat reminiscent of the way files are stored in a torrent.
  ```

## Page 101 / "4.2. Connecting Users and Service Providers" (line 1683)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  4.1.9. Example: “fog computing” platforms as decentralized mixed services. Another example of such a decentralized mixed application arises
  ```
- After (first line after the block):
  ```
  when one wants to perform some specific computations (e.g., 3D rendering or training neural networks), often requiring specific and expensive hardware. Then those having such equipment might offer their services through a similar “exchange”, and those needing such services would rent them, with the obligations of the sides registered by means of smart contracts. This is similar to what “fog computing” platforms, such as Golem (https://golem.network/) or SONM (https://sonm.io/), promise to deliver.
  ```

## Page 102 / "4.3. Accessing TON Services" (line 1703)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  4.2.2. Example: uploading a file into TON Storage. Similarly, if one wants to upload a file into the TON Storage, she must locate some nodes
  ```
- After (first line after the block):
  ```
  willing to sign a smart contract binding them to keep a copy of that file (or of any file below a certain size limit, for that matter). Therefore, a registry of nodes offering their services for storing files is needed.
  ```

## Page 105 / "4.3. Accessing TON Services" (line 1749)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  4.3.7. Translating a TON DNS domain. Once any full node, acting by itself or on behalf of some light client, can look up entries in the database
  ```
- After (first line after the block):
  ```
  of any DNS smart contract, arbitrary TON DNS domain names can be recursively translated, starting from the well-known and fixed root DNS smart contract (account) identifier.
  ```

## Page 109 / "4.3. Accessing TON Services" (line 1805)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  4.3.20. Hyperlinks. Notice that the HTML pages returned by ton-sites may contain ton-hyperlinks—that is, references to other ton-sites, smart contracts and accounts by means of specially crafted URI schemes (cf. 4.3.21)— containing either abstract network addresses, account identifiers, or humanreadable TON DNS domains. Then a “ton-browser” might follow such a
  ```
- After (first line after the block):
  ```
  hyperlink when the user selects it, detect the interface to be used, and display a user interface form as outlined in 4.3.15 and 4.3.16.
  ```

## Page 112 / "5.1. Payment Channels" (line 1851)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  This is usually achieved with the aid of signatures. The payment channel smart contract knows the public keys of _A_ and _B_ , and it can check their signatures if needed. The payment channel protocol requires the parties to sign the intermediate states and send the signatures to each other. Then, if one of the parties cheats—for instance, pretends that some state of the payment channel never existed—its misbehavior can be proved by showing its signature on that state. The payment channel smart contract acts as an “on-chain arbiter”, able to process complaints of the two parties about each other, and punish the guilty party by confiscating all of its money and
  ```
- After (first line after the block):
  ```
  awarding it to the other party.
  ```

## Page 113 / "5.1. Payment Channels" (line 1867)
- Classification: uncertain boundary
- Before (last line before the block):
  ```
  If one of the two parties cheats—for example, by signing two different states as final, or by signing two different next states _Si_ +1 and _Si_<sup>_′_</sup> +1<sup>,orby</sup>
  ```
- After (first line after the block):
  ```
  signing an invalid new state _Si_ +1 (e.g., with imbalance _δi_ +1 _< −a_ or _> b_ )— then the other party may submit proof of this misbehavior to a third method of the smart contract. The guilty party is punished immediately by losing its share in the payment channel completely.
  ```

## Page 115 / "5.1. Payment Channels" (line 1899)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  In particular, we want to be able to commit “promises”, or “conditional money transfers”: _A_ agrees to send _c_ coins to _B_ , but _B_ will get the money
  ```
- After (first line after the block):
  ```
  only if a certain condition is fulfilled, for instance, if _B_ can present some string _u_ with Hash ( _u_ ) = _v_ for a known value of _v_ . Otherwise, _A_ can get the money back after a certain period of time.
  ```

## Page 116 / "5.1. Payment Channels" (line 1917)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  At this point the “everything is a bag of cells” paradigm (cf. 2.5.14) becomes extremely convenient. Since all blocks (including the blocks of the ephemeral payment channel blockchain) are represented as bags of cells (and described by some algebraic data types), and the same holds for messages and Merkle proofs as well, a Merkle proof can easily be embedded into an
  ```
- After (first line after the block):
  ```
  inbound message sent to the payment channel smart contract. The “hash condition” of the Merkle proof will be checked automatically, and when the smart contract accesses the “Merkle proof” presented, it will work with it as if it were a value of the corresponding algebraic data type—albeit incomplete, with some subtrees of the tree replaced by special nodes containing the Merkle hash of the omitted subtree. Then the smart contract will work with that value, which might represent, for instance, a block of the payment channel (virtual) blockchain along with its state, and will evaluate the ev_block function (cf. 2.2.6) of that blockchain on this block and the previous state. Then either the computation nishes, and the final state can be compared with that asserted in the block, or an “absent node” exception is thrown while attempting to access an absent subtree, indicating that the Merkle proof is invalid.
  ```

## Page 117 / "5.2. Payment Channel Network, or “Lightning Network”" (line 1931)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  If the encompassing payment channel is asymmetric, two promises need to be committed into the two workchains: _A_ will promise to pay _−δ_ coins to _B_ if the final settlement of the “internal” simple payment channel yields a negative final imbalance _δ_ with 0 _≤−δ ≤ c_ ; and _B_ will have to promise to pay _δ_ to _A_ if _δ_ is positive. On the other hand, if the encompassing
  ```
- After (first line after the block):
  ```
  payment channel is symmetric, this can be done by committing a single “simple payment channel creation” transaction with parameters ( _c, d_ ) into the single payment channel blockchain by _A_ (which would freeze _c_ coins belonging to _A_ ), and then committing a special “confirmation transaction” by _B_ (which would freeze _d_ coins of _B_ ).
  ```

## Page 119 / "5.2. Payment Channel Network, or “Lightning Network”" (line 1965)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  An alternative is to create a virtual payment channel inside the chain linking _A_ to _E_ in the payment channel network. For this, _A_ and _E_ create
  ```
- After (first line after the block):
  ```
  a (virtual) blockchain for their payments, as if they were going to create a payment channel in the blockchain. However, instead of creating a payment channel smart contract in the blockchain, they ask all intermediate payment channels—those linking _A_ to _B_ , _B_ to _C_ , etc.—to create simple payment channels inside them, bound to the virtual blockchain created by _A_ and _E_ (cf. 5.1.10). In other words, now a promise to transfer money according to the final settlement between _A_ and _E_ exists inside every intermediate payment channel.
  ```

## Page 120 / "5.2. Payment Channel Network, or “Lightning Network”" (line 1977)
- Classification: mid-sentence
- Before (last line before the block):
  ```
  5.2.6. Finding paths in the lightning network. One point remains undiscussed so far: how will _A_ and _E_ nd a path connecting them in the payment network? If the payment network is not too large, an OSPF-like protocol can be used: all nodes of the payment network create an overlay network (cf. 3.3.17), and then every node propagates all available link (i.e., participating payment channel) information to its neighbors by a gossip protocol. Ultimately, all nodes will have a complete list of all payment channels participating in the payment network, and will be able to nd the shortest paths by themselves—for example, by applying a version of Dijkstra's algorithm modified to take into account the “capacities” of the payment channels involved (i.e., the maximal amounts that can be transferred along them). Once a candidate path is found, it can be probed by a special ADNL datagram containing the full path, and asking each intermediate node to confirm the existence of the payment channel in question, and to forward this datagram further according to the path. After that, a chain can be constructed, and a protocol for chain transfers (cf. 5.2.4), or for creating a virtual payment
  ```
- After (first line after the block):
  ```
  channel inside a chain of payment channels (cf. 5.2.5), can be run.
  ```

## Page 123 / "References" (line 2019)
- Classification: uncertain boundary
- Before (last line before the block):
  ```
  - [10] P. Maymounkov, D. Mazières, Kademlia: A peer-to-peer information system based on the XOR metric, in IPTPS '01 revised papers from the First International Workshop on Peer-to-Peer Systems,
  ```
- After (first line after the block):
  ```
  p. 53–65, available at http://pdos.csail.mit.edu/~petar/papers/ maymounkov-kademlia-lncs.pdf, 2002.
  ```
