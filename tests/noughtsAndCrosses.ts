import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NoughtsAndCrosses } from "../target/types/noughts_and_crosses";
const { SystemProgram } = anchor.web3 ;


const STAKE = 1000 ;

describe("noughtsAndCrosses", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();
  const game = anchor.web3.Keypair.generate();
  const owner = anchor.web3.Keypair.generate();
  const opponent = anchor.web3.Keypair.generate();

  const boss = new anchor.web3.PublicKey("7YW6bHV4RugiazD4XEcFfp54KvgYHYZRzTAKatkQ8NrU") ;

  
  const program = anchor.workspace.NoughtsAndCrosses as Program<NoughtsAndCrosses>;



  

  // it("Create game with not enought lamports: should fail", done => {
    
  //   anchor.web3.PublicKey.findProgramAddress( 
  //     [game.publicKey.toBytes()], 
  //     program.programId
  //     ).then( gamePda => {

  //       program.methods
  //       .createGame(new anchor.BN(100) )
  //       .accounts({
  //         treasury : gamePda,
  //         owner : provider.wallet.publicKey,
  //         game : game.publicKey,
  //         systemProgram : SystemProgram.programId,
  //       })
  //       .signers([game])
  //       .rpc()
  //       .catch( err => { 
  //         done();
  //       });

  //     });


  // });
  it("Create game with enought lamports: should success", async () => {


    // await provider.connection.requestAirdrop(game.publicKey, 1);

    let [gamePda] = await anchor.web3.PublicKey.findProgramAddress( 
      [game.publicKey.toBytes()], 
      program.programId
      );

    let gamePdaSols = await provider.connection.getBalance(gamePda) ;
    console.log('game pda sols', gamePdaSols)


    await provider.connection.requestAirdrop(boss, anchor.web3.LAMPORTS_PER_SOL );

    const solsBefore = await provider.connection.getBalance(provider.wallet.publicKey) ;
    console.log('sols before', solsBefore);


    let gameSols = await provider.connection.getBalance(game.publicKey) ;
    console.log('game sols', gameSols);

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

    const solsAfter = await provider.connection.getBalance(provider.wallet.publicKey) ;
    console.log('sols after ', solsAfter);


    gamePdaSols = await provider.connection.getBalance(gamePda) ;
    console.log('game pda sols', gamePdaSols)



    gameSols = await provider.connection.getBalance(game.publicKey) ;
    console.log('game sols', gameSols);


  });


  it("Remove game", async () => {
    // Add your test here.

    const solsBefore = await provider.connection.getBalance(provider.wallet.publicKey) ;
    console.log('sols before', solsBefore);

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
      boss : boss,
      systemProgram : SystemProgram.programId
    })
    .signers([game])
    .rpc();
    console.log("Your transaction signature", tx);

    const solsAfter = await provider.connection.getBalance(provider.wallet.publicKey) ;
    console.log('sols after ', solsAfter);


    let gamePdaSols = await provider.connection.getBalance(gamePda) ;
    console.log('game pda sols', gamePdaSols)

    let gameSols = await provider.connection.getBalance(game.publicKey) ;
    console.log('game sols', gameSols);

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
