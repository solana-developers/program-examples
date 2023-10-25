export type CompressedNft = {
  "version": "0.0.1",
  "name": "compressed_nft",
  "instructions": [
    {
      "name": "new",
      "accounts": [
        {
          "name": "dataAccount",
          "isMut": true,
          "isSigner": false,
          "isOptional": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "isOptional": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "isOptional": false
        }
      ],
      "args": [
        {
          "name": "bump",
          "type": {
            "array": [
              "u8",
              1
            ]
          }
        }
      ]
    },
    {
      "name": "mint",
      "accounts": [
        {
          "name": "tree_authority",
          "isMut": true,
          "isSigner": false,
          "isOptional": false
        },
        {
          "name": "leaf_owner",
          "isMut": false,
          "isSigner": false,
          "isOptional": false
        },
        {
          "name": "leaf_delegate",
          "isMut": false,
          "isSigner": false,
          "isOptional": false
        },
        {
          "name": "merkle_tree",
          "isMut": true,
          "isSigner": false,
          "isOptional": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "isOptional": false
        },
        {
          "name": "tree_delegate",
          "isMut": true,
          "isSigner": true,
          "isOptional": false
        },
        {
          "name": "noop_address",
          "isMut": false,
          "isSigner": false,
          "isOptional": false
        },
        {
          "name": "compression_pid",
          "isMut": false,
          "isSigner": false,
          "isOptional": false
        },
        {
          "name": "bubblegum_pid",
          "isMut": false,
          "isSigner": false,
          "isOptional": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "isOptional": false
        }
      ],
      "args": [
        {
          "name": "uri",
          "type": "string"
        }
      ]
    }
  ],
  "metadata": {
    "address": "BvgEJTPXfriGPopjJr1nLc4vADXm7A7TqjLFVztpd19Q"
  }
};

export const IDL: CompressedNft = {
  "version": "0.0.1",
  "name": "compressed_nft",
  "instructions": [
    {
      "name": "new",
      "accounts": [
        {
          "name": "dataAccount",
          "isMut": true,
          "isSigner": false,
          "isOptional": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "isOptional": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "isOptional": false
        }
      ],
      "args": [
        {
          "name": "bump",
          "type": {
            "array": [
              "u8",
              1
            ]
          }
        }
      ]
    },
    {
      "name": "mint",
      "accounts": [
        {
          "name": "tree_authority",
          "isMut": true,
          "isSigner": false,
          "isOptional": false
        },
        {
          "name": "leaf_owner",
          "isMut": false,
          "isSigner": false,
          "isOptional": false
        },
        {
          "name": "leaf_delegate",
          "isMut": false,
          "isSigner": false,
          "isOptional": false
        },
        {
          "name": "merkle_tree",
          "isMut": true,
          "isSigner": false,
          "isOptional": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "isOptional": false
        },
        {
          "name": "tree_delegate",
          "isMut": true,
          "isSigner": true,
          "isOptional": false
        },
        {
          "name": "noop_address",
          "isMut": false,
          "isSigner": false,
          "isOptional": false
        },
        {
          "name": "compression_pid",
          "isMut": false,
          "isSigner": false,
          "isOptional": false
        },
        {
          "name": "bubblegum_pid",
          "isMut": false,
          "isSigner": false,
          "isOptional": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "isOptional": false
        }
      ],
      "args": [
        {
          "name": "uri",
          "type": "string"
        }
      ]
    }
  ],
  "metadata": {
    "address": "BvgEJTPXfriGPopjJr1nLc4vADXm7A7TqjLFVztpd19Q"
  }
};
