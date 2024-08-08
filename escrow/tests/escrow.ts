import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import makerWallet from "../wba-wallet.json";
import takerWallet from "../dev-wallet.json";
import { PublicKey } from "@solana/web3.js";
import { airdropSol } from "@lightprotocol/stateless.js";
import { transfer, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const maker = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(makerWallet)
  );

  const taker = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(takerWallet)
  );

  const connection = new anchor.web3.Connection(anchor.web3.clusterApiUrl("devnet"));

  console.log(taker.publicKey.toString());
  const program = anchor.workspace.Escrow as Program<Escrow>;

  it("Making!", async () => {
    // Add your test here.
    const tx = await program.methods
    .make(new BN(252), new BN(1_000_000), new BN(500_000))
    .accountsPartial({
      maker: maker.publicKey,
      mintA: new PublicKey("CW3fBWxSVuaTA2GjAEoAhdftgG4F3RS7Caksu9hGR1EY"),
      mintB: new PublicKey("CW3fBWxSVuaTA2GjAEoAhdftgG4F3RS7Caksu9hGR1EY"), 
      escrow: PublicKey.findProgramAddressSync(
        [Buffer.from("escrow"), maker.publicKey.toBuffer(), new BN(252).toArrayLike(Buffer, 'le', 8)],
        program.programId
      )[0],
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .signers([maker])
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Taking!", async () => {
    // Add your test here.
    const tx = await program.methods
      .take()
      .accountsPartial({
        taker: taker.publicKey,
        maker: maker.publicKey,
        mintA: new PublicKey("CW3fBWxSVuaTA2GjAEoAhdftgG4F3RS7Caksu9hGR1EY"),
        mintB: new PublicKey("CW3fBWxSVuaTA2GjAEoAhdftgG4F3RS7Caksu9hGR1EY"),
        escrow: PublicKey.findProgramAddressSync(
          [Buffer.from("escrow"), maker.publicKey.toBuffer(), new BN(252).toArrayLike(Buffer, 'le', 8)],
          program.programId
        )[0],
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([taker])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  // it("Refunding!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });
});