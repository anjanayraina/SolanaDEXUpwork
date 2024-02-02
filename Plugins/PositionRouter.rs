use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnT");
const GOVERNOR_PUBKEY: Pubkey = Pubkey::new_from_array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);

#[program]
mod position_router {
    use super::*;

    // Constructor equivalent in Anchor
    pub fn initialize(ctx: Context<Initialize>, min_execution_fee: u64) -> Result<()> {
        // Initialization logic here
        Ok(())
    }

    // // Function to update position executor
    // pub fn update_position_executor(ctx: Context<UpdatePositionExecutor>, executor: Pubkey, active: bool) -> Result<()> {
    //     // Logic to update position executor
    //     Ok(())
    // }

    // // Function to update delay values
    // pub fn update_delay_values(ctx: Context<UpdateDelayValues>) -> Result<()> {
    //     // Logic to update delay values
    //     Ok(())
    // }

    // // Function to update minimum execution fee
    // pub fn update_min_execution_fee(ctx: Context<UpdateMinExecutionFee>, new_fee: u64) -> Result<()> {
    //     // Logic to update minimum execution fee
    //     Ok(())
    // }

    // // Function to update execution gas limit
    // pub fn update_execution_gas_limit(ctx: Context<UpdateExecutionGasLimit>, /* parameters */) -> Result<()> {
    //     // Logic to update execution gas limit
    //     Ok(())
    // }

    // // Function to create open liquidity position
    // pub fn create_open_liquidity_position(ctx: Context<CreateOpenLiquidityPosition>, /* parameters */) -> Result<u128> {
    //     // Logic to create open liquidity position
    //     Ok(0) // Placeholder return
    // }

    // ... Additional functions as per your contract

}

#[account]
pub struct ContractState {
    usd : Pubkey,
    router : Pubkey ,
    min_execution_fee :u64,
    min_block_delayer_executor : u64 ,
    min_time_delay :u64, 
    max_time_delay :u64, 
    execution_gas_limit :u64 ,
    executor : Vec<bool> ,
    pub open_liquidity_position_requests : Vec<OpenLiquidityPositionRequest>,
    pub close_liquidity_position_requests : Vec<CloseLiquidityPositionRequest>,
    pub adjust_liquidity_position_margin_requests : Vec<AdjustLiquidityPositionMarginRequest>,
    pub increase_risk_buffer_fund_position_request : Vec<IncreaseRiskBufferFundPositionRequest>,
    pub decrease_risk_buffer_fund_position_request : Vec<DecreaseRiskBufferFundPositionRequest>,
    pub increase_position_request : Vec<IncreasePositionRequest>,
    pub decrease_position_request : Vec<DecreasePositionRequest>,



}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct OpenLiquidityPositionRequest{
    account : Pubkey,
    pool : Pubkey,
    side : bool,
    marginDelta : u64,
    sizeDelta : u64 ,
    triggerMarketPriceX96 : u64,
    triggerAbove :bool,
    acceptableTradePriceX96 : u64,
    executionFee:u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CloseLiquidityPositionRequest{
    account : Pubkey,
    pool : Pubkey,
    side : bool,
    marginDelta : u64,
    sizeDelta : u64 ,
    triggerMarketPriceX96 : u64,
    triggerAbove :bool,
    acceptableTradePriceX96 : u64,
    executionFee:u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AdjustLiquidityPositionMarginRequest{
    account : Pubkey,
    pool : Pubkey,
    side : bool,
    marginDelta : u64,
    sizeDelta : u64 ,
    triggerMarketPriceX96 : u64,
    triggerAbove :bool,
    acceptableTradePriceX96 : u64,
    executionFee:u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct IncreaseRiskBufferFundPositionRequest{
    account : Pubkey,
    pool : Pubkey,
    side : bool,
    marginDelta : u64,
    sizeDelta : u64 ,
    triggerMarketPriceX96 : u64,
    triggerAbove :bool,
    acceptableTradePriceX96 : u64,
    executionFee:u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct DecreaseRiskBufferFundPositionRequest{
    account : Pubkey,
    pool : Pubkey,
    side : bool,
    marginDelta : u64,
    sizeDelta : u64 ,
    triggerMarketPriceX96 : u64,
    triggerAbove :bool,
    acceptableTradePriceX96 : u64,
    executionFee:u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct IncreasePositionRequest{
    account : Pubkey,
    pool : Pubkey,
    side : bool,
    marginDelta : u64,
    sizeDelta : u64 ,
    triggerMarketPriceX96 : u64,
    triggerAbove :bool,
    acceptableTradePriceX96 : u64,
    executionFee:u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct DecreasePositionRequest{
    account : Pubkey,
    pool : Pubkey,
    side : bool,
    marginDelta : u64,
    sizeDelta : u64 ,
    triggerMarketPriceX96 : u64,
    triggerAbove :bool,
    acceptableTradePriceX96 : u64,
    executionFee:u64,
}
// Context for Initialize function
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK
    pub position_router_state: Account<'info, ContractState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Context for UpdatePositionExecutor function
#[derive(Accounts)]
pub struct UpdatePositionExecutor<'info> {
    /// CHECK
    #[account(mut)]
    pub position_router_state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}

// ... Additional Context structs for other functions

// PositionRouter state account


// Struct corresponding to IncreasePositionRequest in Solidity
#[derive(Accounts)]
pub struct IncreasePositionRequestContext<'info> {
    // Fields from your Solidity struct...
    /// CHECK
    #[account(mut)]
    pub position_router_state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}

// Struct corresponding to DecreasePositionRequest in Solidity
#[derive(Accounts)]
pub struct DecreasePositionRequestContext<'info> {
    /// CHECK
    // Fields from your Solidity struct...
    #[account(mut)]
    pub position_router_state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateOpenLiquidityPosition<'info> {
    /// CHECK
    // Fields from your Solidity struct...
    #[account(mut)]
    pub position_router_state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateDelayValues<'info> {
    /// CHECK
    // Fields from your Solidity struct...
    #[account(mut)]
    pub position_router_state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateMinExecutionFee<'info> {
    /// CHECK
    // Fields from your Solidity struct...
    #[account(mut)]
    pub position_router_state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateExecutionGasLimit {
    
    // Fields from your Solidity struct...
}
// ... Additional data structures as per your contract

// Custom errors
#[error_code]
pub enum PositionRouterError {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid operation")]
    InvalidOperation,
    // ... Define other custom errors
}

// ... Additional code as needed for your contract logic
