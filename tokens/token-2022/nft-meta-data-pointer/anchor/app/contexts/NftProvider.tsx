import { createContext, useContext, useEffect, useState } from "react"
import { PublicKey } from "@solana/web3.js"
import { useWallet } from "@solana/wallet-adapter-react"
import {
  CONNECTION,
} from "@/utils/anchor"

const NftContext = createContext<{
  nftState: any | null  
}>({
  nftState: null,
})

export const useNftState = () => useContext(NftContext)

export const NftProvider = ({
  children,
}: {
  children: React.ReactNode
}) => {
  const { publicKey } = useWallet()

  const [nftState, setNftState] = useState<any | null>(null)

  useEffect( ()  => {
    setNftState(null)
    if (!publicKey) {
      return
    }
    
    getAssetsByOwner(publicKey);

  }, [publicKey]);

  async function getAssetsByOwner(ownerAddress: PublicKey) {
    const sortBy = {
      sortBy: "created",
      sortDirection: "asc",
    };
    const limit = 1000;
    const page = 1;
    const before = "";
    const after = "";
    const allAssetsOwned = await CONNECTION.getAssetsByOwner(
      ownerAddress.toBase58(),
      sortBy,
      limit,
      page,
      before,
      after
    );

    setNftState(allAssetsOwned);
    console.log(allAssetsOwned);
  }

  return (
    <NftContext.Provider
      value={{
        nftState: nftState,
      }}
    >
      {children}
    </NftContext.Provider>
  )
}
