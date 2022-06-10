import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NoughtsAndCrosses } from "../target/types/noughts_and_crosses";
const { SystemProgram } = anchor.web3 ;


const STAKE = 1000 ;

describe("noughtsAndCrosses remove", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();
  const game = anchor.web3.Keypair.generate();
  // const owner = anchor.web3.Keypair.generate();
  const opponent = anchor.web3.Keypair.generate();

  const boss = new anchor.web3.PublicKey("7YW6bHV4RugiazD4XEcFfp54KvgYHYZRzTAKatkQ8NrU") ;

  
  const program = anchor.workspace.NoughtsAndCrosses as Program<NoughtsAndCrosses>;


  it("Create game with enought lamports: should success", async () => {

    let [gamePda] = await anchor.web3.PublicKey.findProgramAddress( 
      [game.publicKey.toBytes()], 
      program.programId
      );


    await provider.connection.requestAirdrop(boss, anchor.web3.LAMPORTS_PER_SOL );

    const tx = await program.methods
    .createGame(new anchor.BN(100000000) )
    .accounts({
      treasury : gamePda,
      owner : provider.wallet.publicKey,
      game : game.publicKey,
      systemProgram : SystemProgram.programId
    })
    .signers([game])
    .rpc();

  });


  it("Remove game", async () => {

    let [gamePda] = await anchor.web3.PublicKey.findProgramAddress( 
      [game.publicKey.toBytes()], 
      program.programId
      );

    const tx = await program.methods
    .removeGame()
    .accounts({
      treasury : gamePda,
      owner : provider.wallet.publicKey,
      game : game.publicKey,
      systemProgram : SystemProgram.programId
    })
    .signers([game])
    .rpc();

  });

});
