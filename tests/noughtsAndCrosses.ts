import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NoughtsAndCrosses } from "../target/types/noughts_and_crosses";


const STAKE = 100000 ;

describe("noughtsAndCrosses", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NoughtsAndCrosses as Program<NoughtsAndCrosses>;

  it("Create game", async () => {
    // Add your test here.
    const tx = await program.methods.createGame(new anchor.BN(STAKE) ).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Remove game", async () => {
    // Add your test here.
    const tx = await program.methods.removeGame().rpc();
    console.log("Your transaction signature", tx);
  });


  it("Join game", async () => {
    // Add your test here.
    const tx = await program.methods.joinGame().rpc();
    console.log("Your transaction signature", tx);
  });


  it("play", async () => {
    // Add your test here.
    const tx = await program.methods.play(new anchor.BN(0)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("get gain", async () => {
    // Add your test here.
    const tx = await program.methods.getGain().rpc();
    console.log("Your transaction signature", tx);
  });

  

});
