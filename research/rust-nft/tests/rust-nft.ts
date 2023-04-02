import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { RustNft } from "../target/types/rust_nft";

describe("rust-nft", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RustNft as Program<RustNft>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
