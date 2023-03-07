import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Pdas2 } from "../target/types/pdas2";
import { assert } from "chai";

describe("pdas2", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Pdas2 as Program<Pdas2>;
  const wallet = provider.wallet;
  
  const [PDA, _] = anchor.web3.PublicKey.findProgramAddressSync(
    [ // PDA seeds
      Buffer.from("user_stats_"),
      wallet.publicKey.toBuffer(),
    ],
    program.programId, // PDA owner program
  );

  it("Creates a PDA!", async () => {
    const name = "Nathan";

    const tx = await program.methods
      .createUserStats(name)
      .accounts({
        userStats: PDA,
        user: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const fetchedPDA = await program.account.userStats.fetch(PDA);
    assert.equal(fetchedPDA.name.toString(), name);
  });

  it("Updates a PDA!", async () => {
    const name = "Ethan";

    const tx = await program.methods
      .changeUserName(name)
      .accounts({
        userStats: PDA,
        user: wallet.publicKey
      })
      .rpc();
    
    const fetchedPDA = await program.account.userStats.fetch(PDA);
    assert.equal(fetchedPDA.name.toString(), name);
  });
});
