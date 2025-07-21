'use client'

import { useWallet } from '@solana/wallet-adapter-react'
import { WalletButton } from '../solana/solana-provider'
import { useParams } from 'next/navigation'
import React from 'react'
import { useAblTokenProgram, useHasTransferHookEnabled } from './abl-token-data-access'
import { PublicKey } from '@solana/web3.js'
import { PermanentDelegate } from '@solana/spl-token'
import { BN } from '@coral-xyz/anchor'
import { Button } from '@/components/ui/button'

interface TokenInfo {
  address: string;
  name: string | undefined;
  symbol: string | undefined;
  uri: string | undefined;
  decimals: number;
  supply: number;
  mintAuthority: PublicKey | null;
  freezeAuthority: PublicKey | null;
  permanentDelegate: PermanentDelegate | null;
}

function TokenInfo({ tokenInfo }: { tokenInfo: TokenInfo | null }) {
  return (
    <div className="bg-base-200 p-6 rounded-lg">
      <h2 className="text-2xl font-bold mb-4">Token Information</h2>
      {tokenInfo ? (
        <div className="grid grid-cols-2 gap-4">
          <div>Address: {tokenInfo.address}</div>
          <div>Name: {tokenInfo.name}</div>
          <div>Symbol: {tokenInfo.symbol}</div>
          <div>Decimals: {tokenInfo.decimals}</div>
          <div>URI: {tokenInfo.uri}</div>
          <div>Supply: {tokenInfo.supply}</div>
          <div>Mint Authority: {tokenInfo.mintAuthority?.toString()}</div>
          <div>Freeze Authority: {tokenInfo.freezeAuthority?.toString()}</div>
          <div>Permanent Delegate: {tokenInfo.permanentDelegate?.delegate.toString()}</div>
        </div>
      ) : (
        <p>No token information available.</p>
      )}
    </div>
  )
}

function TokenManagement({ tokenInfo }: { tokenInfo: TokenInfo }) {
  const { publicKey } = useWallet()
  const { changeMode, mintTo, attachToExistingToken } = useAblTokenProgram()
  const [mode, setMode] = React.useState<'allow' | 'block' | 'mixed'>('allow')
  const [threshold, setThreshold] = React.useState('100000')
  const [destinationWallet, setDestinationWallet] = React.useState('')
  const hasTransferHookEnabled = useHasTransferHookEnabled(new PublicKey(tokenInfo.address))
  const [name, setName] = React.useState<string>('')
  const [symbol, setSymbol] = React.useState<string>('')
  const [uri, setUri] = React.useState<string>('')


  const handleApplyChanges = async () => {
    if (!publicKey || !tokenInfo) return;

    try {
      await changeMode.mutateAsync({
        mode,
        threshold: new BN(threshold),
        mint: new PublicKey(tokenInfo.address),
      });
    } catch (err) {
      console.error('Failed to apply changes:', err);
    }
  };

  const setTransferHook = async () => {
    if (!publicKey || !tokenInfo) return;

    try {
      await attachToExistingToken.mutateAsync({
        mint: new PublicKey(tokenInfo.address),
        mode,
        threshold: new BN(threshold),
        name,
        symbol,
        uri,
      });
    } catch (err) {
      console.error('Failed to set transfer hook:', err);
    }
  };

  const [mintAmount, setMintAmount] = React.useState('0')

  const handleMint = async () => {
    if (!publicKey || !tokenInfo) return;

    try {
      await mintTo.mutateAsync({
        mint: new PublicKey(tokenInfo.address),
        amount: new BN(mintAmount),
        recipient: publicKey,
      });
      console.log('Minted successfully');
    } catch (err) {
      console.error('Failed to mint tokens:', err);
    }
  };

  return (
    <div className="bg-base-200 p-6 rounded-lg">
      <h2 className="text-2xl font-bold mb-4">Token Management</h2>
      <div className="space-y-4">
        <div>
          {hasTransferHookEnabled.data ? (
            <div>
              <label className="block mb-2">Mode</label>
              <div className="flex gap-4">
                <label>
                  <input
                    type="radio"
                    checked={mode === 'allow'}
                    onChange={() => setMode('allow')}
                    name="mode"
                  /> Allow
                </label>
                <label>
                  <input
                    type="radio"
                    checked={mode === 'block'}
                    onChange={() => setMode('block')}
                    name="mode"
                  /> Block
                </label>
                <label>
                  <input
                    type="radio"
                    checked={mode === 'mixed'}
                    onChange={() => setMode('mixed')}
                    name="mode"
                  /> Mixed
                </label>
              </div>

              {mode === 'mixed' && (
              <div>
                <label className="block mb-2">Threshold Amount</label>
                <input
                  type="number"
                  className="w-full p-2 border rounded"
                  value={threshold}
                  onChange={e => setThreshold(e.target.value)}
                  min="0"
                />
              </div>
              )}

              <div className="mt-4">
                <Button onClick={handleApplyChanges}>
                  Apply Changes
                </Button>
              </div>
              
            </div>
          ) : (
            <div>
            <div className="space-y-4">
              <div>
                <label className="block mb-2">Name (Optional)</label>
                <input
                  type="text"
                  className="w-full p-2 border rounded"
                  value={name}
                  onChange={e => setName(e.target.value)}
                  placeholder="Enter token name"
                />
              </div>
              <div>
                <label className="block mb-2">Symbol (Optional)</label>
                <input
                  type="text"
                  className="w-full p-2 border rounded"
                  value={symbol}
                  onChange={e => setSymbol(e.target.value)}
                  placeholder="Enter token symbol"
                />
              </div>
              <div>
                <label className="block mb-2">URI (Optional)</label>
                <input
                  type="text"
                  className="w-full p-2 border rounded"
                  value={uri}
                  onChange={e => setUri(e.target.value)}
                  placeholder="Enter token URI"
                />
              </div>
            </div>
            <div className="mt-4">
              <Button onClick={setTransferHook}>
                Set Transfer hook
              </Button>
            </div>
            </div>
          )}
        </div>

        <div className="mt-8">
          <h3 className="text-xl font-bold mb-2">Mint New Tokens</h3>
          <div className="space-y-4">
            <div>
              <label className="block mb-2">Destination Wallet</label>
              <input
                type="text"
                className="w-full p-2 border rounded"
                value={destinationWallet}
                onChange={e => setDestinationWallet(e.target.value)}
                placeholder="Enter destination wallet address"
              />
            </div>
            <div className="flex items-center gap-4">
              <input
                type="number"
                className="w-full p-2 border rounded"
                value={mintAmount}
                onChange={e => setMintAmount(e.target.value)}
                min="0"
                placeholder="Amount to mint"
              />
              <Button onClick={handleMint}>
                Mint Tokens
              </Button>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default function ManageTokenDetail() {
  const { publicKey } = useWallet()
  const { getToken } = useAblTokenProgram()
  const params = useParams()
  const tokenAddress = params?.address as string

  const tokenQuery = getToken(new PublicKey(tokenAddress));

  const tokenInfo = React.useMemo(() => {
    if (!tokenQuery?.data || !tokenAddress) return null;
    return {
      ...tokenQuery.data,
      address: tokenAddress,
      supply: 0, // TODO: Get supply from token account
    };
  }, [tokenQuery?.data, tokenAddress]);

  if (!publicKey) {
    return (
      <div className="hero py-[64px]">
        <div className="hero-content text-center">
          <WalletButton />
        </div>
      </div>
    )
  }

  return (
    <div className="space-y-8">
      {tokenQuery?.isLoading ? (
        <p>Loading token information...</p>
      ) : tokenQuery?.isError ? (
        <p>Error loading token information. Please check the token address.</p>
      ) : (
        <>
          <TokenInfo tokenInfo={tokenInfo} />
          {tokenInfo && <TokenManagement tokenInfo={tokenInfo}/>}
          
        </>
      )}
    </div>
  )
}
