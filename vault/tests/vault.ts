import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider} from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import * as bs58 from "bs58";
import wallet from "../wba-wallet.json";

describe("vault", () => {

  const program = anchor.workspace.Vault as Program<Vault>;
  // Create keypair from the imported wallet data
  let keypair = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(wallet)
  );
  
  console.log(keypair.publicKey.toString());
  const [vault, vaultBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault")],
    program.programId
  );

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
    .initialize()
    .accounts(
      keypair.publicKey,
    )
    .signers([keypair])
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Can deposit", async () => {
    const amount = new anchor.BN(100000000); // 1 SOL
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
    const amount = new anchor.BN(50000000); // 0.5 SOL
    const tx = await program.methods
      .withdraw(amount)
      .accounts(
        keypair.publicKey,
      )
      .signers([keypair])
      .rpc();
    console.log("Withdraw transaction signature", tx);
  });

  it("Can close", async () => {
    const tx = await program.methods
      .close()
      .accounts(
        keypair.publicKey,
      )
      .signers([keypair])
      .rpc();
    console.log("Close transaction signature", tx);
  });
});