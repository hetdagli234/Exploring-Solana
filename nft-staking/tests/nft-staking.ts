import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftStaking } from "../target/types/nft_staking";
import stakerWallet from "../wba-wallet.json";
import adminWallet from "../dev-wallet.json";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync,  } from "@solana/spl-token";
import { createNft, findMasterEditionPda, findMetadataPda, mplTokenMetadata, verifyCollection, verifySizedCollectionItem } from '@metaplex-foundation/mpl-token-metadata'
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { KeypairSigner, createSignerFromKeypair, generateSigner, keypairIdentity, percentAmount } from '@metaplex-foundation/umi';
import { associatedAddress } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("nft-staking", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.NftStaking as Program<NftStaking>;
  const staker = anchor.web3.Keypair.fromSecretKey(new Uint8Array(stakerWallet));

  const umi = createUmi('https://api.devnet.solana.com');
  //Staker and the Creator are the same
  const payer = staker;

  let nftMint: KeypairSigner;
  let collectionMint: KeypairSigner;

  const creatorWallet = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(payer.secretKey));
  const creator = createSignerFromKeypair(umi, creatorWallet);
  umi.use(keypairIdentity(creator));
  umi.use(mplTokenMetadata());

  
  const admin = anchor.web3.Keypair.fromSecretKey(new Uint8Array(adminWallet));

  const config = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("config")], program.programId)[0];
  console.log(config);

  const stakerAccount = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("user"), staker.publicKey.toBuffer()], program.programId)[0];
  const stakeAccount = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("stake"), staker.publicKey.toBuffer()], program.programId)[0];
  const rewardsMint = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("rewards"), config.toBuffer()], program.programId)[0]; 

  // const rewardsMint = createMint(connection, admin, admin.publicKey, admin.publicKey, 6)

  console.log(`Staker: ${staker.publicKey.toString()}`)
  console.log(`Creator: ${creator.publicKey.toString()}`)
  console.log(`Admin: ${admin.publicKey.toString()}`)

  it("Mint Collection NFT", async () => {
    collectionMint = generateSigner(umi);
        await createNft(umi, {
            mint: collectionMint,
            name: "GM",
            symbol: "GM",
            uri: "https://arweave.net/123",
            sellerFeeBasisPoints: percentAmount(5.5),
            creators: null,
            collectionDetails: { 
              __kind: 'V1', size: 10,
            }
        }).sendAndConfirm(umi)
        console.log(`Created Collection NFT: ${collectionMint.publicKey.toString()}`)
  });

  it("Mint NFT", async () => {
    nftMint = generateSigner(umi);
        await createNft(umi, {
            mint: nftMint,
            name: "GM",
            symbol: "GM",
            uri: "https://arweave.net/123",
            sellerFeeBasisPoints: percentAmount(5.5),
            collection: {verified: false, key: collectionMint.publicKey},
            creators: null,
        }).sendAndConfirm(umi)
        console.log(`\nCreated NFT: ${nftMint.publicKey.toString()}`)
  });

  it("Initialize config!", async () => {
    const tx = await program.methods
    .initializeConfig(2 , 5, 100)
    .accountsPartial({
      admin: admin.publicKey,
      config: config,
      rewardsMint: rewardsMint,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([admin])
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Initialize user account!", async () => {
    const tx = await program.methods
    .initializeUserAccount()
    .accountsPartial({
      user: staker.publicKey,
      userAccount: stakerAccount,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([staker])
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Stake!", async () => {
    const mintAta = associatedAddress({mint: new anchor.web3.PublicKey(nftMint.publicKey.toString()), owner: staker.publicKey});

    const nftMetadata = findMetadataPda(umi, {mint: nftMint.publicKey});
    const nftEdition = findMasterEditionPda(umi, {mint: nftMint.publicKey});

    const tx = await program.methods
    .stake()
    .accountsPartial({
      user: staker.publicKey,
      mint: nftMint.publicKey,
      collection: collectionMint.publicKey,
      mintAta: mintAta,
      metadata: new anchor.web3.PublicKey(nftMetadata.toString()),
      edition: new anchor.web3.PublicKey(nftEdition.toString()),
      config,
      userAccount: stakerAccount,
      stakeAccount: stakeAccount,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([staker])
    .rpc();
    console.log("Your transaction signature", tx);
  });

  // it("Claim!", async () => {
  //   const tx = await program.methods.claim().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Unstake!", async () => {
  //   const tx = await program.methods.unstake().rpc();
  //   console.log("Your transaction signature", tx);
  // });


});
