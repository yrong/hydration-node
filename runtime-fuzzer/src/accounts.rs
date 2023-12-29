use crate::AccountId;

const ACCOUNT_COUNT: u8 = 20;
const INITIAL_AMOUNT: u128 = 1_000_000_000;

pub fn get_native_endowed_accounts() -> Vec<(AccountId, u128)> {
	let endowed_accounts: Vec<(AccountId, u128)> = (0..ACCOUNT_COUNT)
		.map(|i| ([i; 32].into(), INITIAL_AMOUNT * 10u128.pow(12)))
		.collect();
	endowed_accounts
}
pub fn get_nonnative_endowed_accounts(assets: Vec<(u32, u8)>) -> Vec<(AccountId, Vec<(u32, u128)>)> {
	let mut amounts = Vec::new();
	for (asset_id, decimals) in assets {
		amounts.push((asset_id, INITIAL_AMOUNT * 10u128.pow(decimals as u32)));
	}
	(0..ACCOUNT_COUNT).map(|i| ([i; 32].into(), amounts.clone())).collect()
}

pub fn get_council_members() -> Vec<AccountId> {
	(0..5).map(|i| [i; 32].into()).collect()
}

pub fn get_technical_committee() -> Vec<AccountId> {
	(0..3).map(|i| [i; 32].into()).collect()
}

pub fn get_duster_reward_account() -> AccountId {
	[0; 32].into()
}

pub fn get_duster_dest_account() -> AccountId {
	[0; 32].into()
}

// Owner of initial positions
pub fn get_omnipool_position_owner() -> AccountId {
	[0; 32].into()
}

pub fn get_accounts_as_potential_origins() -> Vec<AccountId> {
	(0..ACCOUNT_COUNT).map(|i| [i; 32].into()).collect()
}
