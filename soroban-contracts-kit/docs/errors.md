# Error codes

## escrow

| Code | Name | Meaning |
|---|---|---|
| 1 | AlreadyInitialized | `init` was already called |
| 2 | InvalidAmount | amount is zero or negative |
| 3 | NotInitialized | contract has not been initialized |
| 4 | NotFunded | escrow was already released or refunded |
| 5 | Unauthorized | caller is not allowed to perform this action |

## vesting

| Code | Name | Meaning |
|---|---|---|
| 1 | AlreadyInitialized | `init` was already called |
| 2 | InvalidAmount | total is zero or negative |
| 3 | InvalidSchedule | duration is 0 or cliff is longer than duration |
| 4 | NotInitialized | contract has not been initialized |
| 5 | NothingToClaim | no tokens are claimable right now |
| 6 | MathOverflow | vesting calculation overflowed |
