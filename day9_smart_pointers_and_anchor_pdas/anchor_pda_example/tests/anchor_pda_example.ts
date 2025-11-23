import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorPdaExample } from "../target/types/anchor_pda_example";

describe("anchor_pda_example", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .anchorPdaExample as Program<AnchorPdaExample>;

  it("Is initialized!", async () => {
    // Add your test here.

    const [pda] = await anchor.web3.PublicKey.findProgramAddressSync(
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

    console.log("Your transaction signature", tx);
    console.log("Expected PDA: ", pda.toBase58());
  });
});
