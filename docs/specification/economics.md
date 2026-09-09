# ONX Specification — Economics

**Status:** Draft

## 1. Reference

- `whitepaper.md` Appendix A: denomination, initial supply, validator rewards, and slashing/burn examples.
- `docs/specification/consensus.md`: stake, rewards, and slashing authority.

## 2. Requirement

Economic values must be explicit ONX decisions rather than implicit inheritance from a historical reference.

## 3. ONX decisions

| Reference figure | ONX decision | Rationale |
| --- | --- | --- |
| $10^9$ base-unit subdivision | **Accept** | `amount_nanos` already names the base unit and uses it canonically. |
| $2^{-16}$ “speck” gas rounding | **Reject** | Gas accounting is integer base units until the execution instruction-set and price table define a demonstrated need for fractional accounting. |
| Initial supply cap | **Defer** | No governance/issuance authority or security budget is specified; no supply is authorized by this draft. |
| Percentage-of-stake validator rewards | **Defer** | Consensus establishes eligibility, but the reward rate must follow a defined issuance and fee policy. |
| Partial burn of slashed stakes | **Defer** | Consensus may slash proven misconduct, but burn/reward allocation needs a complete treasury policy. |

The accepted subdivision is representation only; it does not authorize issuance, fees, rewards, staking returns, or burns. This sequencing follows the Consensus dependency and Appendix A's illustrative role.

## 4. Serialization

Economic configuration is a versioned masterchain object. Each enabled parameter uses an integer base-unit numerator/denominator and an activation height. Unspecified/deferred parameters have no wire value and no implementation behavior.

## 5. Malformed-input behavior

Reject zero denominators, duplicate parameters, activation regressions, values outside their declared integer range, and blocks applying an economic parameter before activation.

## 6. Test plan

Test denomination conversion without rounding loss, config canonicalization, parameter activation boundaries, and that all deferred parameters remain disabled.
