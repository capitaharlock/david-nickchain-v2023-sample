use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::instructions::{
    RegisterDid, RegisterDidParams, UpdateDid, UpdateDidParams, DeleteDid,
    CreateContentSignature, CreateContentSignatureParams
};

pub fn process_register_did(ctx: Context<RegisterDid>, params: RegisterDidParams) -> Result<()> {
    let record = &mut ctx.accounts.did_record;
    if record.is_initialized {
        return Err(error!(ErrorCode::DidAlreadyExists));
    }
    record.did = params.did;
    record.public_key = params.public_key;
    record.metadata = params.metadata;
    record.is_initialized = true;
    record.owner = *ctx.accounts.owner.key;
    record.nickchain = params.nickchain;
    record.profile_url = params.profile_url;
    Ok(())
}

pub fn process_update_did(ctx: Context<UpdateDid>, params: UpdateDidParams) -> Result<()> {
    let record = &mut ctx.accounts.did_record;
    if !record.is_initialized {
        return Err(error!(ErrorCode::DidNotExists));
    }

    // Update data
    if let Some(metadata) = params.metadata {
        record.metadata = metadata;
    }
    if let Some(profile_url) = params.profile_url {
        record.profile_url = profile_url;
    }
    
    Ok(())
}

// RJJ | Consider reuse of that accounts to save some lamports. Option B: add Solana close instructions (better fro tokens than data)
pub fn process_delete_did(ctx: Context<DeleteDid>) -> Result<()> {
    let record = &mut ctx.accounts.did_record;
    if !record.is_initialized {
        return Err(error!(ErrorCode::DidNotExists));
    }

    // Clean user's sensitive data
    record.did = String::new();
    record.public_key = Vec::new();
    record.metadata = String::new();
    record.nickchain = String::new();
    record.profile_url = String::new();
    // Owner kept | (privacy requirements)
    
    record.is_initialized = false;
    Ok(())
}

// Content signatures
pub fn process_create_content_signature(ctx: Context<CreateContentSignature>, params: CreateContentSignatureParams) -> Result<()> {
    // Seed |  nickchain and content_hash
    let bump = *ctx.bumps.get("content_signature_record").expect("bump not found");
    
    let (pda, _bump_seed) = Pubkey::find_program_address(
        &[&params.nickchain.as_bytes(), &params.content_hash.as_bytes(), &[bump]],
        ctx.program_id,
    );

    // Verify that the derived PDA matches the expected account's key
    if pda != ctx.accounts.content_signature_record.key() {
        return Err(ErrorCode::PdaMismatch.into());
    }

    // Initialize the content signature record
    let record = &mut ctx.accounts.content_signature_record;
    record.owner = ctx.accounts.user.key();
    record.nickchain = params.nickchain;
    record.content_hash = params.content_hash;
    record.num_chars = params.num_chars;
    record.datetime_created = params.datetime_created;

    Ok(())
}
