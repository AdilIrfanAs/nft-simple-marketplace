se anchor_lang::prelude::*;
use core::mem::size_of;

declare_id!("EMDybNTuWUg9pQ24NE4Mu6dAEyuyUDzzLfAXn9pKqmF4");
const MAX_BIDS: usize = 30; 
#[program]
mod nft_marketplace {
    use super::*;
    pub fn initialize(ctx: Context<OnbaordNft>,nft_mint:Pubkey) -> Result<()> {
         let onboarding_nft = &mut ctx.accounts.nft_on_sale;
         onboarding_nft.nft_mint=nft_mint;
         onboarding_nft.max_bid_value=0;
        msg!("Nft onbaorded!");
        Ok(())
    }
    pub fn bid(ctx: Context<BidonNFT>,bid_value:u64) -> Result<()> {
        let nft_on_sale = &mut ctx.accounts.nft_on_sale;
        if bid_value>nft_on_sale.max_bid_value
        {
            nft_on_sale.max_bid_value=bid_value;
            nft_on_sale.bids.push( Bid { bidder: ctx.accounts.signer.key(), value: bid_value });
        }
        else{
            return err!(ErrorCode::BidLowerThanCurrentBid);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OnbaordNft<'info> {
    #[account(init, payer = signer, space = size_of::<NftSale>() +(size_of::<Bid>())*MAX_BIDS)]
    pub nft_on_sale: Account<'info, NftSale>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct BidonNFT<'info> {
    #[account(mut)]
    pub nft_on_sale: Account<'info, NftSale>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[account]
pub struct NftSale {
    max_bid_value: u64,
    nft_mint:Pubkey,
    bids:Vec<Bid>
}

#[error_code]
pub enum ErrorCode {
    #[msg("Bid Lower than current maximum Bid ")]
    BidLowerThanCurrentBid,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Bid {
    bidder: Pubkey,
    value: u64
}