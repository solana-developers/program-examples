import {
  Box,
  Button,
  Flex,
  Spacer,
  VStack,
  useDisclosure,
} from "@chakra-ui/react"
import WalletMultiButton from "@/components/WalletMultiButton"
import QrModal from "@/components/QrCodeCnftMint"
export default function Home() {
  const { isOpen, onOpen, onClose } = useDisclosure()
  return (
    <Box>
      <Flex px={4} py={4}>
        <Spacer />
        <WalletMultiButton />
      </Flex>
      <VStack justifyContent="center">
        <Button onClick={onOpen}>Solana Pay Mint</Button>
        {isOpen && <QrModal onClose={onClose} />}
      </VStack>
    </Box>
  )
}
