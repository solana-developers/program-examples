export type CompressedNft = {
  version: "0.3.1";
  name: "compressed_nft";
  instructions: [
    {
      name: "new";
      accounts: [
        {
          name: "dataAccount";
          isMut: true;
          isSigner: false;
          isOptional: false;
        },
        {
          name: "payer";
          isMut: true;
          isSigner: true;
          isOptional: false;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
          isOptional: false;
        }
      ];
      args: [
        {
          name: "bump";
          type: {
            array: ["u8", 1];
          };
        }
      ];
    },
    {
      name: "mint";
      accounts: [
        {
          name: "dataAccount";
          isMut: true;
          isSigner: false;
          isOptional: false;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
          isOptional: false;
        }
      ];
      args: [
        {
          name: "treeAuthority";
          type: "publicKey";
        },
        {
          name: "leafOwner";
          type: "publicKey";
        },
        {
          name: "leafDelegate";
          type: "publicKey";
        },
        {
          name: "merkleTree";
          type: "publicKey";
        },
        {
          name: "payer";
          type: "publicKey";
        },
        {
          name: "treeDelegate";
          type: "publicKey";
        },
        {
          name: "uri";
          type: "string";
        }
      ];
    }
  ];
  metadata: {
    address: "BhDH6TLEnf4dLq9hLn2gLwm5rJdj8Cbdc9ZrsjUpL7kB";
  };
};

export const IDL: CompressedNft = {
  version: "0.3.1",
  name: "compressed_nft",
  instructions: [
    {
      name: "new",
      accounts: [
        {
          name: "dataAccount",
          isMut: true,
          isSigner: false,
          isOptional: false,
        },
        {
          name: "payer",
          isMut: true,
          isSigner: true,
          isOptional: false,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
          isOptional: false,
        },
      ],
      args: [
        {
          name: "bump",
          type: {
            array: ["u8", 1],
          },
        },
      ],
    },
    {
      name: "mint",
      accounts: [
        {
          name: "dataAccount",
          isMut: true,
          isSigner: false,
          isOptional: false,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
          isOptional: false,
        },
      ],
      args: [
        {
          name: "treeAuthority",
          type: "publicKey",
        },
        {
          name: "leafOwner",
          type: "publicKey",
        },
        {
          name: "leafDelegate",
          type: "publicKey",
        },
        {
          name: "merkleTree",
          type: "publicKey",
        },
        {
          name: "payer",
          type: "publicKey",
        },
        {
          name: "treeDelegate",
          type: "publicKey",
        },
        {
          name: "uri",
          type: "string",
        },
      ],
    },
  ],
  metadata: {
    address: "BhDH6TLEnf4dLq9hLn2gLwm5rJdj8Cbdc9ZrsjUpL7kB",
  },
};
