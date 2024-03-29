use crate::evm::precompiles;
use frame_support::dispatch::{Pays, PostDispatchInfo};
use frame_support::ensure;
use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use frame_support::traits::Time;
use pallet_evm::{AddressMapping, GasWeightMapping, Runner};
use pallet_genesis_history::migration::Weight;
use pallet_transaction_multi_payment::EVMPermit;
use primitive_types::{H160, H256, U256};
use primitives::AccountId;
use sp_io::hashing::keccak_256;
use sp_runtime::traits::UniqueSaturatedInto;
use sp_runtime::{DispatchErrorWithPostInfo, DispatchResult};
use sp_std::vec::Vec;

pub struct EvmPermitHandler<R>(sp_std::marker::PhantomData<R>);

impl<R> EVMPermit for EvmPermitHandler<R>
where
	R: frame_system::Config + pallet_evm::Config + pallet_transaction_multi_payment::Config,
	R::Nonce: Into<U256>,
	AccountId: From<R::AccountId>,
{
	fn validate_permit(
		source: H160,
		target: H160,
		input: Vec<u8>,
		value: U256,
		gas_limit: u64,
		deadline: U256,
		v: u8,
		r: H256,
		s: H256,
	) -> DispatchResult {
		let account_id = <R as pallet_evm::Config>::AddressMapping::into_account_id(source);
		let account_nonce = frame_system::Pallet::<R>::account_nonce(&account_id);

		let permit = pallet_evm_precompile_call_permit::CallPermitPrecompile::<R>::generate_permit(
			precompiles::DISPATCH_ADDR,
			source,
			target,
			value,
			input,
			gas_limit,
			account_nonce.into(),
			deadline,
		);

		// Blockchain time is in ms while Ethereum use second timestamps.
		let timestamp: u128 = <R as pallet_evm::Config>::Timestamp::now().unique_saturated_into();
		let timestamp: U256 = U256::from(timestamp / 1000);

		ensure!(
			deadline >= timestamp,
			pallet_transaction_multi_payment::Error::<R>::EvmPermitExpired
		);

		let mut sig = [0u8; 65];
		sig[0..32].copy_from_slice(&r.as_bytes());
		sig[32..64].copy_from_slice(&s.as_bytes());
		sig[64] = v;
		let signer = sp_io::crypto::secp256k1_ecdsa_recover(&sig, &permit)
			.map_err(|_| pallet_transaction_multi_payment::Error::<R>::EvmPermitInvalid)?;
		let signer = H160::from(H256::from_slice(keccak_256(&signer).as_slice()));
		ensure!(
			signer != H160::zero() && signer == source,
			pallet_transaction_multi_payment::Error::<R>::EvmPermitInvalid
		);

		Ok(())
	}

	fn dispatch_permit(
		source: H160,
		target: H160,
		input: Vec<u8>,
		value: U256,
		gas_limit: u64,
		max_fee_per_gas: Option<U256>,
		max_priority_fee_per_gas: Option<U256>,
		nonce: Option<U256>,
		access_list: Vec<(H160, Vec<H256>)>,
	) -> DispatchResultWithPostInfo {
		let is_transactional = true;
		let validate = true;
		let info = match <R as pallet_evm::Config>::Runner::call(
			source,
			target,
			input,
			value,
			gas_limit,
			max_fee_per_gas,
			max_priority_fee_per_gas,
			nonce,
			access_list,
			is_transactional,
			validate,
			None,
			None,
			<R as pallet_evm::Config>::config(),
		) {
			Ok(info) => info,
			Err(e) => {
				return Err(DispatchErrorWithPostInfo {
					post_info: PostDispatchInfo {
						actual_weight: Some(e.weight),
						pays_fee: Pays::Yes,
					},
					error: e.error.into(),
				})
			}
		};
		Ok(PostDispatchInfo {
			actual_weight: {
				let mut gas_to_weight = <R as pallet_evm::Config>::GasWeightMapping::gas_to_weight(
					info.used_gas.standard.unique_saturated_into(),
					true,
				);
				if let Some(weight_info) = info.weight_info {
					if let Some(proof_size_usage) = weight_info.proof_size_usage {
						*gas_to_weight.proof_size_mut() = proof_size_usage;
					}
				}
				Some(gas_to_weight)
			},
			pays_fee: Pays::No,
		})
	}

	fn dispatch_weight(gas_limit: u64) -> Weight {
		let without_base_extrinsic_weight = true;
		<R as pallet_evm::Config>::GasWeightMapping::gas_to_weight(gas_limit, without_base_extrinsic_weight)
	}
}
