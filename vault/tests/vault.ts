import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider} from "@coral-xyz/anchor";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { Vault } from "../target/types/vault";
import * as bs58 from "bs58";
import wallet from "../wba-wallet.json";

describe("vault", () => {

  // Configure the client to use the local cluster
  const provider = AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Vault as Program<Vault>;

  // Create keypair from the imported wallet data
  let keypair = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(wallet)
  );
  
  console.log(keypair.publicKey.toString());

  it("Can deposit", async () => {
    const amount = new anchor.BN(1*LAMPORTS_PER_SOL); // 1 SOL
    const tx = await program.methods
      .deposit(amount)
      .accounts(
        keypair.publicKey,
      )
      .signers([keypair])
      .rpc();
    console.log("Deposit transaction signature", tx);
  });

  it("Can withdraw", async () => {
    const amount = new anchor.BN(1*LAMPORTS_PER_SOL);
    const tx = await program.methods
      .withdraw(amount)
      .accounts(
        keypair.publicKey,
      )
      .signers([keypair])
      .rpc();
    console.log("Withdraw transaction signature", tx);
  });
});