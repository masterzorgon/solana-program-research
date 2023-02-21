import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CpisErrors } from "../target/types/cpis_errors";

describe("cpis-errors", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CpisErrors as Program<CpisErrors>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
