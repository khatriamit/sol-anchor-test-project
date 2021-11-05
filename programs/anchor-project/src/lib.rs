use anchor_lang::prelude::*;
use std::str::from_utf8;

declare_id!("FEhCr41qTvvkS7yK62RBKPvg7tzhtgu4k9i7buNJ9j8X");

#[program]
pub mod anchor_project {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let my_ac = &mut ctx.accounts.my_account;
        my_ac.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn make_post(ctx: Context<MakePost>, new_post: Vec<u8>) -> ProgramResult {
        let post = from_utf8(&new_post).map_err(|err| {
            msg!("Invalid UTF-8, from byte {}", err.valid_up_to());
            ProgramError::InvalidInstructionData
        })?;
        msg!(post);

        let my_ac = &mut ctx.accounts.my_account;
        my_ac.latest_post = new_post;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 // account discriminator
        + 32 // pubkey
        + 566 // make the post max 566 bytes
    )]
    pub my_account: Account<'info, MyAccount>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MakePost<'info> {
    #[account(
        mut,
        has_one=authority
    )]
    pub my_account: Account<'info, MyAccount>,
    pub authority: Signer<'info>,
}

#[account]
pub struct MyAccount {
    pub latest_post: Vec<u8>,
    pub authority: Pubkey,
}
