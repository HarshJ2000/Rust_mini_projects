# 🧠 Day 9 — Smart Pointers + PDA Initialization (Anchor)
This day covered **two powerful concepts**:

## 📌 Part 1 — Rust Smart Pointers (Box, Rc, Arc)

Smart pointers help manage **ownership & memory safely**:
| Pointer | Use Case |
|---------|-----------|
| `Box<T>` | Store data on heap (single ownership) |
| `Rc<T>` | Multiple ownership (reference counting) |
| `Arc<T>` | Same as `Rc`, but thread-safe (for multi-threading) |

🧪 Output (Smart Pointer Demo)

Value stored in heap using Box: 50  
Recursive list using Box: Cons(1, Cons(2, Cons(3, Nil)))  
Reference count initially: 1  
After cloning 1 time: 2  
After cloning 2nd time: 3  
Reference count finally: 2  

-------------------
## 📌 Part 2 — Anchor PDA Initialization

We built a **program that creates a PDA based on**:

- `"user"` (seed 1)
- `wallet public key` (seed 2)

📄 **`program/src/lib.rs`**

```rust
#[account]
pub struct UserPda {
    pub authority: Pubkey,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let pda_account = &mut ctx.accounts.user_pda;
    pda_account.authority = *ctx.accounts.user.key;
    Ok(())
}


🧪 Test → tests/anchor_pda_example.ts

ts
Copy code
it("Initialize PDA!", async () => {
  const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  const tx = await program.methods
    .initialize()
    .accountsStrict({
      userPda: pda,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

  console.log("Tx:", tx);
  console.log("Expected PDA:", pda.toBase58());
});
```
---
🛠 Run It Locally
---
anchor build  
anchor deploy  
anchor test  

---
📌 What I Learned
---
- Difference between Box, Rc, Arc
- When to use smart pointers in real systems
- How PDAs are derived using seeds
- How to write & test PDA initialization in Anchor

