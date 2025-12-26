"use client";
import { getProgram } from "@/lib/anchor";
import { BN, Wallet } from "@coral-xyz/anchor";
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
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
  return (
    <main style={{ padding: 24 }}>
      <h1>Initialize Escrow</h1>
      <WalletMultiButton />
      <br />
      <br />
      <label>
        Initializer Amount:
        <input
          value={initializerAmount}
          onChange={(e) => setInitializerAmount(e.target.value)}
        />
      </label>
      <br />
      <br />
      <label>
        Taker Amount:
        <input
          value={takerAmount}
          onChange={(e) => setTakerAmount(e.target.value)}
        />
      </label>
      <br />
      <br />
      <label>
        Expiry (seconds):
        <input
          value={expirySeconds}
          onChange={(e) => setExpirySeconds(e.target.value)}
        />
      </label>
      <br />
      <br />
      <button onClick={initializeEscrow} disabled={loading}>
        {loading ? "Initializing...." : "Create Escrow"}
      </button>

      {escrowState && (
        <>
          <br />
          <br />
          <strong>Escrow State: </strong>
          <p>{escrowState}</p>
        </>
      )}

      {txSig && (
        <>
          <br />
          <br />
          <a
            href={`https://explorer.solana.com/tx/${txSig}?cluster=devnet`}
            target="_blank"
          >
            View Transaction
          </a>
        </>
      )}
    </main>
  );
}
