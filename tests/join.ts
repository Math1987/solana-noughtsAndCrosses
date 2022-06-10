import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NoughtsAndCrosses } from "../target/types/noughts_and_crosses";
const { SystemProgram } = anchor.web3 ;
import { expect } from "chai";


const STAKE = 100000000 ;

describe("noughtsAndCrosses Join", async () => {

  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();
  const game = anchor.web3.Keypair.generate();
  const opponent = anchor.web3.Keypair.generate();

  const boss = new anchor.web3.PublicKey("7YW6bHV4RugiazD4XEcFfp54KvgYHYZRzTAKatkQ8NrU") ;
  const program = anchor.workspace.NoughtsAndCrosses as Program<NoughtsAndCrosses>;
  
  it("Create game with enought lamports: should success", async () => {

    await provider.connection.requestAirdrop(boss, anchor.web3.LAMPORTS_PER_SOL );
    let [gamePda] = await anchor.web3.PublicKey.findProgramAddress( 
      [game.publicKey.toBytes()], 
      program.programId
      );
      
    await provider.connection.requestAirdrop(game.publicKey, anchor.web3.LAMPORTS_PER_SOL );
    await provider.connection.requestAirdrop(boss, anchor.web3.LAMPORTS_PER_SOL );

    const tx = await program.methods
    .createGame(new anchor.BN(STAKE) )
    .accounts({
      treasury : gamePda,
      owner : provider.wallet.publicKey,
      game : game.publicKey,
      systemProgram : SystemProgram.programId
    })
    .signers([game])
    .rpc();

    const gameResult = await program.account.game.fetch(game.publicKey);
    expect(gameResult.players[0].toString()).equal(provider.wallet.publicKey.toString());

  });


  it("Join game", async () => {


    const signatureAirdrop = await provider.connection.requestAirdrop( opponent.publicKey, anchor.web3.LAMPORTS_PER_SOL*2);
    await provider.connection.confirmTransaction(signatureAirdrop) ;

    let [gamePda] = await anchor.web3.PublicKey.findProgramAddress( 
      [game.publicKey.toBytes()], 
      program.programId
      );

    // Add your test here.

    const tx = await program.methods
    .joinGame()
    .accounts({
      treasury : gamePda,
      opponent : opponent.publicKey,
      game : game.publicKey,
      boss : boss,
      systemProgram : SystemProgram.programId
    })
    .signers([opponent])
    .rpc();


    const gameResult = await program.account.game.fetch(game.publicKey);
    expect(gameResult.players[0].toString()).equal(provider.wallet.publicKey.toString());
    expect(gameResult.players[1].toString()).equal(opponent.publicKey.toString());


    const treasurySol = await provider.connection.getBalance(gamePda);
    expect(treasurySol).equal(STAKE*2*0.9);

  });


  it("play", async () => {
    // Add your test here.
    const tx = await program.methods.play(new anchor.BN(0)).rpc();
  });

  it("get gain", async () => {
    // Add your test here.
    const tx = await program.methods.getGain().rpc();
  });

  

});
