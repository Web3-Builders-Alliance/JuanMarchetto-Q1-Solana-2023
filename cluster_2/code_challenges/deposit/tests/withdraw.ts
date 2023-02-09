import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Deposit } from "../target/types/deposit";
import * as web3 from "@solana/web3.js"
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("withdraw", () => {
  // Configure the client to use the local cluster.
  const anchorProvider = anchor.AnchorProvider.env();
  anchor.setProvider(anchorProvider);
  const program = anchor.workspace.Deposit as Program<Deposit>;
  const name = "test";

  it("withdraw from ", async () => {
    const [vault] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(name)],
      program.programId,
    )
    const tx = await program.methods.withdraw(new anchor.BN(LAMPORTS_PER_SOL))
    .accounts({
      vault,
      user: anchorProvider.wallet.publicKey,
      systemProgram: web3.SystemProgram.programId,
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
