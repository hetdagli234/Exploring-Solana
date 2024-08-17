import * as anchor from "@coral-xyz/anchor";
import { AnchorProvider, Program } from "@coral-xyz/anchor";
import { NftStakingCore } from "../target/types/nft_staking_core";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, generateSigner, signerIdentity, publicKey, TransactionBuilder } from "@metaplex-foundation/umi";
import { MPL_CORE_PROGRAM_ID, mplCore, createV1, transfer, transferV1 } from '@metaplex-foundation/mpl-core'
import { randomBytes } from "crypto";
import creator from "../dev-wallet.json";
import stake from "../wba-wallet.json";
import { BN } from "bn.js";
import { assert } from "chai";
import { Connection } from "@solana/web3.js";

describe("nft-staking-core", () => {
  // Configure the client to use the local cluster.
  // anchor.setProvider(anchor.AnchorProvider.env());

  const creatorWallet = anchor.web3.Keypair.fromSecretKey(new Uint8Array(creator));
  const stakeWallet = anchor.web3.Keypair.fromSecretKey(new Uint8Array(stake));
  const creatorProvider = new AnchorProvider(new Connection("https://api.devnet.solana.com"), new anchor.Wallet(creatorWallet));
  const stakeProvider = new AnchorProvider(new Connection("https://api.devnet.solana.com"), new anchor.Wallet(stakeWallet));

  const umi = createUmi("https://api.devnet.solana.com");
  let umiKeypair = umi.eddsa.createKeypairFromSecretKey(creatorWallet.secretKey);
  const signerKeypair = createSignerFromKeypair(umi, umiKeypair);
  umi.use(signerIdentity(signerKeypair)).use(mplCore());

  const program = anchor.workspace.NftStakingCore as Program<NftStakingCore>;

  const config = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config"),
    creatorWallet.publicKey.toBuffer(),
    ],
    program.programId
  )[0];

  const userStakeAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("stake_account"),
    stakeWallet.publicKey.toBuffer(),
    config.toBuffer(),
    ],
    program.programId
  )[0];

  const asset = generateSigner(umi);

  it("Mint the asset!", async () => {
    // Add your test here.
    const tx = await new TransactionBuilder().add(
      createV1(umi, {
        asset,
        name: "Test Asset",
        uri: "https://arweave.net/ru8Smt5_2-ucHlCaExZifbf_cPFxwqJXBhTjpFK22Ug",
        owner: publicKey(stakeWallet.publicKey),
        updateAuthority: publicKey(stakeWallet.publicKey),
      })
    ).sendAndConfirm(umi);
  });

  it("Initialize config!", async () => {

    const program = programPaidBy(creatorWallet);

    anchor.setProvider(creatorProvider);
    const seed = new BN(randomBytes(8));
    const tx = await program.methods.initConfig(seed)
    .accountsPartial({
      updateAuthority: creatorWallet.publicKey,
      config: config,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([creatorWallet])
    .rpc();
    console.log("Your transaction signature for config initialization", tx);
  });

  it("Stake!", async () => {
    const program = programPaidBy(stakeWallet);

    const tx = await program.methods.stake()
    .accountsPartial({
      config: config,
      updateAuthority: creatorWallet.publicKey,
      owner: stakeWallet.publicKey,
      asset: asset.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      userStakeAccount: userStakeAccount,
      coreProgram: MPL_CORE_PROGRAM_ID,
    })
    .signers([stakeWallet])
    .rpc();

    console.log("Your transaction signature for staking", tx);

    let stakeAccount = await program.account.stakeAccount.fetch(userStakeAccount);

    assert.equal(stakeAccount.points.toNumber(), 0);
  });

  it("Unstake!", async () => {
    const program = programPaidBy(stakeWallet);
    const tx = await program.methods.unstake()
    .accountsPartial({
      config: config,
      updateAuthority: creatorWallet.publicKey,
      owner: stakeWallet.publicKey,
      asset: asset.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      userStakeAccount: userStakeAccount,
      coreProgram: MPL_CORE_PROGRAM_ID,
    })
    .signers([stakeWallet])
    .rpc();

    console.log("Your transaction signature for unstaking", tx);

  });
});

export function programPaidBy(payer: anchor.web3.Keypair): anchor.Program {
  const newProvider = new AnchorProvider(new Connection("https://api.devnet.solana.com"), new anchor.Wallet(payer));
  const program = anchor.workspace.NftStakingCore as Program<NftStakingCore>;

  return new anchor.Program(program.idl as anchor.Idl, newProvider)
}
