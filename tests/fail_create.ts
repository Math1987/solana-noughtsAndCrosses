import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NoughtsAndCrosses } from "../target/types/noughts_and_crosses";
const { SystemProgram } = anchor.web3 ;

describe("noughtsAndCrosses create fail", async () => {

  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();
  const game = anchor.web3.Keypair.generate();
  const program = anchor.workspace.NoughtsAndCrosses as Program<NoughtsAndCrosses>;


  it("Create game with not enought lamports: should fail", done => {
    
    anchor.web3.PublicKey.findProgramAddress( 
      [game.publicKey.toBytes()], 
      program.programId
      ).then( gamePda => {

        program.methods
        .createGame(new anchor.BN(100) )
        .accounts({
          treasury : gamePda,
          owner : provider.wallet.publicKey,
          game : game.publicKey,
          systemProgram : SystemProgram.programId,
        })
        .signers([game])
        .rpc()
        .catch( err => { 
          done();
        });

      });
  });


});
