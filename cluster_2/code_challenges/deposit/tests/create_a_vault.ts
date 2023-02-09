import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Deposit } from "../target/types/deposit";
import * as web3 from "@solana/web3.js"

describe("deposit", () => {
  // Configure the client to use the local cluster.
  const anchorProvider = anchor.AnchorProvider.env();
  anchor.setProvider(anchorProvider);
  const program = anchor.workspace.Deposit as Program<Deposit>;
  const name = "test";

  it("Init a deposit", async () => {
    const [vault] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(name)],
      program.programId,
    )
    const tx = await program.methods.initialize(name)
    .accounts({
      vault,
      owner: anchorProvider.wallet.publicKey,
      systemProgram: web3.SystemProgram.programId,
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
