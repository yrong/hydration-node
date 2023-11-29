use crate::tests::*;
use pretty_assertions::assert_eq;

#[test]
fn complete_referral_flow_should_work_as_expected() {
	let mut volumes = HashMap::new();
	volumes.insert(Level::Novice, Some(100_000_000));
	volumes.insert(Level::Advanced, Some(200_000_000));
	volumes.insert(Level::Expert, None);

	let bob_initial_hdx = 10_000_000_000_000;

	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(BOB, DAI, 2_000_000_000_000_000_000),
			(BOB, HDX, bob_initial_hdx),
			(CHARLIE, DOT, 2_000_000_000_000),
		])
		.with_conversion_price(
			(HDX, DAI),
			FixedU128::from_rational(1_000_000_000_000, 1_000_000_000_000_000_000),
		)
		.with_conversion_price((HDX, DOT), FixedU128::from_rational(1_000_000_000_000, 500_000_000_000))
		.with_tiers(vec![
			(
				DAI,
				Level::Novice,
				Tier {
					referrer: Permill::from_float(0.005),
					trader: Permill::from_float(0.002),
				},
			),
			(
				DOT,
				Level::Novice,
				Tier {
					referrer: Permill::from_float(0.005),
					trader: Permill::from_float(0.002),
				},
			),
			(
				DAI,
				Level::Advanced,
				Tier {
					referrer: Permill::from_float(0.03),
					trader: Permill::from_float(0.01),
				},
			),
			(
				DOT,
				Level::Advanced,
				Tier {
					referrer: Permill::from_float(0.03),
					trader: Permill::from_float(0.01),
				},
			),
			(
				HDX,
				Level::Novice,
				Tier {
					referrer: Permill::from_float(0.002),
					trader: Permill::from_float(0.001),
				},
			),
			(
				HDX,
				Level::Advanced,
				Tier {
					referrer: Permill::from_float(0.03),
					trader: Permill::from_float(0.01),
				},
			),
		])
		.with_tier_volumes(volumes)
		.build()
		.execute_with(|| {
			// ARRANGE
			assert_ok!(Referrals::register_code(
				RuntimeOrigin::signed(ALICE),
				b"BALLS69".to_vec(),
				ALICE
			));
			assert_ok!(Referrals::link_code(RuntimeOrigin::signed(BOB), b"BALLS69".to_vec()));
			assert_ok!(Referrals::link_code(
				RuntimeOrigin::signed(CHARLIE),
				b"BALLS69".to_vec()
			));
			// TRADES
			assert_ok!(MockAmm::trade(RuntimeOrigin::signed(BOB), HDX, DAI, 1_000_000_000_000));
			assert_ok!(MockAmm::trade(
				RuntimeOrigin::signed(BOB),
				DAI,
				HDX,
				1_000_000_000_000_000_000
			));
			assert_ok!(MockAmm::trade(
				RuntimeOrigin::signed(CHARLIE),
				HDX,
				DOT,
				1_000_000_000_000
			));

			// Assert shares
			let alice_shares = Shares::<Test>::get(ALICE);
			assert_eq!(alice_shares, 120_000_000);
			let bob_shares = Shares::<Test>::get(BOB);
			assert_eq!(bob_shares, 30_000_000);
			let charlie_shares = Shares::<Test>::get(CHARLIE);
			assert_eq!(charlie_shares, 20_000_000);
			let total_shares = TotalShares::<Test>::get();
			assert_eq!(total_shares, alice_shares + bob_shares + charlie_shares);

			// CLAIMS
			assert_ok!(Referrals::claim_rewards(RuntimeOrigin::signed(CHARLIE),));
			// Assert charlie rewards
			let shares = Shares::<Test>::get(CHARLIE);
			assert_eq!(shares, 0);
			let total_shares = TotalShares::<Test>::get();
			assert_eq!(total_shares, alice_shares + bob_shares);
			let charlie_balance = Tokens::free_balance(HDX, &CHARLIE);
			assert_eq!(charlie_balance, 20000000);

			assert_ok!(Referrals::claim_rewards(RuntimeOrigin::signed(BOB),));
			// Assert BOB rewards
			let shares = Shares::<Test>::get(BOB);
			assert_eq!(shares, 0);
			let total_shares = TotalShares::<Test>::get();
			assert_eq!(total_shares, alice_shares);
			let bob_balance = Tokens::free_balance(HDX, &BOB);
			assert_eq!(bob_balance, 10_000_000_000_000);

			assert_ok!(Referrals::claim_rewards(RuntimeOrigin::signed(ALICE),));
			// Assert ALICE rewards
			let shares = Shares::<Test>::get(ALICE);
			assert_eq!(shares, 0);
			let total_shares = TotalShares::<Test>::get();
			assert_eq!(total_shares, 0);
			let alice_balance = Tokens::free_balance(HDX, &ALICE);
			assert_eq!(alice_balance, 778_000_120_000_000);
			let (level, total) = Referrer::<Test>::get(ALICE).unwrap();
			assert_eq!(level, Level::Advanced);
			assert_eq!(total, 120_000_000);
		});
}
