import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Implementation } from "../../target/types/implementation";
import { assert } from "chai";
import fs from "fs";

 interface Relationship {
    supplierName: string,
    businessUnitName: string,
    masterEdition: anchor.web3.PublicKey
  }

  interface CreateSupplierArgs {
    name: string,
    address: string,
    phone: string,
    email: string,
    routingNumber: string,
  }

describe("implementation", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Implementation as Program<Implementation>;
  const wallet = provider.wallet;

  const relationships: Relationship[] = [
    {
      supplierName: "Fall Farms",
      businessUnitName: "Tyson Fresh Meats",
      masterEdition: new anchor.web3.PublicKey("BVPntDCVsbDvFWhMtqG3Ra2Q8kHcMbxCvokdKNoXuLLP")
    },
    {
      supplierName: "Fall Farms",
      businessUnitName: "Tyson Fresh Meats",
      masterEdition: new anchor.web3.PublicKey("BVPntDCVsbDvFWhMtqG3Ra2Q8kHcMbxCvokdKNoXuLLP")
    },
    {
      supplierName: "Fall Farms",
      businessUnitName: "Tyson Fresh Meats",
      masterEdition: new anchor.web3.PublicKey("BVPntDCVsbDvFWhMtqG3Ra2Q8kHcMbxCvokdKNoXuLLP")
    }
  ];

  // suppliers data
  const SUPPLIER_PREFIX: string = "SUPPLIER";
  const supplierArgs: CreateSupplierArgs = {
    name: "Fall Farms",
    address: "1234 Main St",
    phone: "123-456-7890",
    email: "fallfarms@gmail.com",
    routingNumber: "123456789",
  };

  // get supplier PDAs
  const [supplierPDA, _none] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from(SUPPLIER_PREFIX, "utf8"),
      Buffer.from("_", "utf8"),
      Buffer.from(supplierArgs.name, "utf8"),
      Buffer.from("_", "utf8"),
      wallet.publicKey.toBuffer()
    ],
    program.programId
  );

  it("Creates a supplier PDA", async () => {
    const tx = await program.methods
      .createSupplier(
        supplierArgs as CreateSupplierArgs,
        relationships as Relationship[]
      )
      .accounts({
        supplier: supplierPDA,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    
    const supplier = await program.account.supplier.fetch(supplierPDA);
    assert.equal(supplier.name.toString(), supplierArgs.name);

    fs.writeFileSync(
      "/Users/masterzorgon/Documents/Programming/personal-projects/blockchain/blockchain-research/implementation/tests/supplier/supplier-data/create-supplier.json",
      JSON.stringify({
        bump: supplier.bump,
        identifier: supplier.identifier.toString(),
        name: supplier.name.toString(),
        address: supplier.address.toString(),
        phone: supplier.phone.toString(),
        email: supplier.email.toString(),
        routing_number: supplier.routingNumber,
        relationships: supplier.relationships,
        total_transactions: supplier.totalTransactions,
      }, null, 2)
    );
  });
});
