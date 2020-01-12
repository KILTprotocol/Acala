//! Mocks for the honzon module.

#![cfg(test)]

use frame_support::{impl_outer_origin, parameter_types};
use primitives::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup, DispatchResult, Perbill};
use support::{AuctionManager, AuctionManagerExtended, ExchangeRate, Price, PriceProvider, Rate, Ratio};

use super::*;

impl_outer_origin! {
	pub enum Origin for Runtime {}
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
	pub const ExistentialDeposit: u64 = 0;
	pub const TransferFee: u64 = 0;
	pub const CreationFee: u64 = 2;
	pub const CollateralCurrencyIds: Vec<CurrencyId> = vec![BTC, DOT];
	pub const GlobalStabilityFee: Rate = Rate::from_parts(0);
	pub const DefaultLiquidationRatio: Ratio = Ratio::from_rational(3, 2);
	pub const DefaulDebitExchangeRate: ExchangeRate = ExchangeRate::from_natural(1);
	pub const MinimumDebitValue: Balance = 2;
	pub const GetNativeCurrencyId: CurrencyId = ACA;
	pub const GetStableCurrencyId: CurrencyId = AUSD;
}

pub type AccountId = u64;
pub type BlockNumber = u64;
pub type Balance = u64;
pub type Amount = i64;
pub type DebitBalance = u64;
pub type DebitAmount = i64;
pub type CurrencyId = u32;
pub type AuctionId = u64;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;

pub const ACA: CurrencyId = 0;
pub const AUSD: CurrencyId = 1;
pub const BTC: CurrencyId = 2;
pub const DOT: CurrencyId = 3;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Runtime;

impl system::Trait for Runtime {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Call = ();
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
}

impl orml_tokens::Trait for Runtime {
	type Event = ();
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
}
pub type Tokens = orml_tokens::Module<Runtime>;

impl pallet_balances::Trait for Runtime {
	type Balance = Balance;
	type OnFreeBalanceZero = ();
	type OnNewAccount = ();
	type OnReapAccount = ();
	type TransferPayment = ();
	type DustRemoval = ();
	type Event = ();
	type ExistentialDeposit = ExistentialDeposit;
	type TransferFee = TransferFee;
	type CreationFee = CreationFee;
}
pub type PalletBalances = pallet_balances::Module<Runtime>;

pub type AdaptedBasicCurrency = orml_currencies::BasicCurrencyAdapter<Runtime, PalletBalances, Balance>;

impl orml_currencies::Trait for Runtime {
	type Event = ();
	type MultiCurrency = Tokens;
	type NativeCurrency = AdaptedBasicCurrency;
	type GetNativeCurrencyId = GetNativeCurrencyId;
}
pub type Currencies = orml_currencies::Module<Runtime>;

impl vaults::Trait for Runtime {
	type Event = ();
	type Convert = cdp_engine::DebitExchangeRateConvertor<Runtime>;
	type Currency = Tokens;
	type RiskManager = CdpEngineModule;
	type DebitBalance = DebitBalance;
	type DebitAmount = DebitAmount;
	type Treasury = CdpTreasury;
}

pub struct MockPriceSource;
impl PriceProvider<CurrencyId, Price> for MockPriceSource {
	#[allow(unused_variables)]
	fn get_price(base: CurrencyId, quote: CurrencyId) -> Option<Price> {
		Some(Price::from_natural(1))
	}

	#[allow(unused_variables)]
	fn lock_price(currency_id: CurrencyId) {}

	#[allow(unused_variables)]
	fn unlock_price(currency_id: CurrencyId) {}
}

pub struct MockAuctionManager;
impl AuctionManager<AccountId> for MockAuctionManager {
	type CurrencyId = CurrencyId;
	type Balance = Balance;

	#[allow(unused_variables)]
	fn new_collateral_auction(
		who: &AccountId,
		currency_id: Self::CurrencyId,
		amount: Self::Balance,
		target: Self::Balance,
		bad_debt: Self::Balance,
	) {
	}

	#[allow(unused_variables)]
	fn new_debit_auction(amount: Self::Balance, fix: Self::Balance) {}

	#[allow(unused_variables)]
	fn new_surplus_auction(amount: Self::Balance) {}

	fn get_total_debit_in_auction() -> Self::Balance {
		Default::default()
	}

	fn get_total_target_in_auction() -> Self::Balance {
		Default::default()
	}
}

impl AuctionManagerExtended<AccountId> for MockAuctionManager {
	type AuctionId = AuctionId;

	#[allow(unused_variables)]
	fn get_total_collateral_in_auction(id: Self::CurrencyId) -> Self::Balance {
		Default::default()
	}

	fn get_total_surplus_in_auction() -> Self::Balance {
		Default::default()
	}

	#[allow(unused_variables)]
	fn cancel_auction(id: Self::AuctionId) -> DispatchResult {
		Ok(())
	}
}

impl cdp_treasury::Trait for Runtime {
	type Currency = Currencies;
	type GetStableCurrencyId = GetStableCurrencyId;
	type AuctionManagerHandler = MockAuctionManager;
}
pub type CdpTreasury = cdp_treasury::Module<Runtime>;

impl cdp_engine::Trait for Runtime {
	type Event = ();
	type AuctionManagerHandler = MockAuctionManager;
	type PriceSource = MockPriceSource;
	type CollateralCurrencyIds = CollateralCurrencyIds;
	type GlobalStabilityFee = GlobalStabilityFee;
	type DefaultLiquidationRatio = DefaultLiquidationRatio;
	type DefaulDebitExchangeRate = DefaulDebitExchangeRate;
	type MinimumDebitValue = MinimumDebitValue;
	type GetStableCurrencyId = GetStableCurrencyId;
	type Treasury = CdpTreasury;
}
pub type CdpEngineModule = cdp_engine::Module<Runtime>;

impl honzon::Trait for Runtime {
	type Event = ();
}
pub type HonzonModule = honzon::Module<Runtime>;

impl Trait for Runtime {
	type Event = ();
	type PriceSource = MockPriceSource;
	type Treasury = CdpTreasury;
	type AuctionManagerHandler = MockAuctionManager;
	type OnShutdown = (CdpTreasury, CdpEngineModule, HonzonModule);
}
pub type EmergencyShutdownModule = Module<Runtime>;

pub struct ExtBuilder {
	endowed_accounts: Vec<(AccountId, CurrencyId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			endowed_accounts: vec![
				(ALICE, BTC, 1000),
				(BOB, BTC, 1000),
				(ALICE, DOT, 1000),
				(BOB, DOT, 1000),
			],
		}
	}
}

impl ExtBuilder {
	pub fn build(self) -> runtime_io::TestExternalities {
		let mut t = system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

		orml_tokens::GenesisConfig::<Runtime> {
			endowed_accounts: self.endowed_accounts,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}
