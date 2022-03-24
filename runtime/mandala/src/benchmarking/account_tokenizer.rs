// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::{
	dollar, AccountId, AccountTokenizer, Balances, CurrencyId, ForeignStateOracle, GetNativeCurrencyId, Origin,
	OriginCaller, PolkadotXcm, Runtime, NFT,
};
use hex_literal::hex;
use runtime_common::MAXIMUM_BLOCK_WEIGHT;

use frame_benchmarking::vec;
use frame_support::{
	assert_ok,
	traits::{Currency, Hooks},
};
use frame_system::RawOrigin;
use sp_runtime::traits::AccountIdConversion;
use xcm::latest::MultiLocation;

const NATIVE: CurrencyId = GetNativeCurrencyId::get();

fn setup_account_tokenizer_benchmark() -> (AccountId, AccountId) {
	let caller = AccountId::new(hex!["65766d3abf0b5a4099f0bf6c8bc4252ebec548bae95602ea0000000000000000"]);
	let proxy = AccountId::new(hex!["b99bbff5de2888225d1b0fcdba9c4e79117f910ae30b042618fecf87bd860316"]);
	let treasury = <Runtime as module_account_tokenizer::Config>::TreasuryAccount::get();

	Balances::make_free_balance_be(&treasury, 1_000 * dollar(NATIVE));
	Balances::make_free_balance_be(&caller, 1_000 * dollar(NATIVE));
	Balances::make_free_balance_be(
		&<Runtime as module_account_tokenizer::Config>::PalletId::get().into_account(),
		1_000 * dollar(NATIVE),
	);
	AccountTokenizer::on_runtime_upgrade();
	(caller, proxy)
}

runtime_benchmarks! {
	{ Runtime, module_account_tokenizer }
	initialize_nft_class {
		Balances::make_free_balance_be(&<Runtime as module_account_tokenizer::Config>::TreasuryAccount::get(), 1_000 * dollar(NATIVE));
	}: {
		AccountTokenizer::on_runtime_upgrade();
	}

	request_mint {
		let (caller, proxy) = setup_account_tokenizer_benchmark();
	}: _(RawOrigin::Signed(caller.clone()), proxy, caller.clone(), 1, 0, 0)

	confirm_mint_request {
		let (caller, proxy) = setup_account_tokenizer_benchmark();
		assert_ok!(AccountTokenizer::request_mint(
			Origin::signed(caller.clone()),
			proxy.clone(),
			caller.clone(),
			1,
			0,
			0
		));
	}: {
		assert_ok!(ForeignStateOracle::respond_query_request(OriginCaller::ForeignStateOracleCommittee(pallet_collective::RawOrigin::Members(1, 1)).into(), 0, vec![1], MAXIMUM_BLOCK_WEIGHT));
	}

	request_redeem {
		let (caller, proxy) = setup_account_tokenizer_benchmark();
		let _ = AccountTokenizer::request_mint(
			Origin::signed(caller.clone()),
			proxy.clone(),
			caller.clone(),
			1,
			0,
			0
		);
		assert_ok!(ForeignStateOracle::respond_query_request(OriginCaller::ForeignStateOracleCommittee(pallet_collective::RawOrigin::Members(1, 1)).into(), 0, vec![1], MAXIMUM_BLOCK_WEIGHT));
		// Sets supported version on PolkadotXcm. This prevents XCM sending to fail.
		assert_ok!(PolkadotXcm::force_xcm_version(
			Origin::root(),
			frame_benchmarking::Box::new(MultiLocation::parent()),
			2,
		));
	}: _(RawOrigin::Signed(caller.clone()), proxy, caller.clone())

	confirm_redeem_account_token {
		let (caller, proxy) = setup_account_tokenizer_benchmark();

		assert_ok!(AccountTokenizer::request_mint(
			Origin::signed(caller.clone()),
			proxy.clone(),
			caller.clone(),
			1,
			0,
			0
		));
		assert_ok!(ForeignStateOracle::respond_query_request(OriginCaller::ForeignStateOracleCommittee(pallet_collective::RawOrigin::Members(1, 1)).into(), 0, vec![1], MAXIMUM_BLOCK_WEIGHT));
		// Sets supported version on PolkadotXcm. This prevents XCM sending to fail.
		assert_ok!(PolkadotXcm::force_xcm_version(
			Origin::root(),
			frame_benchmarking::Box::new(MultiLocation::parent()),
			2,
		));
		assert_ok!(AccountTokenizer::request_redeem(RawOrigin::Signed(caller.clone()).into(), proxy, caller.clone()));
	}: {
		assert_ok!(ForeignStateOracle::respond_query_request(OriginCaller::ForeignStateOracleCommittee(pallet_collective::RawOrigin::Members(1, 1)).into(), 1, vec![], MAXIMUM_BLOCK_WEIGHT));
	}

	transfer_nft {
		let (caller, proxy) = setup_account_tokenizer_benchmark();
		assert_ok!(AccountTokenizer::request_mint(
			Origin::signed(caller.clone()),
			proxy.clone(),
			caller.clone(),
			1,
			0,
			0
		));
		assert_ok!(ForeignStateOracle::respond_query_request(OriginCaller::ForeignStateOracleCommittee(pallet_collective::RawOrigin::Members(1, 1)).into(), 0, vec![1], MAXIMUM_BLOCK_WEIGHT));
		// Sets supported version on PolkadotXcm. This prevents XCM sending to fail.
		assert_ok!(PolkadotXcm::force_xcm_version(
			Origin::root(),
			frame_benchmarking::Box::new(MultiLocation::parent()),
			2,
		));
		assert_ok!(AccountTokenizer::request_redeem(RawOrigin::Signed(caller.clone()).into(), proxy.clone(), caller.clone()));
	}: _(Origin::root(), proxy, caller.clone())

	burn_nft {
		let (caller, proxy) = setup_account_tokenizer_benchmark();
		assert_ok!(AccountTokenizer::request_mint(
			Origin::signed(caller.clone()),
			proxy.clone(),
			caller.clone(),
			1,
			0,
			0
		));
		assert_ok!(ForeignStateOracle::respond_query_request(OriginCaller::ForeignStateOracleCommittee(pallet_collective::RawOrigin::Members(1, 1)).into(), 0, vec![1], MAXIMUM_BLOCK_WEIGHT));
		// Sets supported version on PolkadotXcm. This prevents XCM sending to fail.
		assert_ok!(PolkadotXcm::force_xcm_version(
			Origin::root(),
			frame_benchmarking::Box::new(MultiLocation::parent()),
			2,
		));
		assert_ok!(AccountTokenizer::request_redeem(RawOrigin::Signed(caller.clone()).into(), proxy.clone(), caller.clone()));
	}: _(Origin::root(), proxy)

	transfer_treasury_funds {
		let (caller, proxy) = setup_account_tokenizer_benchmark();
	}: _(Origin::root(), caller, dollar(NATIVE))

	force_unreserve_funds {
		let (caller, proxy) = setup_account_tokenizer_benchmark();
		assert_ok!(AccountTokenizer::request_mint(
			Origin::signed(caller.clone()),
			proxy.clone(),
			caller.clone(),
			1,
			0,
			0
		));
	}: _(Origin::root(), caller, dollar(NATIVE), true)

	remint_burned_nft {
		let (caller, proxy) = setup_account_tokenizer_benchmark();
		assert_ok!(AccountTokenizer::request_mint(
			Origin::signed(caller.clone()),
			proxy.clone(),
			caller.clone(),
			1,
			0,
			0
		));
		assert_ok!(ForeignStateOracle::respond_query_request(OriginCaller::ForeignStateOracleCommittee(pallet_collective::RawOrigin::Members(1, 1)).into(), 0, vec![1], MAXIMUM_BLOCK_WEIGHT));
		let token_id = AccountTokenizer::minted_account(&proxy).unwrap();
		let class_id = AccountTokenizer::nft_class_id();
		NFT::burn(Origin::signed(caller.clone()), (class_id, token_id))?;
	}: _(Origin::root(), proxy, caller)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::benchmarking::utils::tests::new_test_ext;
	use orml_benchmarking::impl_benchmark_test_suite;

	impl_benchmark_test_suite!(new_test_ext(),);
}
