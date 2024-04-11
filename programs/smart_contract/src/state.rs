use anchor_lang::prelude::*;

#[account]
pub struct DidRecord {
    pub did: String,
    pub public_key: Vec<u8>,
    pub metadata: String,
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub nickchain: String,
    pub profile_url: String,
}

#[account]
pub struct ContentSignatureRecord {
    pub owner: Pubkey,
    pub nickchain: String, 
    pub url: String,
    //pub source: u64, // 1-generic_url, 2-twit, 3-image, 4-instagram, ...
    pub content_hash: String,
    pub num_chars: u64, // 0 on images
    pub datetime_created: i64, // UxTimestamp
}
