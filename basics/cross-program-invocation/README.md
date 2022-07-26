# Cross Program Invocation (CPI)

A cross-program invocation *literally* means invoking a program from another program (hence the term "cross-program"). There's really nothing special about it besides that. You're leveraging other programs from within your program to conduct business on Solana accounts.   

Whether or not you should send instructions to a program using a cross-program invocation or a client RPC call is a design choice that's completely up to the developer.   

There are many design considerations when making this decision, but the most common one to acknowledge is a **dependent operation** embedded in your program.   

Consider the below sequence of operations of an example **token mint** program:
1. Create & initialize the mint.
2. Create a metadata account for that mint.
3. Create & initialize a user's token account for that mint.
4. Mint some tokens to the user's token account.

In the above steps, we can't create a metadata account without first creating a mint! In fact, we have to do all of these operations in order.   

Let's say we decided it was essential to have our mint (operation 1) and our "mint to user" (operation 4) tasks on-chain. We would have no choice but to also include the other two operations, since we can't do operation #1, pause the program while we do #2 & #3 from the client, and then resume the program for #4.

### Let's switch the power on and off using a CPI!   

<img src="istockphoto-1303616086-612x612.jpeg" alt="lever" width="128" align="center"/>

In this example, we're just going to simulate a simple CPI - using one program's method from another program.   

Inside our `hand` program's `pull_lever` function, there's a cross-program invocation to our `lever` program's `switch_power` method.   

Simply put, **our hand program will pull the lever on the lever program to switch the power on and off**.