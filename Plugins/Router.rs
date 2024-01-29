pub struct Router {
    pub efc: IEFC,
    pub reward_farm: IRewardFarm,
    pub fee_distributor: IFeeDistributor,
}

impl Router {
    pub fn new(efc: IEFC, reward_farm: IRewardFarm, fee_distributor: IFeeDistributor) -> Self {
        Self { efc, reward_farm, fee_distributor }
    }

    pub fn plugin_transfer(&self, env: &Env, token: IERC20, from: Address, to: Address, amount: Uint256) -> Result<(), Error> {
        self._only_plugin_approved(env, from)?;
        token.safe_transfer_from(env, from, to, amount)
    }

    // Other methods...
}
