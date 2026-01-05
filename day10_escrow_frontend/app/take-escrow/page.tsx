import { getProgram } from "@/lib/anchor";
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { PublicKey } from "@solana/web3.js";
import { useState } from "react";

const MINT = new PublicKey("5WYMZpCiApdbpiVPB1LitttYAe9Q2uRfUisfu2cyTjVm");

export default function TakeEscrowPage() {
  const wallet = useWallet();

  const [escrowAddress, setEscrowAddress] = useState("");
  const [txSig, setTxSig] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  async function depositTokens() {
    if (!wallet.connected || !wallet.publicKey) {
      alert("Connect wallet first!!!!!");
      return;
    }

    try {
      setLoading(true);

      const program = getProgram(wallet);

      const tx = await program.methods
        .depositTokens()
        .accounts({
          initializer: wallet.publicKey,
          escrowState: new PublicKey(escrowAddress),
          mint: MINT,
        })
        .rpc();

      setTxSig(tx);
    } catch (error) {
      console.error(error);
      alert("Deposit Failed!!!!");
    } finally {
      setLoading(false);
    }
  }

  async function withdrawTokens() {
    if (!wallet.connected || !wallet.publicKey) {
      alert("Connect Wallet First!!!!");
      return;
    }

    try {
      setLoading(true);

      const program = getProgram(wallet);

      const tx = await program.methods
        .withdrawTokens()
        .accounts({
          initializer: wallet.publicKey,
          escrowState: new PublicKey(escrowAddress),
          mint: MINT,
        })
        .rpc();

      setTxSig(tx);
    } catch (error) {
      console.error(error);
      alert("Withdraw Failed!!!!!");
    } finally {
      setLoading(false);
    }
  }

  return (
    <main style={{ padding: 24 }}>
      <h1>Take Escrow</h1>

      <WalletMultiButton />

      <br />
      <br />

      <label>
        Escrow State Address:
        <input
          style={{ width: "100%" }}
          value={escrowAddress}
          onChange={(e) => setEscrowAddress(e.target.value)}
          placeholder="Paste escrowState publickey"
        />
      </label>

      <br />
      <br />

      <button onClick={depositTokens} disabled={loading}>
        Deposit Tokens
      </button>

      <br />
      <br />

      <button onClick={withdrawTokens} disabled={loading}>
        Withdraw Tokens
      </button>

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
