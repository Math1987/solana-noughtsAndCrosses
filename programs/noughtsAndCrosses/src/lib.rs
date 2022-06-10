use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, program::invoke, system_instruction}
    };
use std::str::FromStr;


declare_id!("CkxPkxnw9MHLDHVp5gNzAcTnHxLHk1pi5mwVohgjosXZ");

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
            game.players[0] = *ctx.accounts.owner.key ;
            msg!("Game {} created successfully by {} with a stake of {}", game.key(), ctx.accounts.owner.key, stake_value );
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
        msg!("Owner {} has cancel the game {} and get a refund of {}", &ctx.accounts.owner.key, game.key(), ctx.accounts.treasury.lamports() );
        Ok(())
    }
    pub fn join_game(ctx: Context<Join>) -> Result<()> {

        let game = &mut ctx.accounts.game ;

        if game.players[1].to_string() != "11111111111111111111111111111111" {
            msg!("Error joining game : the game is full.");
            Err(error!(ErrorCode::GameFull))
        }else{


            let (treasury_pda, game_seed) = Pubkey::find_program_address(&[&game.key().to_bytes()], ctx.program_id );
            let boss_pk = Pubkey::from_str("7YW6bHV4RugiazD4XEcFfp54KvgYHYZRzTAKatkQ8NrU").unwrap();
            let stake = &ctx.accounts.treasury.lamports() ;

            invoke(
                &system_instruction::transfer( &ctx.accounts.opponent.key, &treasury_pda, *stake),
                &[
                    ctx.accounts.opponent.to_account_info(),
                    ctx.accounts.treasury.to_account_info(),
                    ctx.accounts.system_program.to_account_info()
                ]
            )?;
            let comission = (ctx.accounts.treasury.lamports())*1/10 ;
            invoke_signed(
                &system_instruction::transfer( &treasury_pda, &ctx.accounts.boss.key, comission),
                &[
                    ctx.accounts.treasury.to_account_info(),
                    ctx.accounts.boss.to_account_info(),
                    ctx.accounts.system_program.to_account_info()
                ],
                &[&[
                    &game.key().to_bytes(),
                    &[game_seed],
                ]],
            )?;

            game.players[1] = *ctx.accounts.opponent.key ;
            msg!("Opponent {} joined game {} with a stake of {}", &ctx.accounts.opponent.key, &ctx.accounts.game.key(), stake );
            Ok(())

        }


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
        space = 8 + 32*2 + 64 + 8*9)]
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
    #[account(mut)]
    pub game : Signer<'info>,
    pub system_program : Program<'info, System>
}

#[derive(Accounts)]
pub struct Join<'info> {
    ///CHECK : can be unsafe
    #[account(mut)]
    pub treasury : AccountInfo<'info>,
    ///CHECK : can be unsafe
    #[account(mut)]
    pub boss : AccountInfo<'info>,
    #[account(mut)]
    pub game : Account<'info, Game>,
    pub system_program : Program<'info, System>,
    #[account(mut)]
    pub opponent : Signer<'info>
}

#[derive(Accounts)]
pub struct Play {}

#[derive(Accounts)]
pub struct Gain {}


#[account]
pub struct Game {
    pub players : [Pubkey;2],
    pub stake : u64,
    pub grid : [u8;9]
}

#[error_code]
pub enum ErrorCode {
    #[msg("Game cannot be created beacause the sake of lamports is insufficient")]
    NotEnoughtLamports,
    #[msg("Game creation fail")]
    UnknowFail,
    #[msg("Game is full")]
    GameFull
}