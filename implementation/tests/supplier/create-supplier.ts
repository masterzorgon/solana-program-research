import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Implementation } from "../../target/types/implementation";
import { assert } from "chai";
import fs from "fs";

describe("implementation", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Implementation as Program<Implementation>;
  const supplier = anchor.web3.Keypair.generate();
  const wallet = provider.wallet;

  // suppliers data
  const SUPPLIER_PREFIX: string = "SUPPLIER";
  const suppliers = [
    {
      name: "Fall Farms",
      address: "1234 Main St",
      phone: "123-456-7890",
      email: "fallfarms@gmail.com",
      routingNumber: 123456789
    },
    {
      name: "Cheeky Chickens",
      address: "131 South Ave",
      phone: "222-333-1411",
      email: "cheekychickens@gmail.com",
      routingNumber: 987654321
    },
    {
      name: "Corby's Cows",
      address: "222 Crescent Rd",
      phone: "333-444-5555",
      email: "corbyscows@gmai.com",
      routingNumber: 198273645,
    }
  ];

  interface CreateSupplierArgs {
    name: string,
    address: string,
    phone: string,
    email: string,
    routingNumber: number
  }

  // get supplier PDAs
  const supplierPDAs = suppliers.map(supplier => {
    const [supplierPDA, _none] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(SUPPLIER_PREFIX, "utf8"),
        Buffer.from("_", "utf8"),
        Buffer.from(supplier.name, "utf8"),
        Buffer.from("_", "utf8"),
        wallet.publicKey.toBuffer()
      ],
      program.programId
    );
    return supplierPDA;
  });

  it("Creates a supplier PDA", async () => {
    suppliers.forEach(async (supplier, index) => {
      // get supplier account PDA
      const supplierPDA = supplierPDAs[index];

      // create supplier account
      const tx = await program.methods
        .createSupplier(supplier as CreateSupplierArgs)
        .accounts({
          supplier: supplierPDA,
          signer: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      
      const supplierAccount = await program.account.supplier.fetch(supplierPDA)
      const config = {
          supplierBump: supplierAccount.bump.toString(),
          supplierPDA: supplierPDAs[index].toString(),
          supplierIdentifier: supplierAccount.identifier.toString(),
          supplierTotalTransactions: supplierAccount.totalTransactions.toString(),
          supplierName: supplierAccount.name.toString(),
          supplierAddress: supplierAccount.address.toString(),
          supplierPhone: supplierAccount.phone.toString(),
          supplierEmail: supplierAccount.email.toString(),
          supplierRoutingNumber: supplierAccount.routingNumber.toString(),
          supplierInvoices: supplierAccount.invoices.toString()
      };
      
      assert.equal(supplierAccount.name.toString(), supplier.name);

      fs.writeFileSync(
        "/Users/masterzorgon/Documents/Programming/personal-projects/blockchain/blockchain-research/implementation/tests/supplier-data/config.json",
        JSON.stringify(config, null, 4)
      );

      console.log("transaction successful: ", tx);
    });
  });
});
