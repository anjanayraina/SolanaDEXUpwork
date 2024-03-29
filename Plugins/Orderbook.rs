use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLn2");
const GOVERNOR_PUBKEY: Pubkey = Pubkey::new_from_array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);
#[program]
pub mod order_book {
    use super::*;

    // Initialize the order book
    pub fn initialize(ctx: Context<Initialize>, min_execution_fee: u64 , execution_gas_limit : u64 , increase_order_index : u64 , decrease_order_index : u64 , usd : Pubkey , router : Pubkey ) -> Result<()> {
        require!(ctx.accounts.authorized_account.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);     
        let state =&mut ctx.accounts.state;
        require!(!state.initilized , MyError::AlreadyInitlized );
        state.min_execution_fee = min_execution_fee;
        state.execution_gas_limit = execution_gas_limit;
        state.increase_order_index = increase_order_index;
        state.decrease_order_index;
        state.usd = usd;
        state.router = router;
        Ok(())
    }

    pub fn add_executor(ctx: Context<UpdateExecutor>, new_executor: Pubkey) -> Result<()> {
        // Ensure the caller is the governor
        require!(ctx.accounts.authorized_account.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);

        // Add new executor to the list
        let governance_state = &mut ctx.accounts.state;
        governance_state.executors.push(new_executor);
        Ok(())
    }

        // Function to update executor
    pub fn remove_executor(ctx: Context<UpdateExecutor>, executor: Pubkey) -> Result<()> {
            require!(ctx.accounts.authorized_account.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);
            let address_list = &mut ctx.accounts.state.executors;
            address_list.retain(|&x| x != executor);
            // Logic to update executor
            Ok(())
        }

    // Update minimum execution fee
    pub fn update_min_execution_fee(ctx: Context<UpdateMinExecutionFee>, new_fee: u64) -> Result<()> {
        require!(ctx.accounts.authorized_account.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);     
        let state =&mut ctx.accounts.state;
        state.min_execution_fee = new_fee;
        Ok(())
    }


    // Create increase order
    pub fn create_increase_order(ctx: Context<CreateIncreaseOrder>,side:bool,
        margin_delta:u64,
        sizeDelta:u64,
        trigger_marketPriceX96 :u64,
        trigger_above : bool,
        acceptable_trade_price:u64) -> Result<()> {
        
        if margin_delta > 0  {
            // router plugin transfer
        }

        let new_order = IncreaseOrder {
            account: ctx.accounts.authorized_account.key(), // Replace with your actual account
            pool: ctx.accounts.state.pool, // Replace with your actual pool
            side: true, // Replace with your actual side
            marginDelta: 0, // Replace with your actual margin delta
            sizeDelta: 0, // Replace with your actual size delta
            triggerMarketPriceX96: 0, // Replace with your actual trigger market price X96
            triggerAbove: true, // Replace with your actual trigger above
            acceptableTradePriceX96: 0, // Replace with your actual acceptable trade price X96
            executionFee: 0, // Replace with your actual execution fee
        };

        ctx.accounts.state.all_increase_orders.push(new_order);
        Ok(())
    }

    pub fn create_decrease_order(ctx: Context<CreateDecreaseOrder>,side:bool,
        margin_delta:u64,
        sizeDelta:u64,
        trigger_marketPriceX96 :u64,
        trigger_above : bool,
        acceptable_trade_price:u64 , 
    receiver : Pubkey) -> Result<()> {
        
        if margin_delta > 0  {
            // router plugin transfer
        }

        let new_order = DecreaseOrder {
            account: ctx.accounts.authorized_account.key(), // Replace with your actual account
            pool: ctx.accounts.state.pool, // Replace with your actual pool
            side: true, // Replace with your actual side
            marginDelta: 0, // Replace with your actual margin delta
            sizeDelta: 0, // Replace with your actual size delta
            triggerMarketPriceX96: 0, // Replace with your actual trigger market price X96
            triggerAbove: trigger_above, // Replace with your actual trigger above
            acceptableTradePriceX96: 0, // Replace with your actual acceptable trade price X96
            executionFee: 0, // Replace with your actual execution fee
            receiver : receiver
        };

        ctx.accounts.state.all_decrease_orders.push(new_order);
        Ok(())
    }

    pub fn update_decrease_order(ctx: Context<CreateDecreaseOrder>, order_index: u64, trigger_market_price: u64, acceptable_trade_price: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let index_usize = order_index as usize;
        let order = &mut state.all_decrease_orders[index_usize];
        require!(order.account == ctx.accounts.authorized_account.key(), MyError::CallerUnauthorized);
        order.triggerMarketPriceX96 = trigger_market_price;
        order.acceptableTradePriceX96 = acceptable_trade_price;
    
        Ok(())
    }
    

    pub fn update_increase_order(ctx: Context<CreateIncreaseOrder> , order_index :u64 , trigger_market_price : u64 , acceptable_trade_price: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let index_usize = order_index as usize;
        let order = &mut state.all_increase_orders[index_usize];
        require!(order.account == ctx.accounts.authorized_account.key(), MyError::CallerUnauthorized);
        order.triggerMarketPriceX96 = trigger_market_price;
        order.acceptableTradePriceX96 = acceptable_trade_price;
        Ok(())
    }

    pub fn cancel_increase_order(ctx: Context<CreateIncreaseOrder> , order_index :u64 , fee_reciever: Pubkey ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let index_usize = order_index as usize;
        let order = &mut state.all_increase_orders[index_usize];
        require!(order.account == ctx.accounts.authorized_account.key(), MyError::CallerUnauthorized);
        // token tranfer 
        // sol transfer
        state.all_increase_orders.remove(index_usize);

        Ok(())
    }

    pub fn execute_increase_order(ctx: Context<CreateIncreaseOrder> , order_index :u64 , fee_reciever: Pubkey ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let index_usize = order_index as usize;
        let order = &mut state.all_increase_orders[index_usize];
        // external call to pool required here 
        let market_price = 100;
        _validate_trade_price_X96(order.side , market_price , order.triggerMarketPriceX96);

        // external call to router 
        // sol transfer out 
        state.all_increase_orders.remove(index_usize);

        Ok(())
    }

    pub fn cancel_decrease_order(ctx: Context<CreateIncreaseOrder> , order_index :u64 , fee_reciever: Pubkey ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let index_usize = order_index as usize;
        let order = &mut state.all_decrease_orders[index_usize];
        require!(order.account == ctx.accounts.authorized_account.key(), MyError::CallerUnauthorized);
        //  sol tranfer out 
        state.all_decrease_orders.remove(index_usize);

        
        Ok(())
    }

    pub fn execute_decrease_order(ctx: Context<CreateIncreaseOrder> , order_index :u64 , fee_reciever: Pubkey ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let index_usize = order_index as usize;
        let order = &mut state.all_decrease_orders[index_usize];
        // external call to pool required here 
        let market_price = 100;
        _validate_trade_price_X96(order.side , market_price , order.triggerMarketPriceX96);
        let size_delta_after = order.sizeDelta;
        let margin_delta_after = order.marginDelta;
        if order.sizeDelta == 0{
            // external call to pool 
            let margin_delta_after = 0;
        }

        // external call to router 
        let trade_price = 0;
        if order.sizeDelta != 0 {
            _validate_trade_price_X96(order.side , trade_price , order.acceptableTradePriceX96);
        }
        // sol transfer out 
        state.all_decrease_orders.remove(index_usize);

        Ok(())
    }

    pub fn create_take_profit_and_stop_loss_orders(ctx: Context<CreateIncreaseOrder>,
        pool: Pubkey,
        side: bool,
        margin_deltas: [u64; 2],
        size_deltas: [u64; 2],
        trigger_market_price: [u64; 2],
        acceptable_trade_price: [u64; 2],
        receiver: Pubkey,
    ) -> Result<()>  {
        // Function body goes here
        // add create decrease order
        Ok(())
    }





    // ... Add other functions here
}

pub fn  _validate_trade_price_X96(_side : bool, _tradePriceX96 : u64,  _acceptableTradePriceX96 : u64) -> Result<()> {
    if _side && (_tradePriceX96 > _acceptableTradePriceX96) || (!_side && (_tradePriceX96 > _acceptableTradePriceX96) )  {
        require!(false , MyError::CallerUnauthorized);
    }

    return Ok(())
}

#[account]
pub struct ContractState {
    pub executors: Vec<Pubkey>,
    pub all_increase_orders: Vec<IncreaseOrder>,
    pub all_decrease_orders: Vec<DecreaseOrder>,
    router : Pubkey,
    pool : Pubkey,
    usd : Pubkey,
    price_feed : Pubkey,
    initilized: bool,
    min_execution_fee: u64 ,
    execution_gas_limit : u64 , 
    increase_order_index : u64 , 
    decrease_order_index : u64
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct IncreaseOrder{
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
pub struct DecreaseOrder{
    account : Pubkey,
    pool : Pubkey,
    side : bool,
    marginDelta : u64,
    sizeDelta : u64 ,
    triggerMarketPriceX96 : u64,
    triggerAbove :bool,
    acceptableTradePriceX96 : u64,
    executionFee:u64,
    receiver:Pubkey,
}


// Initialization context
#[derive(Accounts)]
pub struct Initialize<'info> {
     // Adjust space as needed
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, ContractState>,

}

// Update minimum execution fee context
#[derive(Accounts)]
pub struct UpdateMinExecutionFee<'info> {
     // Adjust space as needed
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, ContractState>,
}

// Update order executor context
#[derive(Accounts)]
pub struct UpdateOrderExecutor<'info> {
     // Adjust space as needed
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, ContractState>,
}

#[derive(Accounts)]
pub struct UpdateExecutor<'info> {
     // Adjust space as needed
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, ContractState>,
}

// Create increase order context
#[derive(Accounts)]
pub struct CreateIncreaseOrder<'info> {
     // Adjust space as needed
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, ContractState>,
}

// Create increase order context
#[derive(Accounts)]
pub struct CreateDecreaseOrder<'info> {
     // Adjust space as needed
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, ContractState>,
}

// OrderBook state
#[account]
pub struct OrderBookState {
    pub min_execution_fee: u64,
    // ... Add other state fields as needed
    pub execution_gas_limit: u64,
    pub order_executors: Vec<Pubkey>, // Use a Vec for dynamic size, or a fixed-size array if preferred
    // ... Add mappings for orders
}

// ... Define other structs and enums as needed

// Custom errors
#[error_code]
pub enum MyError {
    #[msg("Unauthorized access")]
    CallerUnauthorized,
    #[msg("Invalid operation")]
    InvalidOperation,
    #[msg("Program Already initilized")]
    AlreadyInitlized,
}
