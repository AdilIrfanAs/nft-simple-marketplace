# Solana Simple NFT Marketplace 

## OnbaordNft
```rust 
#[derive(Accounts)]
pub struct OnbaordNft<'info> {
    #[account(init, payer = signer, space = size_of::<NftSale>() +(size_of::<Bid>())*MAX_BIDS)]
    pub nft_on_sale: Account<'info, NftSale>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```
We are onboarding new NFT to the marketplace to initiate bidding process.

```rust 
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
```
Here, users can place bids on the NFT using the NFT onboarding address.



### ⚙️ Solana Program Deployment 

Task's program code is in file lib.rs


Deployment can be done using following methods

#### 🔵 using native anchor
 - Create new program using 'anchor init' command
 - Copy content from file lib.rs
 - Paste code in newly created lib.rs file from 'anchor init' 
 - Run command 'anchor build' in root directory
 - Run command 'anchor deploy' in root directory
    (make sure to replace programId)


#### 🔵 using solpg
- Head over to https://beta.solpg.io/
- Paste the code from lib.rs
- Navigate to the left side and click on "Build & Deploy."
- After deployment, click on "Test" located on the left side.
- Interact with the "Initialize" function by providing the arguments.
- Interact with the "bid" function by providing the same address of nftOnSale as provided at the time of initialization.
- Under "Accounts," fetch NftSale with the address or use "fetchAll" to view the history.
