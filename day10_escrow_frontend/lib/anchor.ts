import { WalletContextState } from "@solana/wallet-adapter-react";
import idl from "../lib/idl.json";
import { Connection, PublicKey } from "@solana/web3.js";
import { AnchorProvider, Idl, Program } from "@coral-xyz/anchor";

export const PROGRAM_ID = new PublicKey(
  "GWHDeVEboCXJfKZyuZuKX3omooMYYjMXs9SdRYa7HVNB"
);

const RPC_ENDPOINT = "https://api.devnet.solana.com";

export function getProgram(wallet: WalletContextState) {
  if (
    !wallet.publicKey ||
    !wallet.signTransaction ||
    !wallet.signAllTransactions
  ) {
    throw new Error("Wallet not found!!!!!!");
  }

  const connection = new Connection(RPC_ENDPOINT, "confirmed");

  const anchorWallet = {
    publicKey: wallet.publicKey,
    signTransaction: wallet.signTransaction,
    signAllTransactions: wallet.signAllTransactions,
  };

  const provider = new AnchorProvider(connection, anchorWallet, {
    commitment: "confirmed",
  });

  const program = new Program(idl as Idl, provider);

  if (!program.programId.equals(PROGRAM_ID)) {
    throw new Error("PROGRAM_ID mismatch between IDL and Frontend!!!!!!");
  }

  return program;
}
