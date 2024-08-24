import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AlephAndromedaToken } from "../target/types/aleph_andromeda_token";

describe("aleph-andromeda-token", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AlephAndromedaToken as Program<AlephAndromedaToken>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
