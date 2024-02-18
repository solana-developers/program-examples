import { Box, Flex, Heading, Spacer, VStack, Text } from "@chakra-ui/react"
import { useWallet } from "@solana/wallet-adapter-react"
import WalletMultiButton from "@/components/WalletMultiButton"
import DisplayGameState from "@/components/DisplayGameState"
import InitPlayerButton from "@/components/InitPlayerButton"
import SessionKeyButton from "@/components/SessionKeyButton"
import ChopTreeButton from "@/components/ChopTreeButton"
import RequestAirdrop from "@/components/RequestAirdrop"
import DisplayNfts from "@/components/DisplayNfts"
import MintNftButton from "@/components/MintNftButton"

export default function Home() {
  const { publicKey } = useWallet()

  return (
    <Box>
      <Flex px={4} py={4}>
        <Spacer />
        <WalletMultiButton />
      </Flex>
      <VStack>
        <Heading>ExtensionNft</Heading>
        {!publicKey && <Text>Connect to devnet wallet!</Text>}
        <DisplayGameState />
        <InitPlayerButton />
        <SessionKeyButton />
        <ChopTreeButton />
        <MintNftButton />
        <RequestAirdrop />
        <DisplayNfts />
      </VStack>
    </Box>
  )
}
