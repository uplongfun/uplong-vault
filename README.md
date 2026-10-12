# upLong

The on-chain vault behind [uplong](https://uplong.fun): it holds traders' USDC and the $LONG reward pool, and pays them out.

- Program ID: `LongtNv8Jfv4EBVwRfoU67iaT9vcD2Sk1FF3XYi2zSt`
- $LONG mint: `LoNGFLAHbkNYz5CfLy3x7vE1jQjz4LoSNdow41CJ9qm`
- Website: https://uplong.fun
- Twitter: https://x.com/upLongFun

## What it does

Trading on uplong runs on the uplong chain. This program holds the money behind it, in two token accounts owned by the program.

**USDC.** Deposits are sent to the vault. Withdrawals leave only through `pay`, which the operator calls for a withdrawal the uplong chain approved. Each withdrawal has a 32-byte id with its own receipt account, so a withdrawal can't be paid twice.

**$LONG.** Traders earn $LONG for their losses on uplong. The operator calls `distribute` with a trader's running total, and the program pays only the part not paid before. Total payouts can never exceed the cap set at `init_long`.

## Roles

| Role | Can do |
|---|---|
| Operator | `pay` (USDC withdrawals) and `distribute` ($LONG rewards), and nothing else |
| Admin | Change the operator, pause and resume all payouts, set the 24-hour limits, hand the admin role on (`set_admin`) |

Nobody, including the admin, can move vault tokens except through `pay` and `distribute`.

## Instructions

| Instruction | Signer | Effect |
|---|---|---|
| `initialize(operator)` | admin | Creates the config and names the admin and operator |
| `init_usdc(daily_limit)` | admin | Sets the USDC mint and creates the USDC vault (once) |
| `init_long(cap, daily_limit)` | admin | Sets the $LONG mint and creates the $LONG vault (once) |
| `pay(id, amount)` | operator | Pays withdrawal `id` to the user and records a receipt |
| `distribute(earned)` | operator | Pays the user's $LONG up to `earned` |
| `set_operator`, `set_paused`, `set_usdc_daily_limit`, `set_long_daily_limit`, `set_admin` | admin | Settings |

Every instruction emits an event, so all payouts and settings changes are public.

## Accounts

| PDA | Seeds |
|---|---|
| Config | `["config"]` |
| USDC vault | `["usdc_vault"]` |
| $LONG vault | `["long_vault"]` |
| Withdrawal receipt | `["receipt", id]` |
| $LONG recipient | `["recipient", user]` |

Both mints may be SPL Token or Token-2022. The $LONG mint is Token-2022, so transfers use `transfer_checked` through the token interface.

## Safety limits

- **Receipts:** a withdrawal id can be paid once. A second `pay` with the same id fails.
- **Cap:** $LONG payouts can never exceed the cap set at `init_long`.
- **Pause:** the admin can stop all payouts at once.
- **24-hour limits:** each vault can have a rolling 24-hour payout limit, set by the admin. When one is set, a stolen operator key can move at most one day's limit before the admin pauses the vault.

## Build

```sh
cargo build-sbf --manifest-path programs/uplong-vault/Cargo.toml --arch v3
```

## Verify

The deployed program is built with [solana-verify](https://github.com/Ellipsis-Labs/solana-verifiable-build), so anyone can rebuild it from this repository and compare.

```sh
# Hash of the program on mainnet
solana-verify get-program-hash -u https://api.mainnet-beta.solana.com LongtNv8Jfv4EBVwRfoU67iaT9vcD2Sk1FF3XYi2zSt

# Rebuild this repository in Docker and hash the result
solana-verify build --library-name uplong_vault --arch v3
solana-verify get-executable-hash target/deploy/uplong_vault.so
```

The two hashes match when the deployed program was built from this code.
