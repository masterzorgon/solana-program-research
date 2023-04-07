import { initializeKeypair } from "./initializeKeypair"
import * as web3 from "@solana/web3.js"
import { Connection, clusterApiUrl, PublicKey } from "@solana/web3.js"
import {
  Metaplex,
  keypairIdentity,
  bundlrStorage,
  toMetaplexFile,
  NftWithToken,
} from "@metaplex-foundation/js";
import * as fs from "fs";

const tokenName = "Token Name";
const description = "Description";
const symbol = "SYMBOL";
const sellerFeeBasisPoints = 100;
const imageFile_1 = "/Users/masterzorgon/Documents/Programming/personal-projects/blockchain/blockchain-research/research/nfts/src/assets/nathan-blockie.png";
const imageFile_2 = "/Users/masterzorgon/Documents/Programming/personal-projects/blockchain/blockchain-research/research/nfts/src/assets/nathan-galindo.jpg";

const updateNFT = async (
  metaplex: Metaplex,
  uri: string,
  mintAddress: PublicKey
) => {
  const nft = await metaplex.nfts().findByMint({ mintAddress });

  await metaplex
    .nfts()
    .update({
      nftOrSft: nft,
      name: tokenName,
      symbol: symbol,
      uri: uri,
      sellerFeeBasisPoints: sellerFeeBasisPoints,
    });
  
  console.log(`\n[*] Token Mint: https://explorer.solana.com/address/${nft.address.toString()}?cluster=devnet`);
};

const createNFT = async (
  metaplex: Metaplex,
  uri: string
): Promise<NftWithToken> => {
  const { nft } = await metaplex
    .nfts()
    .create({
      uri: uri,
      name: tokenName,
      sellerFeeBasisPoints: sellerFeeBasisPoints,
      symbol: symbol,
    });
  
  console.log(`\n[*] Token Mint: https://explorer.solana.com/address/${nft.address.toString()}?cluster=devnet`);

  return nft;
};

async function main() {
  const connection = new web3.Connection(web3.clusterApiUrl("devnet"));
  const user = await initializeKeypair(connection);

  console.log("PublicKey:", user.publicKey.toBase58());

  // create a metaplex instance
  const metaplex = Metaplex.make(connection)
    .use(keypairIdentity(user))
    .use(
      bundlrStorage({
        address: "https://devnet.bundlr.network",
        providerUrl: "https://api.devnet.solana.com",
        timeout: 60000,
      })
  );
  
  // file to buffer
  const buffer = fs.readFileSync(imageFile_1);

  // buffer to metaplex file
  const file = toMetaplexFile(buffer, "Blockie.png");

  // upload image and get image uri
  const imageURI = await metaplex.storage().upload(file);
  console.log(`\n[*] IMAGE URI: ${imageURI}`);

  // upload metadata and get metadata uri (off chain metadata)
  const { uri } = await metaplex
    .nfts()
    .uploadMetadata({
      name: tokenName,
      description: description,
      image: imageURI
    });
  
  console.log(`\n[*] METADATA URI: ${uri}`);

  const nft = await createNFT(metaplex, uri);

  await updateNFT(metaplex, uri, nft.address);
}

main()
  .then(() => {
    console.log("Finished successfully")
    process.exit(0)
  })
  .catch((error) => {
    console.log(error)
    process.exit(1)
  })
