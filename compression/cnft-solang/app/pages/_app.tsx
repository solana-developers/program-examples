import { ChakraProvider } from "@chakra-ui/react"
import WalletContextProvider from "../contexts/WalletContextProvider"
import type { AppProps } from "next/app"

export default function App({ Component, pageProps }: AppProps) {
  return (
    <ChakraProvider>
      <WalletContextProvider>
        <Component {...pageProps} />
      </WalletContextProvider>
    </ChakraProvider>
  )
}
