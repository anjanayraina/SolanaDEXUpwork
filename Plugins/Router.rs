use anchor_lang::{
    prelude::*
};


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


#[program]
mod router {
    use super::*;
    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<()> {
        ctx.accounts.my_account.data = data;
        Ok(())
    }

    pub fn plugin_transfer(ctx: Context<PluginTransfer>, amount: u64, from: Pubkey, to: Pubkey) -> Result<()> {
        require!(
            AUTHORIZED_ADDRESSES.contains(ctx.accounts.authorized_account.key),
            SimpleAccessControlError::Unauthorized
        );
        // token transfer logic
        Ok(())
    }

    pub fn plugin_transfer_nft(ctx: Context<PluginTransferNFT>, from: Pubkey , to:Pubkey , tokenID:u64) -> Result<()> {
        require!(
            AUTHORIZED_ADDRESSES.contains(ctx.accounts.authorized_account.key),
            SimpleAccessControlError::Unauthorized
        );
        //token transfer
        Ok(())
    }

    pub fn plugin_open_liquidity_position(ctx: Context<LiquidityPosition>, account: Pubkey, margin:u64, liquidity:u64) -> Result<u64> {
        require!(
            AUTHORIZED_ADDRESSES.contains(ctx.accounts.authorized_account.key),
            SimpleAccessControlError::Unauthorized
        );        
        
        Ok(100)
    }

    pub fn plugin_close_liquidity_position(ctx: Context<LiquidityPosition>,  _positionID:u64 ,  _receiver:Pubkey ) -> Result<()> {
        require!(
            AUTHORIZED_ADDRESSES.contains(ctx.accounts.authorized_account.key),
            SimpleAccessControlError::Unauthorized
        );        
        
      Ok(())
    }

    pub fn plugin_adjust_liquidity_position_margin(ctx: Context<LiquidityPosition>, _pool:Pubkey,
        _positionID:u64,
        _marginDelta:u64,
        _receiver:Pubkey) -> Result<()> {
            require!(
                AUTHORIZED_ADDRESSES.contains(ctx.accounts.authorized_account.key),
                SimpleAccessControlError::Unauthorized
            );        // pool open position
        // return a u64 value
      Ok(())
    }

    
      // Increase the liquidity of a risk buffer fund position
      pub fn plugin_increase_risk_buffer_fund_position(
        ctx: Context<RiskBufferFundPosition>, 
        account: Pubkey, 
        liquidity_delta: u64
    ) -> Result<()> {
        require!(
            AUTHORIZED_ADDRESSES.contains(ctx.accounts.authorized_account.key),
            SimpleAccessControlError::Unauthorized
        );        Ok(())
    }

    
    // Decrease the liquidity of a risk buffer fund position
    pub fn plugin_decrease_risk_buffer_fund_position(
        ctx: Context<RiskBufferFundPosition>, 
        account: Pubkey, 
        liquidity_delta: u64, 
        receiver: Pubkey
    ) -> Result<()> {
        require!(
            AUTHORIZED_ADDRESSES.contains(ctx.accounts.authorized_account.key),
            SimpleAccessControlError::Unauthorized
        );        Ok(())
    }

    // Increase the margin/liquidity of a position
    pub fn plugin_increase_position(
        ctx: Context<PositionManagement>, 
        account: Pubkey, 
        side: bool, 
        margin_delta: u64, 
        size_delta: u64
    ) -> Result<u64> {
        // TODO: Implement access control, position increase logic
        require!(
            AUTHORIZED_ADDRESSES.contains(ctx.accounts.authorized_account.key),
            SimpleAccessControlError::Unauthorized
        );        Ok(0) // Placeholder for trade price
    }

    // Decrease the margin/liquidity of a position
    pub fn plugin_decrease_position(
        ctx: Context<PositionManagement>, 
        account: Pubkey, 
        side: bool, 
        margin_delta: u64, 
        size_delta: u64, 
        receiver: Pubkey
    ) -> Result<u64> {
        // TODO: Implement access control, position decrease logic
        require!(
            AUTHORIZED_ADDRESSES.contains(ctx.accounts.authorized_account.key),
            SimpleAccessControlError::Unauthorized
        );        Ok(0) // Placeholder for trade price
    }

    // Close a position by the liquidator
    pub fn plugin_close_position_by_liquidator(
        ctx: Context<PositionManagement>, 
        _pool: Pubkey, 
        side: bool, 
        size_delta: u64, 
        receiver: Pubkey
    ) -> Result<()> {
        // TODO: Implement access control for liquidator, position closing logic
        require!(
            AUTHORIZED_ADDRESSES.contains(ctx.accounts.authorized_account.key),
            SimpleAccessControlError::Unauthorized
        );        Ok(())
    }

        // Collect the referral fee
    pub fn plugin_collect_referral_fee(
            ctx: Context<PositionManagement>, 
            pool: Pubkey, 
            referral_token: u64, 
            receiver: Pubkey
        ) -> Result<()> {
            // TODO: Implement access control for liquidator, position closing logic
            require!(
                AUTHORIZED_ADDRESSES.contains(ctx.accounts.authorized_account.key),
                SimpleAccessControlError::Unauthorized
            );            Ok(())
        }



  


}

#[derive(Accounts)]
pub struct PluginTransfer<'info> {
    // Define accounts needed for plugin_transfer
    // TODO: Define the account structs required for token transfer
    /// CHECK: The authorized_account is checked to be a signer, ensuring that only 

    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PluginTransferNFT<'info>  {
    // Define accounts needed for plugin_transfer_nft
    // TODO: Define the account structs required for NFT transfer
        /// CHECK: The authorized_account is checked to be a signer, ensuring that only 

        #[account(signer)]
        pub authorized_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct LiquidityPosition<'info>  {
    // Define accounts needed for liquidity position functions
    // TODO: Define the account structs required for liquidity position management
        /// CHECK: The authorized_account is checked to be a signer, ensuring that only 

        #[account(signer)]
        pub authorized_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RiskBufferFundPosition<'info>  {
    // Define accounts needed for risk buffer fund position functions
    // TODO: Define the account structs required for risk buffer fund management
    /// CHECK: The authorized_account is checked to be a signer, ensuring that only 

    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PositionManagement<'info>  {
    // Define accounts needed for position management functions
    // TODO: Define the account structs required for position management
    /// CHECK: The authorized_account is checked to be a signer, ensuring that only 

    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ReferralManagement<'info> {
    // Define accounts needed for referral management functions
    // TODO: Define the account structs required for referral management
       /// CHECK: The authorized_account is checked to be a signer, ensuring that only 
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RewardManagement<'info>  {
    // Define accounts needed for reward management functions
    // TODO: Define the account structs required for reward management
       /// CHECK: The authorized_account is checked to be a signer, ensuring that only 
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
}


#[account]
pub struct Pool {
    // Fields representing the state of the pool
}

#[account]
#[derive(Default)]
pub struct MyAccount {
    data: u64
}


#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>
}

#[error_code]
pub enum MyError {
    #[msg("Unauthorized Caller")]
    CallerUnauthorized
    ,
    #[msg("Owner Mismatch")]
    OwnerMismatch
}


// Context struct for the restricted_function
#[derive(Accounts)]
pub struct RestrictedFunction<'info> {
        /// CHECK: The caller account is marked as a signer, ensuring that the transaction is signed by the caller.

    // The account of the caller, which must sign the transaction
    #[account(signer)]
    pub caller: AccountInfo<'info>,
}

// Hardcoded list of authorized addresses
// Replace these with actual public keys of the accounts you wish to authorize
const AUTHORIZED_ADDRESSES: [Pubkey; 2] = [
    Pubkey::new_from_array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]),
    Pubkey::new_from_array([33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64]),
];

// Custom error definitions for the program
#[error_code]
pub enum SimpleAccessControlError {
    // Error when an unauthorized account tries to call the function
    #[msg("The caller is not authorized to perform this action")]
    Unauthorized,
}

