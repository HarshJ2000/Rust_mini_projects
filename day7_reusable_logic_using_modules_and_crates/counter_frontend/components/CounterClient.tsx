import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { PublicKey } from "@solana/web3.js";
import { useState } from "react";

const PROGRAM_ID = new PublicKey(process.env.NEXT_PUBLIC_PROGRAM_ID!);

export const CounterClient = () => {
  const { connection } = useConnection();
  const { publickey, sendTransaction } = useWallet();
  const { loading, setLoading } = useState(false);

  return (
    <div>
      <button>{loading ? "Processing..." : "Increment Counter"}</button>
    </div>
  );
};
