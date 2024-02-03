use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnT");
const GOVERNOR_PUBKEY: Pubkey = Pubkey::new_from_array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);

#[program]
mod position_router {
    use super::*;

    // Constructor equivalent in Anchor
    pub fn initialize(ctx: Context<Initialize>, min_execution_fee: u128 , usd : Pubkey , router : Pubkey , min_block_delayer_executor : u128 , min_time_delay : u128 , max_time_delay : u128 , execution_gas_limit : u128 ) -> Result<()> {
        require!(ctx.accounts.user.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);     
        let state =&mut ctx.accounts.state;
        require!(!state.initilized , MyError::AlreadyInitlized );       
        state.min_execution_fee = min_execution_fee;
        state.usd = usd;
        state.router = router;
        state.min_block_delayer_executor = min_block_delayer_executor;
        state.min_time_delay = min_time_delay;
        state.max_time_delay = max_time_delay;
        state.execution_gas_limit = execution_gas_limit;
        state.initilized = true;
        
        Ok(())
    }

    pub fn add_executor(ctx: Context<UpdateExecutor>, new_executor: Pubkey) -> Result<()> {
        // Ensure the caller is the governor
        require!(ctx.accounts.user.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);

        // Add new executor to the list
        let governance_state = &mut ctx.accounts.state;
        governance_state.executors.push(new_executor);
        Ok(())
    }

        // Function to update executor
    pub fn remove_executor(ctx: Context<UpdateExecutor>, executor: Pubkey) -> Result<()> {
            require!(ctx.accounts.user.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);
            let address_list = &mut ctx.accounts.state.executors;
            address_list.retain(|&x| x != executor);
            // Logic to update executor
            Ok(())
        }

    // Function to update delay values
    pub fn update_delay_values(ctx: Context<UpdateDelayValues> , min_block_delayer_executor : u128 , min_time_delay :u128, max_time_delay : u128 ) -> Result<()> {
        require!(ctx.accounts.user.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);
        let state =&mut ctx.accounts.state;
        state.min_block_delayer_executor = min_block_delayer_executor;
        state.min_time_delay = min_time_delay;
        state.max_time_delay = max_time_delay;
        Ok(())
    }

    // Function to update minimum execution fee
    pub fn update_min_execution_fee(ctx: Context<UpdateMinExecutionFee>, new_fee: u128) -> Result<()> {
        // Logic to update minimum execution fee
        require!(ctx.accounts.user.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);
        let state =&mut ctx.accounts.state;
        state.min_execution_fee = new_fee;

        Ok(())
    }

    // Function to update execution gas limit
    pub fn update_execution_gas_limit(ctx: Context<UpdateExecutionGasLimit>, new_gas_limt : u128 ) -> Result<()> {
        // Logic to update execution gas limit
                // Logic to update minimum execution fee
                require!(ctx.accounts.user.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);
                let state =&mut ctx.accounts.state;
                state.execution_gas_limit = new_gas_limt;
        Ok(())
    }

    // Function to create open liquidity position
    pub fn create_open_liquidity_position(ctx: Context<CreateOpenLiquidityPosition>, pool: Pubkey , margin : u128 , liquidity : u128 ) -> Result<u128> {
        // Logic to create open liquidity position
        // msg.value check 
        // external call to router 
        let value = 100;
        let state =&mut ctx.accounts.state;
        let clock: Clock = Clock::get().unwrap();
        let clock2: Clock = Clock::get()?;
        let position = OpenLiquidityPositionRequest {
           account :  ctx.accounts.user.key(),
            pool : pool,
            blockNumber : clock.slot as u128,
            liquidity: liquidity,
            executionFee : value , // change it to msg.value after wards 
            margin : margin , 
            blockTime :  clock2.unix_timestamp as u128

        };
        let positions = &mut state.open_liquidity_position_requests;
        positions.push(position);
        Ok((positions.len() as u128).into() ) 
    }

    pub fn cancel_open_liquidity_position(ctx: Context<CreateOpenLiquidityPosition>, index : u128 , execution_fee_reciever : Pubkey) -> Result<bool>{
        // should cancel check to be added 
        // transfer eth out 
        let state =&mut ctx.accounts.state;
        state.open_liquidity_position_requests.remove(index.try_into().unwrap());
        Ok(true)
    }

    pub fn execute_open_liquidity_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: u128,
        execution_fee_receiver: Pubkey,
    ) -> Result<bool> {
        
        // should execute check 
        // usdc transfer 
        // external call to plugin 
        // transfer out eth 
        let state =&mut ctx.accounts.state;
        state.open_liquidity_position_requests.remove(index.try_into().unwrap());
        Ok(true)
    }

    pub fn create_close_liquidity_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        pool: Pubkey,
        position_id: u128,
        receiver: Pubkey,
    ) -> Result<u128> {
        // Logic to create open liquidity position
        // msg.value check 
        // external call to router 
        let value = 100;
        let state =&mut ctx.accounts.state;
        let clock: Clock = Clock::get().unwrap();
        let clock2: Clock = Clock::get()?;
        let position = CloseLiquidityPositionRequest {
           account :  ctx.accounts.user.key(),
            pool : pool,
            positionID : position_id , 
            blockNumber : clock.slot as u128 ,
            receiver: receiver,
            executionFee : value , // change it to msg.value after wards
            blockTime :  clock2.unix_timestamp as u128

        };
        let positions = &mut state.close_liquidity_position_requests;
        positions.push(position);
        Ok((positions.len() as u128).into() ) 
    }

    pub fn cancel_close_liquidity_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: u128,
        execution_fee_receiver: Pubkey,
    ) -> Result<bool> {
        // should cancel check to be added 
        // transfer eth out 
        let state =&mut ctx.accounts.state;
        state.close_liquidity_position_requests.remove(index.try_into().unwrap());
        Ok(true)
    }

    pub fn execute_close_liquidity_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: u128,
        execution_fee_receiver: Pubkey,
    ) -> Result<bool> {
        // should execute check 
        // usdc transfer 
        // external call to plugin 
        // transfer out eth 
        let state =&mut ctx.accounts.state;
        state.close_liquidity_position_requests.remove(index.try_into().unwrap());
        
        Ok(true)
    }

    pub fn create_adjust_liquidity_position_margin(
        ctx: Context<CreateOpenLiquidityPosition>,
        pool: Pubkey, 
        position_id: u128, 
        margin_delta: u128, 
        receiver: Pubkey,
    ) -> Result<()> {
            // Logic to create open liquidity position
        // msg.value check 
        // external call to router 
        let value = 100;
        let state =&mut ctx.accounts.state;
        let clock: Clock = Clock::get().unwrap();
        let clock2: Clock = Clock::get()?;
        let position = AdjustLiquidityPositionMarginRequest {
           account :  ctx.accounts.user.key(),
            pool : pool,
            blockNumber : clock.slot as u128,
            executionFee : value , // change it to msg.value after wards 
           receiver : receiver,
            blockTime :  clock2.unix_timestamp as u128,
            positionID : position_id , 
            margin_delta : margin_delta , 

        };
        let positions = &mut state.adjust_liquidity_position_margin_requests;
        positions.push(position);
        Ok(()) 
        
    }

    pub fn cancel_adjust_liquidity_position_margin(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: u128,
    ) -> Result<()> {
              // should cancel check to be added 
        // transfer eth out 
        let state =&mut ctx.accounts.state;
        state.adjust_liquidity_position_margin_requests.remove(index.try_into().unwrap());
        Ok(())

    }
    
    pub fn execute_adjust_liquidity_position_margin(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: u128,
    ) -> Result<()> {
        // should execute check 
        // usdc transfer 
        // external call to plugin 
        // transfer out eth 
        let state =&mut ctx.accounts.state;
        state.adjust_liquidity_position_margin_requests.remove(index.try_into().unwrap());
        
        Ok(())
    }

    pub fn create_increase_risk_buffer_fund_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        pool: Pubkey, 
        liquidity_delta: u128,
    ) -> Result<u128> {
  // Logic to create open liquidity position
        // msg.value check 
        // external call to router 
        let value = 100;
        let state =&mut ctx.accounts.state;
        let clock: Clock = Clock::get().unwrap();
        let clock2: Clock = Clock::get()?;
        let position = IncreaseRiskBufferFundPositionRequest {
           account :  ctx.accounts.user.key(),
            pool : pool,
            blockNumber : clock.slot as u128 ,
            executionFee : value , // change it to msg.value after wards
            blockTime :  clock2.unix_timestamp as u128,
            liquidityDelta : liquidity_delta , 

        };
        let positions = &mut state.increase_risk_buffer_fund_position_request;
        positions.push(position);
        Ok((positions.len() as u128).into() ) 
    
        
    }

    pub fn cancel_increase_risk_buffer_fund_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: u128,
    ) -> Result<()> {
  // should cancel check to be added 
        // transfer eth out 
        let state =&mut ctx.accounts.state;
        state.increase_risk_buffer_fund_position_request.remove(index.try_into().unwrap());
            
        Ok(())
    }

    pub fn execute_increase_risk_buffer_fund_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: u128,
    ) -> Result<bool> {
     // should execute check 
        // usdc transfer 
        // external call to plugin 
        // transfer out eth 
        let state =&mut ctx.accounts.state;
        state.increase_risk_buffer_fund_position_request.remove(index.try_into().unwrap());
       
    
        Ok(true)
    }
    
    pub fn create_decrease_risk_buffer_fund_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        pool: Pubkey, 
        liquidity_delta: u128, 
        receiver: Pubkey,
    ) -> Result<u128> {
        let value = 100;
        let state =&mut ctx.accounts.state;
        let clock: Clock = Clock::get().unwrap();
        let clock2: Clock = Clock::get()?;
        let position = DecreaseRiskBufferFundPositionRequest {
           account :  ctx.accounts.user.key(),
            pool : pool,
            receiver : receiver , 
            liquidityDelta : liquidity_delta , 

        };
        let positions = &mut state.decrease_risk_buffer_fund_position_request;
        positions.push(position);
        Ok((positions.len() as u128).into() ) 
    }

    pub fn cancel_decrease_risk_buffer_fund_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: u128,
    ) -> Result<bool> {
        // Function logic here
        let state =&mut ctx.accounts.state;
        state.decrease_risk_buffer_fund_position_request.remove(index.try_into().unwrap());
        Ok(true) // Placeholder for the cancellation success status
    }
    
    pub fn execute_decrease_risk_buffer_fund_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: u128,
    ) -> Result<bool> {
        let state =&mut ctx.accounts.state;
        state.decrease_risk_buffer_fund_position_request.remove(index.try_into().unwrap());
        Ok(true) // Placeholder for the cancellation success status
    }

    pub fn create_increase_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        pool : Pubkey , 
        side: bool, // Assuming Side is a boolean for simplicity
        margin_delta: u128,
        size_delta: u128,
        acceptable_trade_price_x96: u128,
    ) -> Result<u128> {
       // Logic to create open liquidity position
        // msg.value check 
        // external call to router 
        let value = 100;
        let state =&mut ctx.accounts.state;
        let clock: Clock = Clock::get().unwrap();
        let clock2: Clock = Clock::get()?;
        let position = IncreasePositionRequest {
           account :  ctx.accounts.user.key(),
            pool : pool,
            side : side ,
            blockNumber : clock.slot as u128 ,
            sizeDelta : size_delta , 
            marginDelta : margin_delta , 
            acceptableTradePriceX96: acceptable_trade_price_x96,
            executionFee : value , // change it to msg.value after wards
            blockTime :  clock2.unix_timestamp as u128

        };
        let positions = &mut state.increase_position_request;
        positions.push(position);
        Ok((positions.len() as u128).into() ) 
    }

    pub fn cancel_increase_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: u128, // Assuming index is passed to identify the request
    ) -> Result<()> {
      // Function logic here
      let state =&mut ctx.accounts.state;
      state.increase_position_request.remove(index.try_into().unwrap());
      Ok(()) // Placeholder for the cancellation success status
    }

    pub fn execute_increase_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: u128,
    ) -> Result<()> {
        let state =&mut ctx.accounts.state;
        state.increase_position_request.remove(index.try_into().unwrap());
        Ok(()) // Placeholder for the cancellation success status
    }

    pub fn create_decrease_position(ctx: Context<CreateOpenLiquidityPosition>, margin_delta: u128, size_delta: u128, acceptable_trade_price_x96: u128, receiver: Pubkey , side : bool , pool : Pubkey ) -> Result<u128> {
      // Logic to create open liquidity position
        // msg.value check 
        // external call to router 
        let value = 100;
        let state =&mut ctx.accounts.state;
        let clock: Clock = Clock::get().unwrap();
        let clock2: Clock = Clock::get()?;
        let position = DecreasePositionRequest {
           account :  ctx.accounts.user.key(),
            pool : pool,
            side : side ,
            blockNumber : clock.slot as u128 ,
            sizeDelta : size_delta , 
            marginDelta : margin_delta , 
            acceptableTradePriceX96: acceptable_trade_price_x96,
            executionFee : value , // change it to msg.value after wards
            blockTime :  clock2.unix_timestamp as u128

        };
        let positions = &mut state.decrease_position_request;
        positions.push(position);
        Ok((positions.len() as u128).into() ) 
    }

    pub fn cancel_decrease_position(ctx: Context<CreateOpenLiquidityPosition>, index: u128, execution_fee_receiver: Pubkey) -> Result<()> {
      // Function logic here
      let state =&mut ctx.accounts.state;
      state.decrease_position_request.remove(index.try_into().unwrap());
      Ok(()) // Placeholder for the cancellation success status
    }

    pub fn execute_decrease_position(ctx: Context<CreateOpenLiquidityPosition>, index: u128, execution_fee_receiver: Pubkey) -> Result<()> {
        let state =&mut ctx.accounts.state;
        state.decrease_position_request.remove(index.try_into().unwrap());
        Ok(()) // Placeholder for the cancellation success status
    }
    

}

#[account]
pub struct ContractState {
    usd : Pubkey,
    router : Pubkey ,
    min_execution_fee :u128,
    min_block_delayer_executor : u128 ,
    min_time_delay :u128, 
    max_time_delay :u128, 
    execution_gas_limit :u128 ,
    executors : Vec<Pubkey> ,
    pub open_liquidity_position_requests : Vec<OpenLiquidityPositionRequest>,
    pub close_liquidity_position_requests : Vec<CloseLiquidityPositionRequest>,
    pub adjust_liquidity_position_margin_requests : Vec<AdjustLiquidityPositionMarginRequest>,
    pub increase_risk_buffer_fund_position_request : Vec<IncreaseRiskBufferFundPositionRequest>,
    pub decrease_risk_buffer_fund_position_request : Vec<DecreaseRiskBufferFundPositionRequest>,
    pub increase_position_request : Vec<IncreasePositionRequest>,
    pub decrease_position_request : Vec<DecreasePositionRequest>,
    initilized : bool,



}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct OpenLiquidityPositionRequest{
     account : Pubkey, 
    blockNumber : u128,
     pool : Pubkey  ,
    blockTime : u128 ,
    margin : u128,
    liquidity : u128,
    executionFee : u128 ,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CloseLiquidityPositionRequest{
    account: Pubkey,
    pool: Pubkey ,
    positionID: u128,
    receiver: Pubkey,
    executionFee: u128,
    blockNumber: u128,
    blockTime: u128 , 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AdjustLiquidityPositionMarginRequest{
    account: Pubkey,
    pool: Pubkey ,
    positionID: u128,
    receiver: Pubkey,
    executionFee: u128,
    blockNumber: u128,
    blockTime: u128 , 
    margin_delta : u128 , 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct IncreaseRiskBufferFundPositionRequest{
    account: Pubkey,
    pool: Pubkey,
    liquidityDelta: u128,
    executionFee: u128,
    blockNumber: u128,
    blockTime: u128 , 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct DecreaseRiskBufferFundPositionRequest{
    account: Pubkey,
    pool: Pubkey,
    liquidityDelta: u128,
    receiver: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct IncreasePositionRequest{
    account : Pubkey,
    pool : Pubkey,
    side : bool,
    marginDelta : u128,
    sizeDelta : u128 ,
    blockNumber : u128,
    blockTime :u128,
    acceptableTradePriceX96 : u128,
    executionFee:u128,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct DecreasePositionRequest{
    account : Pubkey,
    pool : Pubkey,
    side : bool,
    marginDelta : u128,
    sizeDelta : u128 ,
    blockNumber : u128,
    blockTime :u128,
    acceptableTradePriceX96 : u128,
    executionFee:u128,
}
// Context for Initialize function
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateExecutor<'info> {
    /// CHECK
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Context for UpdatePositionExecutor function
#[derive(Accounts)]
pub struct UpdatePositionExecutor<'info> {
    /// CHECK
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub user: AccountInfo<'info>,
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
    pub state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateDelayValues<'info> {
    /// CHECK
    // Fields from your Solidity struct...
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateMinExecutionFee<'info> {
    /// CHECK
    // Fields from your Solidity struct...
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateExecutionGasLimit<'info> {
    /// CHECK
    // Fields from your Solidity struct...
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub user: AccountInfo<'info>,
}
// ... Additional data structures as per your contract

// Custom errors
#[error_code]
pub enum MyError {
    #[msg("Unauthorized access")]
    CallerUnauthorized,
    #[msg("Invalid operation")]
    InvalidOperation,
    // ... Define other custom errors
    #[msg("Already Initilized")]
    AlreadyInitlized,
}

// ... Additional code as needed for your contract logic
