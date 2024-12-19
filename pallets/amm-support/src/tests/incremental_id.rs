// This file is part of HydraDX-node.

// Copyright (C) 2020-2022  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::tests::mock::*;
use crate::Event;

#[test]
fn stack_should_be_populated_when_pushed() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(AmmSupport::add_to_context(ExecutionType::Router));
		assert_eq!(AmmSupport::execution_context(), vec![ExecutionType::Router(0)]);
		assert_eq!(AmmSupport::execution_context().into_inner(), vec![ExecutionType::Router(0)]);

		assert_ok!(AmmSupport::add_to_context(ExecutionType::Router));
		assert_eq!(
			AmmSupport::execution_context(),
			vec![ExecutionType::Router(0), ExecutionType::Router(1)]
		);
		assert_eq!(
			AmmSupport::execution_context().into_inner(),
			vec![ExecutionType::Router(0), ExecutionType::Router(1)]
		);

		assert_ok!(AmmSupport::add_to_context(ExecutionType::ICE));
		assert_eq!(
			AmmSupport::execution_context(),
			vec![
				ExecutionType::Router(0),
				ExecutionType::Router(1),
				ExecutionType::ICE(2)
			]
		);
		assert_eq!(
			AmmSupport::execution_context().into_inner(),
			vec![
				ExecutionType::Router(0),
				ExecutionType::Router(1),
				ExecutionType::ICE(2)
			]
		);
	});
}

#[test]
fn stack_should_not_panic_when_full() {
	ExtBuilder::default().build().execute_with(|| {
		for _id in 0..MAX_STACK_SIZE {
			assert_ok!(AmmSupport::add_to_context(ExecutionType::Router));
		}

		assert_err!(
			AmmSupport::add_to_context(ExecutionType::Router),
			Error::<Test>::MaxStackSizeReached
		);
	});
}

#[test]
fn stack_should_be_reduced_when_poped() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(AmmSupport::add_to_context(ExecutionType::Router));
		assert_ok!(AmmSupport::add_to_context(ExecutionType::Router));
		assert_ok!(AmmSupport::add_to_context(ExecutionType::ICE));

		assert_ok!(AmmSupport::remove_from_context(), ExecutionType::ICE(2));
		assert_eq!(
			AmmSupport::execution_context(),
			vec![ExecutionType::Router(0), ExecutionType::Router(1)]
		);
		assert_eq!(
			AmmSupport::execution_context().into_inner(),
			vec![ExecutionType::Router(0), ExecutionType::Router(1)]
		);

		assert_ok!(AmmSupport::add_to_context(ExecutionType::ICE));
		assert_eq!(
			AmmSupport::execution_context(),
			vec![
				ExecutionType::Router(0),
				ExecutionType::Router(1),
				ExecutionType::ICE(3)
			]
		);
		assert_eq!(
			AmmSupport::execution_context().into_inner(),
			vec![
				ExecutionType::Router(0),
				ExecutionType::Router(1),
				ExecutionType::ICE(3)
			]
		);
	});
}

#[test]
fn pop_from_empty_stack_should_not_panic() {
	ExtBuilder::default().build().execute_with(|| {
		assert_err!(AmmSupport::remove_from_context(), Error::<Test>::EmptyStack);
	});
}

#[test]
fn event_should_be_deposited() {
	ExtBuilder::default().build().execute_with(|| {
		AmmSupport::deposit_trade_event(
			ALICE,
			BOB,
			Filler::Omnipool,
			TradeOperation::ExactIn,
			vec![Asset::new(HDX, 1_000_000)],
			vec![Asset::new(DOT, 2_000_000)],
			vec![Fee::new(HDX, 1_000, ALICE.into()), Fee::new(DOT, 2_000, BOB.into())],
		);

		expect_events(vec![Event::Swapped {
			swapper: ALICE,
			filler: BOB,
			filler_type: Filler::Omnipool,
			operation: TradeOperation::ExactIn,
			inputs: vec![Asset::new(HDX, 1_000_000)],
			outputs: vec![Asset::new(DOT, 2_000_000)],
			fees: vec![Fee::new(HDX, 1_000, ALICE), Fee::new(DOT, 2_000, BOB)],
			operation_id: vec![],
		}
		.into()]);
	});
}
