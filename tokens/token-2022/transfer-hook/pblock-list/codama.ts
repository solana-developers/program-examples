import { renderJavaScriptUmiVisitor, renderJavaScriptVisitor, renderRustVisitor } from '@codama/renderers';
import { accountLinkNode, accountNode, booleanTypeNode, booleanValueNode, constantDiscriminatorNode, constantPdaSeedNodeFromString, constantValueNode, createFromRoot, instructionAccountLinkNode, instructionAccountNode, instructionArgumentNode, instructionNode, numberTypeNode, numberValueNode, optionTypeNode, pdaLinkNode, pdaNode, pdaSeedValueNode, pdaValueNode, programNode, publicKeyTypeNode, publicKeyValueNode, resolverValueNode, rootNode, sizeDiscriminatorNode, sizePrefixTypeNode, stringTypeNode, stringValueNode, structFieldTypeNode, structTypeNode, variablePdaSeedNode } from "codama"
import path from "path";
import fs from "fs";

const rustClientsDir = path.join(__dirname, "..", "sdk", "rust");
const typescriptClientsDir = path.join(
  __dirname,
  "..",
  "sdk",
  "ts",
);

const root = rootNode(
    programNode({
        name: "block-list",
        publicKey: "BLoCKLSG2qMQ9YxEyrrKKAQzthvW4Lu8Eyv74axF6mf",
        version: "1.0.0",
        accounts: [
          accountNode({
            name: "config",
            discriminators: [
              constantDiscriminatorNode(constantValueNode(numberTypeNode("u8"), numberValueNode(0))),
            ],
            size: 41,
            pda: pdaLinkNode("config"),
            docs: ["The config PDA account"],
            data: structTypeNode([
              structFieldTypeNode({
                name: "discriminator",
                type: numberTypeNode("u8"),
                defaultValueStrategy: "omitted",
              }),
              structFieldTypeNode({
                name: "authority",
                type: publicKeyTypeNode(),
              }),
              structFieldTypeNode({
                name: "blocked_wallets_count",
                type: numberTypeNode("u64"),
              }),
            ]),
          }),
          accountNode({
            name: "walletBlock",
            discriminators: [
              constantDiscriminatorNode(constantValueNode(numberTypeNode("u8"), numberValueNode(1))),
            ],
            size: 33,
            pda: pdaLinkNode("walletBlock"),
            docs: ["The config PDA account"],
            data: structTypeNode([
              structFieldTypeNode({
                name: "authority",
                type: publicKeyTypeNode(),
              }),
            ])
          }),
          accountNode({
            name: "extraMetas",
            pda: pdaLinkNode("extraMetas"),
            docs: ["The extra metas PDA account"],
          })
        ],
        instructions: [
          instructionNode({
            name: "init",
            arguments: [
              instructionArgumentNode({
                  name: 'discriminator',
                  type: numberTypeNode('u8'),
                  defaultValue: numberValueNode(0xF1),
                  defaultValueStrategy: 'omitted',
              }),
            ],
            accounts: [
              instructionAccountNode({
                name: "authority",
                isSigner: true,
                isWritable: true,
              }),
              instructionAccountNode({
                name: "config",
                isSigner: false,
                isWritable: true,
                defaultValue: pdaValueNode(pdaLinkNode("config"))
              }),
              instructionAccountNode({
                name: "systemProgram",
                defaultValue: publicKeyValueNode("11111111111111111111111111111111", "systemProgram"),
                isSigner: false,
                isWritable: false,
              })
            ],
            discriminators: [
              constantDiscriminatorNode(constantValueNode(numberTypeNode("u8"), numberValueNode(0xF1))),
            ],
            docs: ["Initialize the config PDA account"],
          }),
          instructionNode({
            name: "blockWallet",
            arguments: [
              instructionArgumentNode({
                  name: 'discriminator',
                  type: numberTypeNode('u8'),
                  defaultValue: numberValueNode(0xF2),
                  defaultValueStrategy: 'omitted',
              }),
            ],
            accounts: [
              instructionAccountNode({
                name: "authority",
                isSigner: true,
                isWritable: true,
              }),
              instructionAccountNode({
                name: "config",
                isSigner: false,
                isWritable: true,
                defaultValue: pdaValueNode(pdaLinkNode("config"))
              }),
              instructionAccountNode({
                name: "wallet",
                isSigner: false,
                isWritable: false,
              }),
              instructionAccountNode({
                name: "walletBlock",
                isSigner: false,
                isWritable: true,
              }),
              instructionAccountNode({
                name: "systemProgram",
                defaultValue: publicKeyValueNode("11111111111111111111111111111111", "systemProgram"),
                isSigner: false,
                isWritable: false,
              })
            ],
            discriminators: [
              constantDiscriminatorNode(constantValueNode(numberTypeNode("u8"), numberValueNode(0xF2))),
            ],
            docs: ["Block a wallet"],
          }),
          instructionNode({
            name: "unblockWallet",
            arguments: [
              instructionArgumentNode({
                  name: 'discriminator',
                  type: numberTypeNode('u8'),
                  defaultValue: numberValueNode(0xF3),
                  defaultValueStrategy: 'omitted',
              }),
            ],
            accounts: [
              instructionAccountNode({
                name: "authority",
                isSigner: true,
                isWritable: true,
              }),
              instructionAccountNode({
                name: "config",
                isSigner: false,
                isWritable: true,
                defaultValue: pdaValueNode(pdaLinkNode("config"))
              }),
              instructionAccountNode({
                name: "walletBlock",
                isSigner: false,
                isWritable: true,
              }),
              instructionAccountNode({
                name: "systemProgram",
                defaultValue: publicKeyValueNode("11111111111111111111111111111111", "systemProgram"),
                isSigner: false,
                isWritable: false,
              })
            ],
            discriminators: [
              constantDiscriminatorNode(constantValueNode(numberTypeNode("u8"), numberValueNode(0xF3))),
            ],
            docs: ["Unblock a wallet"],
          }),
          instructionNode({
            name: "setupExtraMetas",
            arguments: [
              instructionArgumentNode({
                  name: 'discriminator',
                  type: numberTypeNode('u8'),
                  defaultValue: numberValueNode(0x6A),
                  defaultValueStrategy: 'omitted',
              }),
              instructionArgumentNode({
                name: 'checkBothWallets',
                type: booleanTypeNode(),
                defaultValue: booleanValueNode(false),
                defaultValueStrategy: 'optional',
              }),
            ],
            accounts: [
              instructionAccountNode({
                name: "authority",
                isSigner: true,
                isWritable: true,
              }),
              instructionAccountNode({
                name: "config",
                isSigner: false,
                isWritable: false,
                defaultValue: pdaValueNode(pdaLinkNode("config"))
              }),
              instructionAccountNode({
                name: "mint",
                isSigner: false,
                isWritable: false,
              }),
              instructionAccountNode({
                name: "extraMetas",
                isSigner: false,
                isWritable: true,
                defaultValue: pdaValueNode(pdaLinkNode("extraMetas"))
              }),
              instructionAccountNode({
                name: "systemProgram",
                defaultValue: publicKeyValueNode("11111111111111111111111111111111", "systemProgram"),
                isSigner: false,
                isWritable: false,
              })
            ],
            discriminators: [
              constantDiscriminatorNode(constantValueNode(numberTypeNode("u8"), numberValueNode(0x6A))),
            ],
            docs: ["Unblock a wallet"],
          }),
        ],
        pdas: [
          pdaNode({
            name: "config",
            seeds: [constantPdaSeedNodeFromString("utf8", "config")],
            docs: ["The config PDA account"],
          }),
          pdaNode({
            name: "walletBlock",
            seeds: [
              constantPdaSeedNodeFromString("utf8", "wallet_block"),
              variablePdaSeedNode("wallet", publicKeyTypeNode()),
            ],
            docs: ["The wallet block PDA account"],
          }),
          pdaNode({
            name: "extraMetas",
            seeds: [
              constantPdaSeedNodeFromString("utf8", "extra-account-metas"),
              variablePdaSeedNode("mint", publicKeyTypeNode()),
            ],
            docs: ["The extra metas PDA account"],
          }),
        ]
    })
);


function preserveConfigFiles() {
    const filesToPreserve = ['package.json', 'tsconfig.json', '.npmignore', 'pnpm-lock.yaml', 'Cargo.toml'];
    const preservedFiles = new Map();
    
    filesToPreserve.forEach(filename => {
      const filePath = path.join(typescriptClientsDir, filename);
      const tempPath = path.join(typescriptClientsDir, `${filename}.temp`);
      
      if (fs.existsSync(filePath)) {
        fs.copyFileSync(filePath, tempPath);
        preservedFiles.set(filename, tempPath);
      }
    });
    
    return {
      restore: () => {
        preservedFiles.forEach((tempPath, filename) => {
          const filePath = path.join(typescriptClientsDir, filename);
          if (fs.existsSync(tempPath)) {
            fs.copyFileSync(tempPath, filePath);
            fs.unlinkSync(tempPath);
          }
        });
      }
    };
  }

const codama = createFromRoot(root)

const configPreserver = preserveConfigFiles();

codama.accept(renderJavaScriptVisitor('sdk/ts/src', { formatCode: true }));
codama.accept(renderRustVisitor('sdk/rust/src/client', { crateFolder: 'sdk/rust/', formatCode: true }));
