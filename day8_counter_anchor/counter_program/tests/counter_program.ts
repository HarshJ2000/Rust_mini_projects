import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterProgram } from "../target/types/counter_program";
import { expect } from "chai";

describe("counter_program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.counterProgram as Program<CounterProgram>;

  it("Is Initialized!!!", async () => {
    const authority = provider.wallet.publicKey;

    const [counterPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter"), authority.toBuffer()],
      program.programId
    );

    await program.methods
      .initialize(new anchor.BN(0))
      .accountsStrict({
        counterAcc: counterPda,
        authority,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.counter.fetch(counterPda);
    expect(account.count.toNumber()).to.equal(0);

    console.log("Counter PDA: ", counterPda.toString());
  });

  it("Increment Counter....", async () => {
    const authority = provider.wallet.publicKey;

    const [counterPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter"), authority.toBuffer()],
      program.programId
    );

    await program.methods
      .increment(new anchor.BN(5))
      .accounts({
        counterAcc: counterPda,
        authority,
      })
      .rpc();

    const account = await program.account.counter.fetch(counterPda);
    expect(account.count.toNumber()).to.equal(5);

    console.log("Counter Incremented by 5...");
  });

  it("Decrement Counter...", async () => {
    const authority = provider.wallet.publicKey;

    const [counterPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter"), authority.toBuffer()],
      program.programId
    );

    await program.methods
      .decrement(new anchor.BN(2))
      .accounts({
        counterAcc: counterPda,
        authority,
      })
      .rpc();

    const account = await program.account.counter.fetch(counterPda);
    expect(account.count.toNumber()).to.equal(3);

    console.log("Counter Decremented by 2...");
  });

  it("Reset Counter...", async () => {
    const authority = provider.wallet.publicKey;

    const [counterPda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter"), authority.toBuffer()],
      program.programId
    );

    await program.methods
      .reset()
      .accounts({
        counterAcc: counterPda,
        authority,
      })
      .rpc();

    const account = await program.account.counter.fetch(counterPda);
    expect(account.count.toNumber()).to.equal(0);

    console.log("Counter reset to 0");
  });
});
