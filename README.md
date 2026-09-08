Open Network X

Open Network X (ONX) is an independent blockchain implementation project inspired by the architecture and technical vision described in the original Telegram Open Network (TON) white paper.

ONX is an attempt to explore, reconstruct, and implement that vision independently and from first principles.

It is not the TON blockchain, is not an official continuation of TON, and is not intended to replace the existing TON network or its community.

The native currency of Open Network X is Onyx.

---

What Is Open Network X?

The original TON design describes a highly scalable, decentralized blockchain architecture built around concepts including:

- Masterchains
- Workchains
- Shardchains
- Dynamic sharding
- Asynchronous message passing
- Validator networks
- Byzantine fault-tolerant consensus
- Smart contracts
- A specialized virtual machine
- Distributed storage
- Scalable blockchain infrastructure

Open Network X exists to investigate what it would look like to implement that architecture independently.

The guiding question is:

«What would the Open Network look like if we went back to the original design and built an independent implementation from the specification?»

ONX is therefore not intended to be a conventional fork.

We are not taking an existing implementation, changing its name, and continuing from there.

Instead, the project begins with the protocol's published design and works forward toward an independent implementation.

---

Independence

Open Network X is a separate project.

ONX:

- Does not claim to be TON.
- Does not claim to represent the TON Foundation, TON Society, or the TON community.
- Does not attempt to replace the existing TON network.
- Does not require the existing TON network to function.
- Does not treat the current TON implementation as the authoritative specification for ONX.
- May make independent technical decisions where the original specification is ambiguous or incomplete.

Similarity between ONX and existing TON architecture is intentional where that similarity follows from the original protocol design.

---

The White Paper Is the Starting Point

The original TON white paper is the primary historical and architectural reference for this project.

It is included in this repository as a reference document.

ONX should not modify the white paper to make the implementation easier.

Instead:

the implementation should adapt to the specification.

Where the white paper is precise, ONX should strive for faithful implementation.

Where the white paper is ambiguous, ONX must document its interpretation.

Where the white paper does not provide sufficient information, ONX must explicitly identify the missing information and document the engineering decision that fills the gap.

---

Onyx

Onyx is the native currency of Open Network X.

The currency exists as part of the ONX protocol rather than as a separate application-layer token.

The exact monetary policy, denomination system, issuance mechanism, validator economics, transaction fees, and other economic parameters will be specified as the protocol develops.

---

Project Philosophy

ONX follows several principles.

1. Specification before implementation

We begin with the protocol design, not with existing source code.

2. Independent implementation

Existing implementations may be studied for educational and interoperability research purposes, but ONX is intended to be independently implemented.

3. Explicit decisions

When the reference material does not provide an answer, the project records the decision instead of silently inventing behavior.

4. Testable protocol behavior

Important protocol properties should eventually have deterministic tests.

5. No accidental compatibility

ONX should not inherit compatibility with another network merely because doing so is convenient.

Compatibility must be an intentional protocol decision.

6. Transparency

Architectural deviations, interpretations, limitations, and known incompatibilities should be documented openly.

---

Project Status

Early research / architecture phase.

The initial goal is to establish the protocol specification and engineering principles before attempting to build a production blockchain.

Nothing in this repository should currently be interpreted as production-ready blockchain infrastructure.

---

Initial Repository Structure

open-network-x/
├── README.md
├── INSTRUCTIONS.md
└── whitepaper/
    └── TON-Whitepaper.pdf

As implementation begins, the repository will expand around the protocol specification derived from the reference material.

---

Long-Term Goal

The long-term goal is to develop an independent, functioning blockchain network that faithfully implements the core architecture described by the original TON design while maintaining a distinct identity, implementation, network, and ecosystem.

ONX should ultimately be able to stand on its own.

The project does not need to replace TON to be successful.

It needs to demonstrate what an independent implementation of the underlying architectural vision can become.

---

Disclaimer

Open Network X is an independent project.

ONX, Open Network X, and Onyx should not be represented as official TON products, networks, or services.

The use of historical TON technical material as a reference does not imply endorsement, affiliation, or control by the organizations or communities associated with the existing TON ecosystem.

---

License

Licensing will be established before substantial implementation is distributed.

Individual reference materials may have their own copyright and licensing requirements. See the "whitepaper/" directory for the applicable source and attribution information.
