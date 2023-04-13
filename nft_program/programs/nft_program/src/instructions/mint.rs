use anchor_lang::prelude::*;

const TOKEN_MAX_SUPPLY: u64 = 1; // On Solana, NFTs are often thought of as SPL Tokens with 0 decimals and a supply of 1

pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
    mint_token_to_associated_token_account(&ctx)?;
    create_master_edition_account(&ctx)?;

    Ok(())
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub mint_account: Signer<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub associated_token_account: UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub master_edition_account: UncheckedAccount<'info>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn mint_token_to_associated_token_account(ctx: &Context<MintToken>) -> Result<()> {
    msg!("Minting token to Associated Token Account...");
    msg!("Mint Account {}", ctx.accounts.mint_account.key());
    msg!(
        "Associated Token Account {}",
        ctx.accounts.associated_token_account.key()
    );

    let amount = TOKEN_MAX_SUPPLY;

    anchor_spl::token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
        ),
        amount,
    )?;

    msg!(
        "Successfully minted {} token to Associated Token Account {}",
        amount,
        ctx.accounts.associated_token_account.key()
    );

    Ok(())
}

pub fn create_master_edition_account(ctx: &Context<MintToken>) -> Result<()> {
    msg!("Creating Master Edition Account...");
    msg!(
        "Master Edition Account {}",
        ctx.accounts.master_edition_account.key()
    );

    anchor_lang::solana_program::program::invoke(
        &mpl_token_metadata::instruction::create_master_edition_v3(
            ctx.accounts.token_metadata_program.key(), // Token Metadata Program ID
            ctx.accounts.master_edition_account.key(), // Master Edition Account (is PDA of ['metadata', token metadata program id, mint account, 'edition'])
            ctx.accounts.mint_account.key(),           // Mint Account
            ctx.accounts.mint_authority.key(),         // Update Authority
            ctx.accounts.mint_authority.key(),         // Mint Authority
            ctx.accounts.metadata_account.key(),       // Metadata Account
            ctx.accounts.payer.key(),                  // Payer Account
            Some(TOKEN_MAX_SUPPLY), // The maximum number of times NFTs can be printed from this Master Edition
                                    // When set to None, the program will enable unlimited prints. You can disable NFT printing by setting the Max Supply to 0
        ),
        &[
            ctx.accounts.master_edition_account.to_account_info(), // [writable] Master Edition Account (is PDA of ['metadata', token metadata program id, mint account, 'edition'])
            ctx.accounts.mint_account.to_account_info(),           // [writable] Mint Account
            ctx.accounts.mint_authority.to_account_info(),         // [signer] Update Authority
            ctx.accounts.mint_authority.to_account_info(),         // [signer] Mint Authority
            ctx.accounts.payer.to_account_info(), // [writable, signer] Payer Account
            ctx.accounts.metadata_account.to_account_info(), // [writable] Metadata Account (is PDA of ['metadata', token metadata program id, mint account])
            ctx.accounts.token_program.to_account_info(),    // [] Token Program
            ctx.accounts.system_program.to_account_info(),   // [] System Program
            ctx.accounts.rent.to_account_info(),             // [] Rent info
        ],
    )?;

    msg!(
        "Master Edition Account {} created successfully",
        ctx.accounts.master_edition_account.key()
    );

    Ok(())
}
