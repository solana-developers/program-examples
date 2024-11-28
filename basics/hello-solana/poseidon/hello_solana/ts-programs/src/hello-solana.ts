import { Pubkey, Result } from '@solanaturbine/poseidon'

export default class HelloSolana {
  static PROGRAM_ID = new Pubkey('DaNK9CdncCbPrHRWJpWL1oyEBS9M985YYXR8WTQzYSdE')

  hello(): Result {
    console.log('Hello, Solana!')

    console.log(`Our program's Program ID: ${HelloSolana.PROGRAM_ID}`)
  }
}
