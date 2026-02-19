import * as anchor from '@coral-xyz/anchor';

module.exports = async function (provider: anchor.AnchorProvider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add your deploy script here.
  console.log("CPI Example program deployed successfully!");
};