use crate as pallet_coming_auction;
use frame_support::{
    parameter_types,
    traits::{GenesisBuild, OnFinalize, OnInitialize},
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

pub use pallet_balances::Error as BalancesError;
pub use pallet_coming_auction::{
    Auction, Cid, ComingIdError, ComingNFT, Config, Error, Event as AuctionEvent, PalletAuctionId,
    MIN_DURATION,
};

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
        Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
        ComingId: pallet_coming_id::{Pallet, Call, Config<T>, Storage, Event<T>},
        ComingAuction: pallet_coming_auction::{Pallet, Call, Config<T>, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
    pub const MaxDataSize: u32 = 1024 * 1024;
}

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    // AccountId must be u128 for auction account
    type AccountId = u128;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u128>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 500;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
    pub const AuctionId: PalletAuctionId = PalletAuctionId(*b"/auc");
}

impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    /// The type for recording an account's balance.
    type Balance = u128;
    /// The ubiquitous event type.
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}

impl pallet_coming_id::Config for Test {
    type Event = Event;
    type WeightInfo = ();
    type MaxDataSize = MaxDataSize;
}

impl pallet_coming_auction::Config for Test {
    type ComingNFT = ComingId;
    type Event = Event;
    type Currency = Balances;
    type PalletId = AuctionId;
    type WeightInfo = ();
}

// Build test environment by setting the admin `key` for the Genesis.
pub fn new_test_ext(
    admin_key: <Test as frame_system::Config>::AccountId,
) -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (1, 10_000_000_000),
            (2, 10_000_000_000),
            (3, 1_000_000_000),
            (4, 10_000_000_000),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    pallet_coming_id::GenesisConfig::<Test> {
        high_admin_key: admin_key,
        medium_admin_key: admin_key,
        medium_admin_key2: admin_key,
        medium_admin_key3: admin_key,
        low_admin_key: admin_key,
    }
    .assimilate_storage(&mut t)
    .unwrap();

    pallet_coming_auction::GenesisConfig::<Test> {
        admin_key: Some(admin_key),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

pub(crate) fn last_event() -> Event {
    system::Pallet::<Test>::events()
        .pop()
        .expect("Event expected")
        .event
}

pub(crate) fn expect_event<E: Into<Event>>(e: E) {
    assert_eq!(last_event(), e.into());
}

/// Run until a particular block.
pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        Balances::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        Balances::on_initialize(System::block_number());
    }
}
