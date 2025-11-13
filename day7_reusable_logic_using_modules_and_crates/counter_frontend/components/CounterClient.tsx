"use client";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { useState } from "react";

const PROGRAM_ID = new PublicKey(process.env.NEXT_PUBLIC_PROGRAM_ID!);

export const CounterClient = () => {
  const { connection } = useConnection();
  const { publicKey, sendTransaction } = useWallet();
  const [loading, setLoading] = useState(false);

  const increment = async () => {
    if (!publicKey) return alert("Connect your wallet first!!!");

    setLoading(true);

    try {
      const txn = new Transaction().add(
        SystemProgram.transfer({
          fromPubkey: publicKey!,
          toPubkey: publicKey!,
          lamports: 0,
        })
      );

      const sig = await sendTransaction(txn, connection);
      await connection.confirmTransaction(sig, "confirmed");
      alert(`Transaction confirmed: ${sig}`);
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <button onClick={increment} disabled={loading}>
        {loading ? "Processing..." : "Increment Counter"}
      </button>
    </div>
  );
};
