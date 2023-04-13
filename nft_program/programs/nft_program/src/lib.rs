use anchor_lang::prelude::*;

pub mod instructions;

use instructions::*;

declare_id!("GUrSjCpQUkXaooRhj7yiCdqy59qJEo5aV2JSGguyBmQ");

#[program]
pub mod nft_program {
    use super::*;

    pub fn create_token(
        ctx: Context<CreateToken>,
        nft_name: String,
        nft_symbol: String,
        nft_uri: String,
    ) -> Result<()> {
        create::create_token(ctx, nft_name, nft_symbol, nft_uri)
    }

    pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
        mint::mint_token(ctx)
    }
}
