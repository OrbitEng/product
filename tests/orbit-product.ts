import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { OrbitProduct } from "../target/types/orbit_product";

describe("orbit-product", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.OrbitProduct as Program<OrbitProduct>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
