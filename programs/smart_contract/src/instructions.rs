use anchor_lang::prelude::*;

use crate::state::{DidRecord,ContentSignatureRecord};

// DID management

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct RegisterDidParams {
    pub did: String,
    pub public_key: Vec<u8>,
    pub metadata: String,
    pub nickchain: String,
    pub profile_url: String,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UpdateDidParams {
    pub metadata: Option<String>,
    pub profile_url: Option<String>,
}

#[derive(Accounts)]
pub struct RegisterDid<'info> {
    #[account(init, payer = owner, space = 9000)]
    pub did_record: Account<'info, crate::state::DidRecord>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateDid<'info> {
    #[account(mut, has_one = owner)]
    pub did_record: Account<'info, DidRecord>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteDid<'info> {
    #[account(mut, has_one = owner)]
    pub did_record: Account<'info, crate::state::DidRecord>,
    pub owner: Signer<'info>,
}

// Content signatures

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateContentSignatureParams {
    pub nickchain: String,
    pub content_hash: String,
    pub num_chars: u64,
    pub datetime_created: i64,
}

#[derive(Accounts)]
pub struct CreateContentSignature<'info> {
    #[account(init, payer = user, space = 8 + 256)] // RJJ adjust size if needed
    pub content_signature_record: Account<'info, ContentSignatureRecord>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}