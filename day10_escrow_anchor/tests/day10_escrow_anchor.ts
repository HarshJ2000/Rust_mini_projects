import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day10EscrowAnchor } from "../target/types/day10_escrow_anchor";
import { createMint, getAccount } from "@solana/spl-token";

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
  const initializer_amount = new anchor.BN(500_000);
  const taker_amount = new anchor.BN(1);
  let escrowExpiry: anchor.BN;

  // getting the token account balance
  const getBalance = async (ata: anchor.web3.PublicKey) => {
    const account = await getAccount(provider.connection, ata);
    return Number(account.amount);
  };

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
