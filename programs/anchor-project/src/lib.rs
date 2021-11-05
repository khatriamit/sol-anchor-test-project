use anchor_lang::prelude::*;

declare_id!("FEhCr41qTvvkS7yK62RBKPvg7tzhtgu4k9i7buNJ9j8X");

#[program]
pub mod anchor_project {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let my_ac = &mut ctx.accounts.my_account;
        my_ac.authority = *ctx.accounts.authority.key;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 //account discriminator
        + 32 //pubkey
        + 566 //make the post max 566 bytes
    )]
    pub my_account: Account<'info, MyAccount>,
    // #[account()]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyAccount {
    pub latest_post: u64,
    pub authority: Pubkey,
}
