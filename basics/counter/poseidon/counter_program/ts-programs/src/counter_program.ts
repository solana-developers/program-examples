import { Account, Pubkey, Signer, u8, u64 } from '@solanaturbine/poseidon';

 export interface Counter extends Account {
   count: u64;
   bump: u8;
 }

 export default class counter_program {
   static PROGRAM_ID = new Pubkey(
     "EvcknV23Y3dkbSa4afZNGw2PgoowcfxCy4qvP8Ghogwu"
   );

   initializeCounter(counter: Counter, payer: Signer) {
     counter.derive(["count"]).init();
     counter.count = new u64(0);
   }
   increment(counter: Counter) {
     counter.derive(["count"]);
     counter.count = counter.count.add(1);
   }
   decrement(counter: Counter) {
     counter.derive(["count"]);
     counter.count = counter.count.sub(1);
   }
 }
 