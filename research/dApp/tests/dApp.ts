import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DApp } from "../target/types/d_app";

describe("dApp", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DApp as Program<DApp>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
