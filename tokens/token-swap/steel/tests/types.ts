import { struct, u16 } from '@solana/buffer-layout';
import { publicKey, u64 } from '@solana/buffer-layout-utils';
import { PublicKey } from '@solana/web3.js';

export interface Amm {
  disc: bigint;
  id: PublicKey;
  admin: PublicKey;
  fee: number;
}
export interface Pool {
  disc: bigint;
  amm: PublicKey;
  mintA: PublicKey;
  mintB: PublicKey;
}
export interface TokenAccount {
  mint: PublicKey;
  owner: PublicKey;
  amount: bigint;
  delegateOption: 1 | 0;
  delegate: PublicKey;
  isNativeOption: 1 | 0;
  isNative: bigint;
  delegateAmount: bigint;
  closeAuthorityOption: 1 | 0;
  closeAuthority: PublicKey;
}

interface Offer {
  disc: bigint;
  id: bigint;
  maker: PublicKey;
  tokenMintA: PublicKey;
  tokenMintB: PublicKey;
  tokenBWantedAmount: bigint;
}

export const TokenLayout = struct<TokenAccount>([
  publicKey('mint'),
  publicKey('owner'),
  u64('amount'),
  u64('delegateOption'),
  publicKey('delegate'),
  u64('isNativeOption'),
  u64('isNative'),
  u64('delegateAmount'),
  u64('closeAuthorityOption'),
  publicKey('closeAuthority'),
]);

export const OfferLayout = struct<Offer>([
  u64('disc'),
  u64('id'),
  publicKey('maker'),
  publicKey('tokenMintA'),
  publicKey('tokenMintB'),
  u64('tokenBWantedAmount'),
]);

export const AmmLayout = struct<Amm>([u64('disc'), publicKey('id'), publicKey('admin'), u16('fee')]);

export const PoolLayout = struct<Pool>([u64('disc'), publicKey('amm'), publicKey('mintA'), publicKey('mintB')]);
// Example of decoding
export function decodeAccount(data) {
  return TokenLayout.decode(data);
}
export function decodeOffer(data) {
  return OfferLayout.decode(data);
}
