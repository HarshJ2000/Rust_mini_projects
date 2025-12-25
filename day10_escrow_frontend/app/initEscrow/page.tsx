import { getProgram } from "@/lib/anchor";
import { BN } from "@coral-xyz/anchor";
import { useWallet } from "@solana/wallet-adapter-react";
import { Keypair, PublicKey } from "@solana/web3.js";
import { useState } from "react";
import toast from "react-hot-toast";

const MINT = new PublicKey("5WYMZpCiApdbpiVPB1LitttYAe9Q2uRfUisfu2cyTjVm");

export default function InitEscrowPage() {
  const wallet = useWallet();

  const [initializerAmount, setInitializerAmount] = useState("500000");
  const [takerAmount, setTakerAmount] = useState("1");
  const [expirySeconds, setExpirySeconds] = useState("120");

  const [escrowState, setEscrowState] = useState<string | null>(null);
  const [txSig, setTxSig] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  async function initializeEscrow() {
    if (!wallet.connected || !wallet.publicKey) {
      alert("Connect Wallet first!!!!!");
      return;
    }

    setLoading(true);

    try {
      const program = getProgram(wallet);

      const escrowKeypair = Keypair.generate();

      const initAmount = new BN(initializerAmount);
      const takeAmount = new BN(takerAmount);
      const expiry = new BN(
        Math.floor(Date.now() / 1000) + Number(expirySeconds)
      );

      const tx = await program.methods
        .initializeEscrow()
        .accounts({
          initializer: wallet.publicKey,
          taker: escrowKeypair.publicKey,
          mint: MINT,
        })
        .signers([escrowKeypair])
        .rpc();

      setEscrowState(escrowKeypair.publicKey.toBase58());
      setTxSig(tx);
    } catch (error) {
      console.error(error);
      toast.error("Failed to initialize escrow!!!!!");
    } finally {
      setLoading(false);
    }
  }
  return ();
}
