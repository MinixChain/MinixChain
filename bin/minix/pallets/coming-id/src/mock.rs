use super::*;
use crate as pallet_coming_id;
use sp_core::H256;
use frame_support::{
	parameter_types,
	traits::{
		GenesisBuild, OnInitialize, OnFinalize
	}
};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header,
};
use frame_system as system;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		ComingId: pallet_coming_id::{Pallet, Call, Config<T>, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
	pub const ClaimValidatePeriod: u32 = 600;
	pub const CidsLimit: u32 = 5;
}

impl system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

impl pallet_coming_id::Config for Test {
	type Event = Event;
	type ClaimValidatePeriod = ClaimValidatePeriod;
	type CidsLimit = CidsLimit;
}

// Build test environment by setting the admin `key` for the Genesis.
pub fn new_test_ext(admin_key: u64) -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_coming_id::GenesisConfig::<Test>{
		admin_key: admin_key,
	}.assimilate_storage(&mut t).unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

/// Run until a particular block.
pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
	}
}

pub(crate) fn last_event() -> Event {
	system::Pallet::<Test>::events().pop().expect("Event expected").event
}

pub(crate) fn expect_event<E: Into<Event>>(e: E) {
	assert_eq!(last_event(), e.into());
}

pub(crate) fn last_events(n: usize) -> Vec<Event> {
	system::Pallet::<Test>::events().into_iter().rev().take(n).rev().map(|e| e.event).collect()
}

pub(crate) fn expect_events(e: Vec<Event>) {
	assert_eq!(last_events(e.len()), e);
}
