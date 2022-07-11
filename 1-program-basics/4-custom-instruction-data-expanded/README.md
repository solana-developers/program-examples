# Custom Instruction Data - Expanded

:gem: Here's where Anchor really shines. :gem:   

You'll notice in these examples that the `native` version of the program is much more verbose on the client side - due to the fact that we have to re-create every custom data type from our Rust code.   

This is one of Anchor's greatest features. The framework will extract all of that information out of the Rust source and into JSON and TypeScript files. This makes it much easier and more dynamic to re-create these data types on the client side. Although it's still necessary to do (which can be trivial), it's >100x easier with Anchor.