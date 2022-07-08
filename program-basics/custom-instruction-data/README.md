# Custom Instruction Data

OK, couple new things here. We want to make use of that data field in our transaction instruction.

**For native**, we need to add `borsh` and `borsh-derive` so we can mark a struct as serializable to/from **BPF format**.