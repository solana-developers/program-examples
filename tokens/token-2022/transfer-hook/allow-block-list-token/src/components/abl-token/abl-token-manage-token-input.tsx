'use client'

import { useWallet } from '@solana/wallet-adapter-react'
import { WalletButton } from '../solana/solana-provider'

import { redirect } from 'next/navigation'
import React from 'react'
import { Button } from '@/components/ui/button'

export default function ManageTokenInput() {
  const { publicKey } = useWallet()
  const [tokenAddress, setTokenAddress] = React.useState('')

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (tokenAddress) {
      redirect(`/manage-token/${tokenAddress.toString()}`)
    }
  }

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
    <div className="hero py-[64px]">
      <div className="hero-content">
        <form onSubmit={handleSubmit} className="w-full max-w-md">
          <div className="space-y-4">
            <label className="block">
              Token Address
              <input
                type="text"
                className="w-full p-2 border rounded"
                value={tokenAddress}
                onChange={e => setTokenAddress(e.target.value)}
                placeholder="Enter token address"
                required
              />
            </label>
            <Button type="submit">
              Manage Token
            </Button>
          </div>
        </form>
      </div>
    </div>
  )
}
