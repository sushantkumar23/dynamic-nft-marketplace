use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize_nft(ctx: Context<InitializeNFT>, nft_address: Pubkey) -> Result<()> {
        let nft_account = &mut ctx.accounts.nft_account;
        nft_account.authority = *ctx.accounts.user.key;
        nft_account.sold = false;
        nft_account.nft_address = nft_address;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeNFT<'info> {
    #[account(init, payer = user)]
    pub nft_account: Account<'info, NFTAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NFTAccount {
    pub authority: Pubkey,
    pub sold: bool,
    pub nft_address: Pubkey,
}
