import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Cpipda } from "../target/types/cpipda";
import { assert } from "chai";

describe("cpipda", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Cpipda as Program<Cpipda>;
  const wallet = provider.wallet;

  const [pda, _nonce] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user_socials_", "utf8"), wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Sets user socials", async () => { 
    const tx = await program.methods
      .setUserSocials()
      .accounts({
        userSocials: pda,
        user: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .rpc();
    
    const userInfo = await program.account.userInfo.fetch(pda);
    assert.equal(userInfo.name.toString(), "");
  });

  it("Creates a user", async () => {
    const info = {
      name: "John Doe",
      twitter: "johndoe",
      discord: "johndoe",
    };

    const tx = await program.methods
      .createUserSocials(info)
      .accounts({
        userSocials: pda,
        user: wallet.publicKey
      })
      .rpc();
    
    const userInfo = await program.account.userInfo.fetch(pda);
    assert.equal(userInfo.name.toString(), info.name);
  });
});
