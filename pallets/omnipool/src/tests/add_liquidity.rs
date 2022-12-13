use super::*;
use frame_support::assert_noop;
use test_case::test_case;

#[test]
fn add_liquidity_should_work_when_asset_exists_in_pool() {
	ExtBuilder::default()
		.add_endowed_accounts((LP1, 1_000, 5000 * ONE))
		.add_endowed_accounts((LP2, 1_000, 5000 * ONE))
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.with_token(1_000, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.build()
		.execute_with(|| {
			let token_amount = 2000 * ONE;
			let liq_added = 400 * ONE;

			// ACT

			assert_ok!(Omnipool::add_liquidity(Origin::signed(LP1), 1_000, liq_added));

			// ASSERT - asset state, pool state, position
			assert_asset_state!(
				1_000,
				AssetReserveState {
					reserve: token_amount + liq_added,
					hub_reserve: 1560 * ONE,
					shares: 2400 * ONE,
					protocol_shares: Balance::zero(),
					cap: DEFAULT_WEIGHT_CAP,
					tradable: Tradability::default(),
				}
			);

			let position = Positions::<Test>::get(1).unwrap();

			let expected = Position::<Balance, AssetId> {
				asset_id: 1_000,
				amount: liq_added,
				shares: liq_added,
				price: (1560 * ONE, token_amount + liq_added),
			};

			assert_eq!(position, expected);

			assert_pool_state!(12_060 * ONE, 24_120 * ONE, SimpleImbalance::default());

			assert_balance!(LP1, 1_000, 4600 * ONE);

			let minted_position = POSITIONS.with(|v| v.borrow().get(&1).copied());

			assert_eq!(minted_position, Some(LP1));
		});
}

#[test]
fn add_stable_asset_liquidity_works() {
	ExtBuilder::default()
		.add_endowed_accounts((LP1, DAI, 5000 * ONE))
		.add_endowed_accounts((LP2, 1_000, 5000 * ONE))
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			let liq_added = 400 * ONE;
			let position_id = <NextPositionId<Test>>::get();
			assert_ok!(Omnipool::add_liquidity(Origin::signed(LP1), DAI, liq_added));

			assert_asset_state!(
				DAI,
				AssetReserveState {
					reserve: 1000 * ONE + liq_added,
					hub_reserve: 700000000000000,
					shares: 1400000000000000,
					protocol_shares: 1000 * ONE,
					cap: DEFAULT_WEIGHT_CAP,
					tradable: Tradability::default(),
				}
			);

			let position = Positions::<Test>::get(position_id).unwrap();

			let expected = Position::<Balance, AssetId> {
				asset_id: DAI,
				amount: liq_added,
				shares: liq_added,
				price: (700 * ONE, 1400 * ONE),
			};

			assert_eq!(position, expected);

			assert_pool_state!(10_700 * ONE, 21_400 * ONE, SimpleImbalance::default());

			assert_balance!(LP1, DAI, 4600 * ONE);

			let minted_position = POSITIONS.with(|v| v.borrow().get(&position_id).copied());

			assert_eq!(minted_position, Some(LP1));
		});
}

#[test]
fn add_liquidity_for_non_pool_token_fails() {
	ExtBuilder::default()
		.add_endowed_accounts((LP1, 1_000, 5000 * ONE))
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			assert_noop!(
				Omnipool::add_liquidity(Origin::signed(LP1), 1_000, 2000 * ONE,),
				Error::<Test>::AssetNotFound
			);
		});
}

#[test]
fn add_liquidity_with_insufficient_balance_fails() {
	ExtBuilder::default()
		.add_endowed_accounts((LP1, 1_000, 5000 * ONE))
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.with_token(1_000, FixedU128::from_float(0.65), LP1, 2000 * ONE)
		.build()
		.execute_with(|| {
			assert_noop!(
				Omnipool::add_liquidity(Origin::signed(LP3), 1_000, 2000 * ONE,),
				Error::<Test>::InsufficientBalance
			);
		});
}

#[test]
fn add_liquidity_exceeding_weight_cap_fails() {
	ExtBuilder::default()
		.add_endowed_accounts((LP1, 1_000, 5000 * ONE))
		.with_asset_weight_cap(Permill::from_float(0.1))
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.with_token(1_000, FixedU128::from_float(0.65), LP1, 100 * ONE)
		.build()
		.execute_with(|| {
			assert_noop!(
				Omnipool::add_liquidity(Origin::signed(LP1), 1_000, 2000 * ONE,),
				Error::<Test>::AssetWeightCapExceeded
			);
		});
}

#[test]
fn add_insufficient_liquidity_fails() {
	ExtBuilder::default()
		.add_endowed_accounts((LP1, 1_000, 5000 * ONE))
		.with_min_added_liquidity(5 * ONE)
		.with_asset_weight_cap(Permill::from_float(0.1))
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.with_token(1_000, FixedU128::from_float(0.65), LP1, 2000 * ONE)
		.build()
		.execute_with(|| {
			assert_noop!(
				Omnipool::add_liquidity(Origin::signed(LP3), 1_000, ONE,),
				Error::<Test>::InsufficientLiquidity
			);
		});
}

#[test]
fn add_liquidity_should_fail_when_asset_state_does_not_include_add_liquidity() {
	ExtBuilder::default()
		.add_endowed_accounts((LP1, 1_000, 5000 * ONE))
		.with_min_added_liquidity(ONE)
		.with_asset_weight_cap(Permill::from_float(0.1))
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.with_token(1_000, FixedU128::from_float(0.65), LP1, 2000 * ONE)
		.build()
		.execute_with(|| {
			assert_ok!(Omnipool::set_asset_tradable_state(
				Origin::root(),
				1000,
				Tradability::SELL | Tradability::BUY | Tradability::REMOVE_LIQUIDITY
			));

			assert_noop!(
				Omnipool::add_liquidity(Origin::signed(LP1), 1_000, 2 * ONE),
				Error::<Test>::NotAllowed
			);
		});
}

#[test_case(0)]
#[test_case(ONE)]
#[test_case(100 * ONE)]
fn add_liquidity_should_work_when_trade_volume_limit_not_exceeded(diff_from_max_limit: Balance) {
	// Arrange
	let initial_liquidity = 1_000_000 * ONE;
	ExtBuilder::default()
		.add_endowed_accounts((LP1, 1_000, 2_000_000 * ONE))
		.add_endowed_accounts((LP2, 1_000, 2_000_000 * ONE))
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.with_token(1_000, FixedU128::from_float(0.65), LP2, initial_liquidity)
		.with_max_liquidity_limit_per_block(Some(TEN_PERCENT))
		.build()
		.execute_with(|| {
			let liq_added =
				CircuitBreaker::calculate_limit(initial_liquidity, TEN_PERCENT).unwrap() - diff_from_max_limit;

			// Act & Assert
			assert_ok!(Omnipool::add_liquidity(Origin::signed(LP1), 1_000, liq_added));
		});
}

#[test]
fn add_liquidity_should_fail_when_trade_volume_limit_exceeded() {
	// Arrange
	let initial_liquidity = 1_000_000 * ONE;
	ExtBuilder::default()
		.add_endowed_accounts((LP1, 1_000, 2_000_000 * ONE))
		.add_endowed_accounts((LP2, 1_000, 2_000_000 * ONE))
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.with_token(1_000, FixedU128::from_float(0.65), LP2, initial_liquidity)
		.with_max_liquidity_limit_per_block(Some(TEN_PERCENT))
		.build()
		.execute_with(|| {
			let liq_added = CircuitBreaker::calculate_limit(initial_liquidity, TEN_PERCENT).unwrap() + ONE;

			// Act & Assert
			assert_noop!(
				Omnipool::add_liquidity(Origin::signed(LP1), 1_000, liq_added),
				pallet_circuit_breaker::Error::<Test>::MaxLiquidityLimitPerBlockReached
			);
		});
}

#[test]
fn add_liquidity_should_fail_when_consequent_calls_exceed_trade_volume_limit() {
	// Arrange
	let initial_liquidity = 1_000_000 * ONE;
	ExtBuilder::default()
		.add_endowed_accounts((LP1, 1_000, 2_000_000 * ONE))
		.add_endowed_accounts((LP2, 1_000, 2_000_000 * ONE))
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.with_token(1_000, FixedU128::from_float(0.65), LP2, initial_liquidity)
		.with_max_liquidity_limit_per_block(Some(TEN_PERCENT))
		.build()
		.execute_with(|| {
			let liq_added = CircuitBreaker::calculate_limit(initial_liquidity, FIVE_PERCENT).unwrap() + ONE;

			// Act & Assert
			assert_ok!(Omnipool::add_liquidity(Origin::signed(LP1), 1_000, liq_added));
			assert_noop!(
				Omnipool::add_liquidity(Origin::signed(LP1), 1_000, liq_added),
				pallet_circuit_breaker::Error::<Test>::MaxLiquidityLimitPerBlockReached
			);
		});
}
