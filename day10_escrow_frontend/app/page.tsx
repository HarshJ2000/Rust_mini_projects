import { getProgram } from "@/lib/anchor";
import { BN } from "@coral-xyz/anchor";
import { useWallet } from "@solana/wallet-adapter-react";
import { Keypair, PublicKey } from "@solana/web3.js";
import toast from "react-hot-toast";

const MINT = new PublicKey("");

export default function Home() {
  const wallet = useWallet();

  const initEscrow = async () => {
    try {
      if (!wallet.publicKey) throw new Error("Wallet not found!!!!!");

      const program = getProgram(wallet);
      const escrow = Keypair.generate();
      const expiry = Math.floor(Date.now() / 1000) + 300;

      await program.methods
        .initializeEscrow(new BN(500_000), new BN(1), new BN(expiry))
        .accounts({
          initializer: wallet.publicKey,
          escrowState: escrow.publicKey,
          mint: MINT,
        })
        .signers([escrow])
        .rpc();

      localStorage.setItem("escrow", escrow.publicKey.toBase58());
      toast.success("Escrow Initialized.....");
    } catch (error: any) {
      console.error(error);
      toast.error(error.message);
    }
  };

  const depositInEscrow = async () => {
    try {
      if (!wallet.publicKey) throw new Error("Wallet not found!!!!!");

      const escrow = localStorage.getItem("escrow");
      if (!escrow) throw new Error("No Escrow Found!!!!!!!");

      const program = getProgram(wallet);

      await program.methods
        .depositTokens()
        .accounts({
          initializer: wallet.publicKey,
          escrowState: new PublicKey(escrow),
          mint: MINT,
        })
        .rpc();

      toast.success("Tokens Deposited.....");
    } catch (error: any) {
      console.error(error);
      toast.error(error.message);
    }
  };
}
