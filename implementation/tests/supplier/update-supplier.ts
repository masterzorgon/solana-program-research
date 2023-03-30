import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Implementation } from "../../target/types/implementation";
import { assert } from "chai";
import fs from "fs";

describe("implementation", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Implementation as Program<Implementation>;
    const wallet = provider.wallet;

    // suppliers data
    const SUPPLIER_PREFIX: string = "SUPPLIER";
    const suppliers = [
        {
        name: "Fall Acres", // changed from "Fall Farms"
        address: "1234 Central Blvd", // changed from "1234 Main St"
        phone: "123-456-7890",
        email: "fallfarms@gmail.com",
        routingNumber: 123456789
        },
    ];

    interface CreateSupplierArgs {
        name: string,
        address: string,
        phone: string,
        email: string,
        routingNumber: number
    }

    const existingSupplierName = "Fall Farms";
    const [supplierPDA, _none] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from(SUPPLIER_PREFIX, "utf8"),
            Buffer.from("_", "utf8"),
            Buffer.from(existingSupplierName, "utf8"),
            Buffer.from("_", "utf8"),
            wallet.publicKey.toBuffer()
        ],
        program.programId
    );

    it("Updates a supplier", async () => {
        suppliers.forEach(async (supplier, index) => {
            await program.methods
                .updateSupplier(supplier as CreateSupplierArgs)
                .accounts({
                    supplier: supplierPDA,
                    signer: wallet.publicKey,
                })
                .rpc();
        });

        // get and check supplier data
        const updatedSupplier = await program.account.supplier.fetch(supplierPDA);
        assert.equal(updatedSupplier.name.toString(), suppliers[0].name);

        // write data to file
        fs.writeFileSync(
            "/Users/masterzorgon/Documents/Programming/personal-projects/blockchain/blockchain-research/implementation/tests/supplier/supplier-data/update-supplier.json",
            JSON.stringify(updatedSupplier, null, 4)
        );
    });
});
