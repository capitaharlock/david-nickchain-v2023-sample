use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;
pub mod processor;

declare_id!("GMcDYK145SfvFszTnE27DUkZYTd1X5Ug5FCWrsSuv3Ym");

use instructions::*;

#[program]
pub mod did_registry {
    use super::*;
    
    pub fn register_did(ctx: Context<RegisterDid>, params: RegisterDidParams) -> Result<()> {
        processor::process_register_did(ctx, params)
    }
    
    pub fn update_did(
        ctx: Context<UpdateDid>, 
        metadata: Option<String>, 
        profile_url: Option<String>
    ) -> Result<()> {
        let params = UpdateDidParams {
            metadata,
            profile_url,
        };
        processor::process_update_did(ctx, params)
    }

    pub fn delete_did(ctx: Context<DeleteDid>) -> Result<()> {
        processor::process_delete_did(ctx)
    }

    pub fn create_content_signature(ctx: Context<CreateContentSignature>, params: CreateContentSignatureParams) -> Result<()> {
        processor::process_create_content_signature(ctx, params)
    }
}
