**Anchor Escrow Challenge – Blueshift**


**Overview**
This repository contains my solution for the Anchor Escrow challenge on Blueshift⁠�.
The challenge was to build a Solana escrow program using the Anchor framework that allows users to:
Make an escrow offer :– A maker can initiate an escrow swap.
Take an escrow offer :– A taker can accept an escrow swap.
Cancel an escrow offer – The maker can cancel and refund the escrowed funds.


**About This Project**
This project was a learning experience in building secure Solana programs. The key focus areas included:
Understanding Anchor instructions and account discriminators.
Working with program-derived accounts (PDAs).
Handling rent-exemption and account ownership.
Debugging deployment issues, including DeclaredProgramIdMismatch errors.

Core Functionality
Make Offer: A "Maker" initiates a swap by locking tokens into a Program Derived Address (PDA) vault.
Take Offer: A "Taker" accepts the swap, sending the requested tokens to the Maker and receiving the vaulted tokens in return.
Refund/Cancel: The Maker can cancel the offer and reclaim their escrowed funds before a Taker intervenes.



Technical Deep Dive
During this challenge, I implemented several advanced Solana development patterns:
PDA Management: Utilized Program Derived Addresses to securely hold user funds without requiring a private key, ensuring the program has sole authority over the vault.
Anchor Constraints: Leveraged #[account(has_one = ...)] and #[account(seeds = ...)] to enforce strict security checks at the account level, preventing unauthorized access or spoofing.
Token Program Integration: Managed SPL-Token transfers and account initialization using Anchor's CpiContext.
State Management: Structured the Escrow state to efficiently track the Maker, the tokens offered, and the tokens requested.
🛠️ Lessons Learned & Debugging
The path to "Build Success" wasn't linear. I encountered and resolved several key hurdles:
The Program ID Puzzle: I navigated the DeclaredProgramIdMismatch error by aligning my local configuration with the Blueshift testing environment requirements (2222...2222).
Account Validation: I gained a much deeper understanding of how Anchor handles account discriminators and why strict validation is the backbone of Solana security.
Support & Community: Big thanks to Evolu and Daniel Authensis for their guidance when I hit roadblocks with deployment constraints!


**How to Build**

Clone the repository:
Bash
git clone https://github.com/taiwokassim/blueshift-anchor-escrow.git
cd blueshift-anchor-escrow

yarn install
# or
npm install

anchor build
anchor test


Note: The program ID is fixed to 22222222222222222222222222222222222222222222 to remain compatible with the Blueshift challenge environment.



.
├── programs/
│   └── blueshift_anchor_escrow/
│       └── src/
│           ├── lib.rs           # Program entry point & instruction routing
│           ├── instructions/    # Logic for Make, Take, and Refund
│           └── state.rs         # Data structures for the Escrow account
├── tests/
│   └── anchor_escrow_tests.rs   # Integration tests for the full lifecycle
└── Anchor.toml                  # Program configuration



**Install dependencies**:
Bash
Copy code
anchor install

**Build the program**:
Bash
anchor build
Note: The program ID has been aligned with the fixed example ID expected by the Blueshift challenge:
declare_id!("22222222222222222222222222222222222222222222");
Run tests:
Bash
Copy code
anchor test
Structure
Copy code

programs/
  blueshift_anchor_escrow/
    src/
      lib.rs        # Main program logic
      instructions/ # Make, Take, Refund instructions
      state.rs      # State account definitions
tests/
  anchor_escrow_tests.rs  # Tests for all instructions
Anchor.toml

![Challenge Completion](./IMG_20260324_204246_658.jpg)


**Submission & Verification**
Wallet for NFT: DCwqftGn7mtRZj6UT1VZikUfpUbLBiecZtwAE1S5STsG
Twitter thread sharing my experience: 

https://x.com/i/status/2036700225369194517

**Notes**
Some issues encountered were platform-specific, such as the program ID expected by Blueshift tests.
Debugging these helped deepen understanding of Anchor deployments and Solana account mechanics.
