import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Keypair } from "@solana/web3.js";
import { assert, expect } from "chai";
import { Puppet } from "../target/types/puppet";
import { ProxyProgram } from "../target/types/proxy_program";

describe("cpis", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const puppetProgram = anchor.workspace.Puppet as Program<Puppet>;
  const proxyProgram = anchor.workspace.ProxyProgram as Program<ProxyProgram>;

  const puppetKeypair = Keypair.generate();

  it("Does CPI!", async () => {
    await puppetProgram.methods
      .initialize()
      .accounts({
        puppet: puppetKeypair.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([puppetKeypair])
      .rpc();
    
    await proxyProgram.methods
      .pullStrings(new anchor.BN(42))
      .accounts({
        puppet: puppetKeypair.publicKey,
        puppetProgram: puppetProgram.programId,
      })
      .rpc();

    const fetchedPuppet = await puppetProgram.account
      .data
      .fetch(puppetKeypair.publicKey);
    assert.equal(fetchedPuppet.data.toNumber(), 42);
  });
});
