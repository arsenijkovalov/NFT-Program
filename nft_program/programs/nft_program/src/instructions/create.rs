use anchor_lang::prelude::*;

const TOKEN_DECIMALS: u8 = 0; // On Solana, NFTs are often thought of as SPL Tokens with 0 decimals and a supply of 1

pub fn create_token(
    ctx: Context<CreateToken>,
    nft_name: String,
    nft_symbol: String,
    nft_uri: String,
) -> Result<()> {
    create_mint_account(&ctx)?;
    initialize_mint_account(&ctx)?;
    create_associated_token_account(&ctx)?;
    create_metadata_account(&ctx, nft_name, nft_symbol, nft_uri)?;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
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
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_mint_account(ctx: &Context<CreateToken>) -> Result<()> {
    msg!("Creating Mint Account...");
    msg!("Mint Account {}", ctx.accounts.mint_account.key());

    let space = anchor_spl::token::Mint::LEN as u64;
    let lamports = Rent::get()?.minimum_balance(space as usize);
    let owner = &ctx.accounts.token_program.key();

    anchor_lang::system_program::create_account(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_lang::system_program::CreateAccount {
                from: ctx.accounts.mint_authority.to_account_info(),
                to: ctx.accounts.mint_account.to_account_info(),
            },
        ),
        lamports,
        space,
        owner,
    )?;

    msg!(
        "Mint Account {} created successfully",
        ctx.accounts.mint_account.key()
    );

    Ok(())
}

pub fn initialize_mint_account(ctx: &Context<CreateToken>) -> Result<()> {
    msg!("Initializing Mint Account...");
    msg!("Mint Account {}", ctx.accounts.mint_account.key());

    let decimals = TOKEN_DECIMALS;
    let authority = &ctx.accounts.mint_authority.key();
    let freeze_authority = Some(authority);

    anchor_spl::token::initialize_mint(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::InitializeMint {
                mint: ctx.accounts.mint_account.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        decimals,
        authority,
        freeze_authority,
    )?;

    msg!(
        "Mint Account {} initialized successfully",
        ctx.accounts.mint_account.key()
    );

    Ok(())
}

pub fn create_associated_token_account(ctx: &Context<CreateToken>) -> Result<()> {
    msg!("Creating Associated Token Account...");
    msg!(
        "Associated Token Account {}",
        ctx.accounts.associated_token_account.key()
    );

    anchor_spl::associated_token::create(CpiContext::new(
        ctx.accounts.associated_token_program.to_account_info(),
        anchor_spl::associated_token::Create {
            payer: ctx.accounts.payer.to_account_info(),
            associated_token: ctx.accounts.associated_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
            mint: ctx.accounts.mint_account.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        },
    ))?;

    msg!(
        "Associated Token Account {} created successfully",
        ctx.accounts.associated_token_account.key()
    );

    Ok(())
}

pub fn create_metadata_account(
    ctx: &Context<CreateToken>,
    token_name: String,
    token_symbol: String,
    token_uri: String,
) -> Result<()> {
    msg!("Creating Metadata Account...");
    msg!("Metadata Account {}", ctx.accounts.metadata_account.key());

    anchor_lang::solana_program::program::invoke(
        &mpl_token_metadata::instruction::create_metadata_accounts_v3(
            ctx.accounts.token_metadata_program.key(), // Token Metadata Program ID
            ctx.accounts.metadata_account.key(), // Metadata Account (is PDA of ['metadata', token metadata program id, mint account])
            ctx.accounts.mint_account.key(),     // Mint Account
            ctx.accounts.mint_authority.key(),   // Mint Authority
            ctx.accounts.payer.key(),            // Payer Account
            ctx.accounts.mint_authority.key(),   // Update Authority
            token_name, // The on-chain name of the token, limited to 32 bytes. For instance "Degen Ape #1"
            token_symbol, // The on-chain symbol of the token, limited to 10 bytes. For instance "DAPE"
            token_uri,    // The URI of the token, limited to 200 bytes
            Some(vec![
                // An array of creators and their share of the royalties. This array is limited to 5 creators
                mpl_token_metadata::state::Creator {
                    address: ctx.accounts.mint_authority.key(), // The public key of the creator
                    share: 100, // The creator's shares of the royalties in percentage (1 byte) — i.e. 55 means 55%
                    verified: false, // A boolean indicating if the creator signed the NFT
                },
            ]),
            100,   // The royalties shared by the creators in basis points — i.e. 550 means 5.5%
            true,  // A boolean indicating if the Update Authority is a signer
            false, // A boolean indicating if the Metadata Account can be updated. Once flipped to False, it cannot ever be True again
            None, // This field optionally links to the Mint address of another NFT that acts as a Collection NFT
            None, // This field can make NFTs usable. Meaning you can load it with a certain amount of "uses" and use it until it has run out
            None, // This optional enum allows us to differentiate Collection NFTs from Regular NFTs whilst
                  // adding additional context such as the amount of NFTs that are linked to the Collection NFT
        ),
        &[
            ctx.accounts.metadata_account.to_account_info(), // [writable] Metadata Account (is PDA of ['metadata', token metadata program id, mint account])
            ctx.accounts.mint_account.to_account_info(),     // [] Mint account
            ctx.accounts.mint_authority.to_account_info(),   // [signer] Mint Authority
            ctx.accounts.payer.to_account_info(),            // [writable, signer] Payer Account
            ctx.accounts.mint_authority.to_account_info(),   // [signer] Update Authority
            ctx.accounts.system_program.to_account_info(),   // [] System Program
            ctx.accounts.rent.to_account_info(),             // Optional [] Rent info
        ],
    )?;

    msg!(
        "Metadata Account {} created successfully",
        ctx.accounts.metadata_account.key()
    );

    Ok(())
}
