import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day10EscrowAnchor } from "../target/types/day10_escrow_anchor";
import {
  createMint,
  getAccount,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { expect } from "chai";
import assert from "assert";

describe("day10_escrow_anchor", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env(); // Get the provider from the enviroment
  anchor.setProvider(provider);

  const program = anchor.workspace
    .day10EscrowAnchor as Program<Day10EscrowAnchor>; // Loading the deployed program on localnet

  const initializer = provider.wallet; // Getting an initializer

  // Shared variables [Variables used in the tests]
  let mint: anchor.web3.PublicKey;
  let initializerAta: anchor.web3.PublicKey;
  let vaultAuthority: anchor.web3.PublicKey;
  let vaultAta: anchor.web3.PublicKey;
  let escrowState: anchor.web3.Keypair;

  // setting amounts required for tests using BN(BigNumber)
  const initializerAmount = new anchor.BN(500_000);
  const takerAmount = new anchor.BN(1);
  let escrowExpiry: anchor.BN;

  // getting the token account balance
  const getBalance = async (ata: anchor.web3.PublicKey) => {
    const account = await getAccount(provider.connection, ata);
    return Number(account.amount);
  };

  // Setting shared states
  before(async () => {
    // 1. creating mint
    mint = await createMint(
      provider.connection,
      initializer.payer,
      initializer.publicKey,
      null,
      6
    );

    // 2. create initializer ATA
    let initATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      initializer.payer,
      mint,
      initializer.publicKey
    );
    initializerAta = initATA.address;

    // 3. mint tokens to the initializer ATA
    await mintTo(
      provider.connection,
      initializer.payer,
      mint,
      initializerAta,
      initializer.publicKey,
      1_000_000
    );

    // 4. derive vault PDA
    [vaultAuthority] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), initializer.publicKey.toBuffer()], // ? about why we have used initializer.publickey.toBuffer()
      program.programId
    );

    // 5. creating vault ATA so the initializer can deposit tokens into the escrow vault
    vaultAta = anchor.utils.token.associatedAddress({
      mint,
      owner: vaultAuthority,
    });

    // 6. creating escrow state keypair
    escrowState = anchor.web3.Keypair.generate();

    // 7. setting expiry for the vault
    const now = Math.floor(Date.now() / 1000);
    escrowExpiry = new anchor.BN(now + 5);

    // 8. Now initializing the escrow
    program.methods
      .initializeEscrow(initializerAmount, takerAmount, escrowExpiry)
      .accounts({
        // inside this the vaultAuthority, vaultATA, systemProgram, tokenProgram, associatedTokenProgram and rent, all are automatically derived by the IDL generated for our anchor program and thus we don't need to provide them manually but still if want to provide it manually then can use accountsStrict and provide them manually
        initializer: initializer.publicKey,
        // vaultAuthority,
        // vaultAta,
        escrowState: escrowState.publicKey,
        mint,
        // systemProgram: anchor.web3.SystemProgram.programId,
        // tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        // associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        // rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([escrowState]) // ? about -> if the escrowState acting here as a signer for initializing escrow?
      .rpc();
  });

  //  ----------------------------
  //      DEPOSIT TEST
  //  --------------------------
  it("Deposit tokens into the escrow vault", async () => {
    const beforeInit = await getBalance(initializerAta);

    await program.methods
      .depositTokens()
      .accounts({
        initializer: initializer.publicKey,
        escrowState: escrowState.publicKey,
        // vaultAuthority,
        // vaultAta,
        mint,
        // initializerAta,
        // tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .rpc();

    const afterInit = await getBalance(initializerAta);
    const vaultBal = await getBalance(vaultAta);

    expect(afterInit).to.equal(beforeInit - initializerAmount.toNumber());
    expect(vaultBal).to.equal(initializerAmount.toNumber());

    const escrow = await program.account.escrowState.fetch(
      escrowState.publicKey
    );

    expect(escrow.state.deposited).to.not.be.undefined;
  });

  // --------------------------
  //     WITHDRAW TEST (before expiry of escrow ->  should fail)
  // --------------------------
  it("fails to withdraw before expiry", async () => {
    await assert.rejects(
      program.methods
        .withdrawTokens()
        .accounts({
          initializer: initializer.publicKey,
          escrowState: escrowState.publicKey,
          // vaultAuthority,
          // vaultAta,
          // initializerAta,
          mint,
          // tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        })
        .rpc()
    );
  });

  // -------------------------------
  //       WITHDRAW TEST (after expiry of the escrow ->  should pass)
  // -------------------------------
  it("success in withdrawing tokens after expiry of escrow", async () => {
    // waiting for escrow to expire
    await new Promise((resolve) => setTimeout(resolve, 6000));

    const beforeInit = await getBalance(initializerAta);

    await program.methods
      .withdrawTokens()
      .accounts({
        initializer: initializer.publicKey,
        escrowState: escrowState.publicKey,
        // vaultAuthority,
        // vaultAta,
        mint,
        // initializerAta,
        // tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .rpc();

    const afterInit = await getBalance(initializerAta);
    const vaultBal = await getBalance(vaultAta);

    expect(vaultBal).to.equal(0);
    expect(afterInit).to.equal(beforeInit + initializerAmount.toNumber());

    const escrow = await program.account.escrowState.fetch(
      escrowState.publicKey
    );
    expect(escrow.state.cancelled).to.not.be.undefined;
  });
});
