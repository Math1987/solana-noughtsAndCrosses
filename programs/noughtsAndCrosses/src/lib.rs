use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, program::invoke, system_instruction}
    };
use std::str::FromStr;


declare_id!("CkxPkxnw9MHLDHVp5gNzAcTnHxLHk1pi5mwVohgjosXZ");

const TREASURY_PDA_SEED : &[u8] = b"treasury" ;

#[program]
pub mod noughts_and_crosses {
    use super::*;

    //CREATE GAME
    //A player create a game, set an amount of stake
    //A game pda is created with the datas of the game
    //a treasury pda is created and receive the amount of stake inside
    
    pub fn create_game(ctx: Context<Create>, stake_value : u64) -> Result<()> {
        if stake_value >= 1000000 {

            let game = &mut ctx.accounts.game ;
            let (treasury_pda, game_seed) = Pubkey::find_program_address(&[&game.key().to_bytes()], ctx.program_id );

            invoke(
                &system_instruction::transfer( &ctx.accounts.owner.key, &treasury_pda, stake_value),
                &[
                    ctx.accounts.owner.to_account_info(),
                    ctx.accounts.treasury.to_account_info(),
                    ctx.accounts.system_program.to_account_info()
                ]
            )?;


            msg!("create game ok {}", stake_value );
            Ok(())
        }else{
            msg!("create game fail: not enought stake {}", stake_value );
            Err(error!(ErrorCode::NotEnoughtLamports))
        }
    }
    pub fn remove_game(ctx: Context<Remove>) -> Result<()> {

        let game = &mut ctx.accounts.game ;
        let (treasury_pda, game_seed) = Pubkey::find_program_address(&[&game.key().to_bytes()], ctx.program_id );

        let boss_pk = Pubkey::from_str("7YW6bHV4RugiazD4XEcFfp54KvgYHYZRzTAKatkQ8NrU").unwrap();


        msg!("lamports in game {}", &ctx.accounts.treasury.lamports() );


        invoke_signed(
            &system_instruction::transfer(  &treasury_pda, &ctx.accounts.owner.key, ctx.accounts.treasury.lamports()),
            &[
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.system_program.to_account_info()
            ],
            &[&[
                &game.key().to_bytes(),
                &[game_seed],
            ]],
        )?;

        // invoke(
        //     &system_instruction::transfer( &ctx.accounts.game.key, &ctx.accounts.owner.key, 890880),
        //     &[
        //         ctx.accounts.game.to_account_info(),
        //         ctx.accounts.owner.to_account_info(),
        //         ctx.accounts.system_program.to_account_info()
        //     ]
        // )?;

        msg!("remove game ok" );
        Ok(())
    }
    pub fn join_game(ctx: Context<Join>) -> Result<()> {
        msg!("join ok");
        Ok(())
    }
    pub fn play(ctx: Context<Play>, case : u64) -> Result<()> {
        msg!("play on {}", case );
        Ok(())
    }
    pub fn get_gain(ctx: Context<Gain>) -> Result<()> {
        msg!("getting gain (if the opponent has leaved before end).");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    ///CHECK : can be unsafe
    #[account(mut)]
    pub treasury : AccountInfo<'info>,
    #[account(mut)]
    pub owner : Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + 64 + 8*9)]
    pub game : Account<'info, Game>,
    pub system_program : Program<'info, System>
}

#[derive(Accounts)]
pub struct Remove<'info> {
    ///CHECK : can be unsafe
    #[account(mut)]
    pub treasury : AccountInfo<'info>,
    ///CHECK : can be unsafe
    #[account(mut)]
    pub owner : AccountInfo<'info>,
    ///CHECK : can be unsafe
    #[account(mut)]
    pub boss : AccountInfo<'info>,
    #[account(mut)]
    pub game : Signer<'info>,
    pub system_program : Program<'info, System>
}

#[derive(Accounts)]
pub struct Join {}

#[derive(Accounts)]
pub struct Play {}

#[derive(Accounts)]
pub struct Gain {}


#[account]
pub struct Game {
    pub stake : u64,
    pub grid : [u8;9]
}

#[error_code]
pub enum ErrorCode {
    #[msg("Game cannot be created beacause the sake of lamports is insufficient")]
    NotEnoughtLamports,
    #[msg("Game creation fail")]
    UnknowFail,
}