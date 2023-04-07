import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Introductions } from "../target/types/introductions";
import { assert } from "chai";

describe("introductions", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Introductions as Program<Introductions>;

  const introArgs = {
    name: "Nathan",
    content: "Hello, World!",
  };

  const initializer = provider.wallet.publicKey;

  const [introPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("INTRO_ACCOUNT", "utf8"), 
      Buffer.from(introArgs.name, "utf8"), 
      initializer.toBuffer()
    ],
    program.programId
  );

  it("Adds intro account", async () => {
    const tx = await program.methods
      .addIntro(introArgs)
      .accounts({ introAccount: introPDA })
      .rpc()

    const account = await program.account.studentIntroAccount.fetch(introPDA)
    assert.equal(introArgs.name, account.name);
    assert.equal(introArgs.content, account.content);
  });

  it("Updates intro account", async () => {
    const update = "Hello, World! This is an update."
    introArgs.content = update;

    const tx = await program.methods
      .updateIntro(introArgs)
      .accounts({ introAccount: introPDA })
      .rpc();
    
    const account = await program.account.studentIntroAccount.fetch(introPDA);
    assert.equal(update, account.content);
  });

  it("Deletes intro account", async () => {
    const tx = await program.methods
      .closeIntro(introArgs)
      .accounts({ introAccount: introPDA })
      .rpc();
  });
});
