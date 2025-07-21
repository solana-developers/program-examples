'use client'

import { useWallet } from '@solana/wallet-adapter-react'
import { ExplorerLink } from '../cluster/cluster-ui'
import { WalletButton } from '../solana/solana-provider'
import { useAblTokenProgram } from './abl-token-data-access'
import { AblTokenCreate, AblTokenProgram } from './abl-token-ui'
import { AppHero } from '../app-hero'
import { ellipsify } from '@/lib/utils'

export default function AblTokenFeature() {
  const { publicKey } = useWallet()
  const { programId } = useAblTokenProgram()

  return publicKey ? (
    <div>
      <AppHero title="Create New Token">
        <p className="mb-6">
          <ExplorerLink path={`account/${programId}`} label={ellipsify(programId.toString())} />
        </p>
        <AblTokenCreate />
      </AppHero>
    </div>
  ) : (
    <div className="max-w-4xl mx-auto">
      <div className="hero py-[64px]">
        <div className="hero-content text-center">
          <WalletButton className="btn btn-primary" />
        </div>
      </div>
    </div>
  )
}
