import { initializeKeypair } from "./initializeKeypair"
import { Transaction, Connection, PublicKey, Keypair} from "@solana/web3.js"
import * as web3 from "@solana/web3.js"
import * as token from "@solana/spl-token"
import {
    Metaplex,
    keypairIdentity,
    bundlrStorage,
    toMetaplexFile,
} from "@metaplex-foundation/js"
import {
    DataV2,
    createCreateMetadataAccountV2Instruction,
    createUpdateMetadataAccountV2Instruction,
} from "@metaplex-foundation/mpl-token-metadata"
import * as fs from "fs"

const createMint = async (
    connection: Connection,
    payer: Keypair,
    mintAuthority: PublicKey,
    freezeAuthority: PublicKey,
    decimals: number
) => {
    const tokenMint = await token.createMint(
        connection,
        payer,
        mintAuthority,
        freezeAuthority,
        decimals
    );

    console.log(`\n[*] Created Token Mint: https://explorer.solana.com/address/${tokenMint}?cluster=devnet`);

    return tokenMint;
};

const createMetadata = async (
    connection: Connection,
    metaplex: Metaplex,
    mint: PublicKey,
    payer: Keypair,
    name: string,
    symbol: string,
    description: string
) => {
    const buffer = fs.readFileSync("/Users/masterzorgon/Documents/Programming/personal-projects/blockchain/blockchain-research/research/solana-token-client/src/assets/blockie.png");
    const file = toMetaplexFile(buffer, "blockie.png");
    const imageURI = await metaplex.storage().upload(file);

    console.log(`\n[*] Uploaded Image URI: ${imageURI}`);

    const { uri } = await metaplex
        .nfts()
        .uploadMetadata({
            name: name,
            description: description,
            image: imageURI
        });
    
    console.log(`\n[*] Uploaded Metadata URI: ${uri}`);

    const metadataPDA = await metaplex.nfts().pdas().metadata({ mint });

    const tokenMetadata = {
        name: name,
        symbol: symbol,
        uri: uri,
        sellerFeeBasisPoints: 0,
        creators: null,
        collection: null,
        uses: null
    } as DataV2;

    const instruction = createCreateMetadataAccountV2Instruction(
        {
            metadata: metadataPDA,
            mint: mint,
            mintAuthority: payer.publicKey,
            payer: payer.publicKey,
            updateAuthority: payer.publicKey,
        },
        {
            createMetadataAccountArgsV2: {
                data: tokenMetadata,
                isMutable: true
            }
        }
    );

    const tx = new Transaction().add(instruction);
    const txSig = await web3.sendAndConfirmTransaction(
        connection,
        tx,
        [payer]
    );

    console.log(`\n[*] Created Metadata Account: https://explorer.solana.com/tx/${txSig}?cluster=devnet`);
};

const getCreateTokenAccount = async (
    connection: Connection,
    signer: Keypair,
    mint: PublicKey,
    owner: PublicKey,
) => {
    const tokenAccount = await token.getOrCreateAssociatedTokenAccount(
        connection,
        signer,
        mint,
        owner,
    );

    console.log(`\n[*] Token Account: https://explorer.solana.com/address/${tokenAccount.address}?cluster=devnet`);
};

async function main() {
    const connection = new Connection(web3.clusterApiUrl("devnet"));
    const user = await initializeKeypair(connection);

    // create new token mint
    const tokenMint = await createMint(
        connection,
        user,
        user.publicKey,
        user.publicKey,
        2
    );

    // get connection to metaplex
    const metaplex = Metaplex.make(connection)
        .use(keypairIdentity(user))
        .use(
            bundlrStorage({
                address: "https://devnet.bundlr.network",
                providerUrl: "https://api.devnet.solana.com",
                timeout: 60000,
            })
        );

    // create a metadata account for the token mint
    await createMetadata(
        connection,
        metaplex,
        tokenMint,
        user,
        "Blockie",
        "BLOCK",
        "The OG Blockie token"
    );

    // get or create a token account (getOrCreateAssociatedTokenAccount)
    await getCreateTokenAccount(
        connection,
        user,
        tokenMint,
        user.publicKey,
    );
}

main()
    .then(() => {
        console.log("\n[*] Finished successfully")
        process.exit(0)
    })
    .catch((error) => {
        console.log(`\n[*] Error: ${error}`)
        process.exit(1)
    })
