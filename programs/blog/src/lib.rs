use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod blog {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn init_blog(ctx: Context<InitBlog>) -> ProgramResult {
            Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
pub struct InitBlog<'info>  {
    #[account(init, payer = authority, space = 8 + 32 + 32)]
    pub blog_account: Account<'info, BlogState>,
    #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 32 +8)]
    pub genesis_post_account: Account<'info, PostState>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[account]
pub struct BlogState {
    pub current_post_key: Pubkey,
    pub authority: Pubkey,
}
