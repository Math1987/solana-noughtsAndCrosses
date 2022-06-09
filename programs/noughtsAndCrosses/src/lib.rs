use anchor_lang::prelude::*;

declare_id!("CkxPkxnw9MHLDHVp5gNzAcTnHxLHk1pi5mwVohgjosXZ");

#[program]
pub mod noughts_and_crosses {
    use super::*;

    pub fn create_game(ctx: Context<Create>, stake_value : u64) -> Result<()> {
        msg!("create ok {}", stake_value );
        Ok(())
    }
    pub fn remove_game(ctx: Context<Remove>) -> Result<()> {
        msg!("leave ok",  );
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
pub struct Create {}

#[derive(Accounts)]
pub struct Remove {}

#[derive(Accounts)]
pub struct Join {}

#[derive(Accounts)]
pub struct Play {}

#[derive(Accounts)]
pub struct Gain {}