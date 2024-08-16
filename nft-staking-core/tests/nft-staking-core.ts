import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftStakingCore } from "../target/types/nft_staking_core";

describe("nft-staking-core", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NftStakingCore as Program<NftStakingCore>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
