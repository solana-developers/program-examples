# Transfer Hook: Metadata-Based Transfer Disabling

## Overview

This project demonstrates a proof of concept for a Solana transfer hook that disables token transfers based on a flag set in the mint's metadata extension. This mechanism allows for dynamic control over token transferability without modifying the token program itself or requiring extra PDAs to store the current transferability state.

## Key Features

- Custom transfer hook program
- Utilization of mint metadata extensions
- Dynamic transfer disabling based on metadata flag ("disable-transfers")
- No extra PDAs needed to store the current transferability state

## Future Improvements

Once [PR #7099](https://github.com/solana-labs/solana-program-library/pull/7099) in the Solana Program Library is merged and deployed to mainnet, this implementation can be further simplified:

- The extra account metas will no longer be required.
- This will reduce the complexity of clients, providing some backwards compatibility



## How It Works

1. The mint's metadata extension includes a boolean flag indicating whether transfers are allowed.
2. When a transfer is initiated, the transfer hook program is invoked.
3. The program checks the mint's metadata extension for the transfer permission flag.
4. If transfers are disabled, the program returns an error, preventing the transfer.
5. If transfers are enabled, the program allows the transfer to proceed.

## Key Instructions

The program includes two main instructions for controlling the transfer flag:

1. `stop_transfer`: This instruction sets the "disable-transfers" flag in the mint's metadata to "true", effectively disabling transfers for the token.

2. `resume_transfer`: This instruction sets the "disable-transfers" flag in the mint's metadata to "false", re-enabling transfers for the token.

These instructions allow for dynamic control over token transferability without needing to modify the token program or maintain separate state accounts.


## Use Cases

- Temporary transfer freezing for regulatory compliance
- Phased token distribution with time-based transfer enabling
- Conditional transferability based on external factors

## Limitations and Considerations

- This is a proof of concept and may require further security audits for production use.
- Ensure proper access controls for modifying the metadata extension flag.


## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

