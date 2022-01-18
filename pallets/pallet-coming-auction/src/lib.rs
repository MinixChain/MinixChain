#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

pub use pallet::*;
pub use pallet_coming_id::{Cid, ComingNFT, Error as ComingIdError};
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

use frame_support::{
    pallet_prelude::*,
    traits::{Currency, ExistenceRequirement},
    transactional
};
use sp_runtime::{
    traits::{AccountIdConversion, UniqueSaturatedInto, StaticLookup},
    TypeId
};
use sp_std::vec::Vec;
use sp_arithmetic::helpers_128bit::multiply_by_rational;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_arithmetic::traits::Saturating;

#[derive(Clone, Eq, PartialEq, Encode, Decode, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Auction<AccountId, Balance, BlockNumber> {
    pub seller: AccountId,
    pub start_price: Balance,
    pub end_price: Balance,
    pub duration: BlockNumber,
    pub start: BlockNumber,
}

/// A pallet identifier. These are per pallet and should be stored in a registry somewhere.
#[derive(Clone, Copy, Eq, PartialEq, Encode, Decode, scale_info::TypeInfo)]
pub struct PalletAuctionId(pub [u8; 4]);

impl TypeId for PalletAuctionId {
    const TYPE_ID: [u8; 4] = *b"modl";
}

pub const MIN_DURATION: u32 = 100;

pub type BalanceOf<T> =
<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// Use percentile. [0%, 255%]
pub struct DefaultRemintPoint;
impl Get<u8> for DefaultRemintPoint {
    fn get() -> u8 { 50u8 }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::dispatch::DispatchResult;
    use frame_system::pallet_prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_coming_id::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// The implement of ComingNFT triat, eg. pallet-coming-id
        type ComingNFT: ComingNFT<Self::AccountId>;
        /// The native balance
        type Currency: Currency<Self::AccountId>;
        /// This pallet id.
        #[pallet::constant]
        type PalletId: Get<PalletAuctionId>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    /// The auctions in progress
    #[pallet::storage]
    #[pallet::getter(fn auctions)]
    pub type Auctions<T: Config> =
    StorageMap<_, Twox64Concat, Cid, Auction<T::AccountId, BalanceOf<T>, T::BlockNumber>>;

    /// The auction stats.
    /// (total_auctions, success_auctions, cancel_auctions)
    #[pallet::storage]
    #[pallet::getter(fn stats)]
    pub(super) type Stats<T: Config> = StorageValue<_, (u64, u64, u64), ValueQuery>;

    /// The pallet admin key.
    #[pallet::storage]
    #[pallet::getter(fn admin_key)]
    pub(super) type Admin<T: Config> = StorageValue<_, T::AccountId>;

    /// The protocol fee point.
    /// [0‱, 255‱] or [0%, 2.55%]
    #[pallet::storage]
    #[pallet::getter(fn point)]
    pub(super) type Point<T: Config> = StorageValue<_, u8, ValueQuery>;

    /// The remint fee point.
    /// [0%, 255%]
    #[pallet::storage]
    #[pallet::getter(fn remint_point)]
    pub(super) type RemintPoint<T: Config> = StorageValue<_, u8, ValueQuery, DefaultRemintPoint>;

    /// The emergency stop.
    #[pallet::storage]
    #[pallet::getter(fn in_emergency)]
    pub(super) type InEmergency<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        /// The `AccountId` of the admin key.
        pub admin_key: Option<T::AccountId>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                admin_key: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            if let Some(key) = &self.admin_key {
                <Admin<T>>::put(key.clone());
            }
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // cid, seller, start_price, end_price, duration, start_at
        AuctionCreated(Cid, T::AccountId, BalanceOf<T>, BalanceOf<T>, T::BlockNumber, T::BlockNumber),
        // cid, buyer, price, buy_at
        AuctionSuccessful(Cid, T::AccountId, BalanceOf<T>, T::BlockNumber),
        // cid, cancel_at
        AuctionCanceled(Cid, T::BlockNumber),
        // pause_at
        Paused(T::BlockNumber),
        // unpause_at
        UnPaused(T::BlockNumber),
    }

    #[pallet::error]
    pub enum Error<T> {
        OnAuction,
        NotOnAuction,
        TooLittleDuration,
        InEmergency,
        OnlyInEmergency,
        NotMatchSeller,
        RequireAdmin,
        LowBidValue,
        LessThanMinBalance
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(<T as pallet::Config>::WeightInfo::create())]
        pub fn create(
            origin: OriginFor<T>,
            cid: Cid,
            start_price: BalanceOf<T>,
            end_price: BalanceOf<T>,
            duration: T::BlockNumber
        ) -> DispatchResult {
            ensure!(!Self::is_in_emergency(), Error::<T>::InEmergency);
            ensure!(!Self::is_on_auction(cid), Error::<T>::OnAuction);
            ensure!(Self::is_more_min_balance(start_price, end_price), Error::<T>::LessThanMinBalance);
            ensure!(duration >= T::BlockNumber::from(MIN_DURATION), Error::<T>::TooLittleDuration);

            let seller = ensure_signed(origin)?;
            let auction_account = Self::auction_account_id(cid);
            let start = Self::now();

            // temporarily escrow
            T::ComingNFT::transfer(
                &seller,
                cid,
                &auction_account,
            )?;

            Auctions::<T>::insert(cid, Auction{
                seller: seller.clone(),
                start_price,
                end_price,
                duration,
                start,
            });

            // update total_auctions
            Self::stats_mutate(|total_auctions, _success_auctions, _cancel_auctions, |{
                *total_auctions = total_auctions.saturating_add(1);
            });

            Self::deposit_event(Event::AuctionCreated(cid, seller, start_price, end_price, duration, start));

            Ok(())
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::bid())]
        #[transactional]
        pub fn bid(
            origin: OriginFor<T>,
            cid: Cid,
            value: BalanceOf<T>,
        ) -> DispatchResult {
            ensure!(!Self::is_in_emergency(), Error::<T>::InEmergency);
            ensure!(Self::is_on_auction(cid), Error::<T>::NotOnAuction);

            let buyer = ensure_signed(origin)?;
            let auction_account = Self::auction_account_id(cid);

            Auctions::<T>::mutate_exists(cid, |auction|{
                if let Some(inner_auction) = auction {
                    let current_price =  Self::get_current_price(cid);

                    ensure!(value >= current_price, Error::<T>::LowBidValue);

                    let service_fee = match Admin::<T>::get() {
                        Some(admin) => {
                            let fee_point = Point::<T>::get();
                            let service_fee = Self::calculate_fee(value, fee_point);

                            // transfer `service_fee` to admin
                            T::Currency::transfer(
                                &buyer,
                                &admin,
                                service_fee,
                                ExistenceRequirement::KeepAlive
                            )?;

                            service_fee
                        }
                        None => BalanceOf::<T>::default(),
                    };

                    let tax_fee = match T::ComingNFT::card_of_meta(cid){
                        Some(meta) => {
                            let tax_fee = Self::calculate_fee(value, meta.tax_point);

                            // transfer `tax_fee` to issuer
                            T::Currency::transfer(
                                &buyer,
                                &meta.issuer,
                                tax_fee,
                                ExistenceRequirement::KeepAlive
                            )?;

                            tax_fee
                        },
                        None => BalanceOf::<T>::default(),
                    };

                    let to_seller = value
                        .saturating_sub(service_fee)
                        .saturating_sub(tax_fee);

                    T::Currency::transfer(
                        &buyer,
                        &inner_auction.seller,
                        to_seller,
                        ExistenceRequirement::KeepAlive
                    )?;

                    // transfer cid to buyer
                    T::ComingNFT::transfer(
                        &auction_account,
                        cid,
                        &buyer,
                    )?;

                    // remove this auction
                    *auction = None;

                    // update success_auctions
                    Self::stats_mutate(|_total_auctions, success_auctions, _cancel_auctions, |{
                        *success_auctions = success_auctions.saturating_add(1);
                    });

                    Self::deposit_event(Event::AuctionSuccessful(cid, buyer, value, Self::now()));
                }

                Ok(())
            })
        }

        #[pallet::weight(0)]
        #[transactional]
        pub fn remint(
            origin: OriginFor<T>,
            cid: Cid,
            card: Vec<u8>,
            tax_point: u8
        ) -> DispatchResult {
            ensure!(!Self::is_in_emergency(), Error::<T>::InEmergency);
            let who = ensure_signed(origin)?;

            // 1. remint fee
            let remint = match T::ComingNFT::card_of_meta(cid) {
                Some(meta) => meta.remint,
                None => 0,
            };
            let remint_fee = Self::calculate_remint_fee(remint);

            if let Some(admin) = Admin::<T>::get() {
                // transfer `remint fee` to admin
                T::Currency::transfer(
                    &who,
                    &admin,
                    remint_fee,
                    ExistenceRequirement::KeepAlive
                )?;
            }

            // 2. remint
            T::ComingNFT::remint(&who, cid, card, tax_point)?;

            Ok(())
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::cancel())]
        pub fn cancel(
            origin: OriginFor<T>,
            cid: Cid,
        ) -> DispatchResult {
            ensure!(Self::is_on_auction(cid), Error::<T>::NotOnAuction);

            let seller = ensure_signed(origin)?;
            let auction_account = Self::auction_account_id(cid);

            Auctions::<T>::mutate_exists(cid, |auction| {
                if let Some(inner_auction) = auction {
                    ensure!(inner_auction.seller == seller, Error::<T>::NotMatchSeller);

                    // transfer back to seller
                    T::ComingNFT::transfer(
                        &auction_account,
                        cid,
                        &seller,
                    )?;

                    // remove this auction
                    *auction = None;

                    // update cancel_auctions
                    Self::stats_mutate(|_total_auctions, _success_auctions, cancel_auctions, |{
                        *cancel_auctions = cancel_auctions.saturating_add(1);
                    });

                    Self::deposit_event(Event::AuctionCanceled(cid, Self::now()));
                }

                Ok(())
            })
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::pause())]
        pub fn pause(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_admin(who), Error::<T>::RequireAdmin);

            InEmergency::<T>::try_mutate(|in_emergency| {
                if !*in_emergency {
                    *in_emergency = true;

                    Self::deposit_event(Event::Paused(Self::now()));
                }

                Ok(())
            })
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::unpause())]
        pub fn unpause(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_admin(who), Error::<T>::RequireAdmin);

            InEmergency::<T>::try_mutate(|in_emergency| {
                if *in_emergency {
                    *in_emergency = false;

                    Self::deposit_event(Event::UnPaused(Self::now()));
                }

                Ok(())
            })
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::cancel_when_pause())]
        pub fn cancel_when_pause(
            origin: OriginFor<T>,
            cid: Cid
        ) -> DispatchResult {
            ensure!(Self::is_in_emergency(), Error::<T>::OnlyInEmergency);

            let who = ensure_signed(origin)?;
            ensure!(Self::is_admin(who), Error::<T>::RequireAdmin);

            let auction_account = Self::auction_account_id(cid);

            Auctions::<T>::mutate_exists(cid, |auction| {
                if let Some(inner_auction) = auction {
                    // transfer back to seller
                    T::ComingNFT::transfer(
                        &auction_account,
                        cid,
                        &inner_auction.seller,
                    )?;

                    // remove this auction
                    *auction = None;

                    // update cancel_auctions
                    Self::stats_mutate(|_total_auctions, _success_auctions, cancel_auctions, |{
                        *cancel_auctions = cancel_auctions.saturating_add(1);
                    });

                    Self::deposit_event(Event::AuctionCanceled(cid, Self::now()));
                }

                Ok(())
            })
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_fee_point())]
        pub fn set_fee_point(
            origin: OriginFor<T>,
            new_point: u8
        ) -> DispatchResult {
            ensure!(!Self::is_in_emergency(), Error::<T>::InEmergency);
            let who = ensure_signed(origin)?;
            ensure!(Self::is_admin(who), Error::<T>::RequireAdmin);

            Point::<T>::mutate(|point| *point = new_point);

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn set_remint_point(
            origin: OriginFor<T>,
            new_point: u8
        ) -> DispatchResult {
            ensure!(!Self::is_in_emergency(), Error::<T>::InEmergency);
            let who = ensure_signed(origin)?;
            ensure!(Self::is_admin(who), Error::<T>::RequireAdmin);

            RemintPoint::<T>::mutate(|point| *point = new_point);

            Ok(())
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_admin())]
        pub fn set_admin(
            origin: OriginFor<T>,
            new_admin: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let new_admin = T::Lookup::lookup(new_admin)?;

            Admin::<T>::mutate(|admin|{
                *admin = Some(new_admin)
            });

            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    fn is_on_auction(cid: Cid) -> bool {
        Auctions::<T>::contains_key(cid)
    }

    fn is_in_emergency() -> bool {
        InEmergency::<T>::get()
    }

    fn is_admin(who: T::AccountId) -> bool {
        matches!(Admin::<T>::get(), Some(admin) if admin == who)
    }

    fn is_more_min_balance(start: BalanceOf<T>, end: BalanceOf<T>) -> bool {
        let min = T::Currency::minimum_balance();

        start > min && end > min
    }

    fn now() -> T::BlockNumber {
        frame_system::Pallet::<T>::block_number()
    }

    fn stats_mutate<F: FnMut(&mut u64, &mut u64, &mut u64)>(mut f: F) {
        <Stats<T>>::mutate(|stats|{
            f(&mut stats.0, &mut stats.1, &mut stats.2)
        })
    }

    /// The account ID of an auction account
    /// "modl" + "/auc" is 8 bytes
    /// for AccountId16(u128) which used for tests, 8 bytes remaining for Cid, just ok.
    /// for AccountId32, 24 bytes remaining for Cid, it's enough.
    /// ```golang
    /// func auction_account(cid uint64) []byte {
    ///     // TYPE_ID = "modl"
    ///     var type_id = []byte{109, 111, 100, 108}
    ///     // PalletAuction_ID = "/auc"
    ///     var auction_id = []byte{47, 97, 117, 99}
    ///
    ///     var encode_cid = make([]byte, 8)
    ///     binary.LittleEndian.PutUint64(encode_cid, cid)
    ///
    ///     var account_id = make([]byte, 32)
    ///     copy(account_id[:4], type_id)
    ///     copy(account_id[4:8], auction_id)
    ///     copy(account_id[8:16], encode_cid)
    ///
    ///     return account_id
    /// }
    /// ```
    ///
    pub fn auction_account_id(cid: Cid) -> T::AccountId {
        T::PalletId::get().into_sub_account(cid)
    }

    pub fn get_current_price(cid: Cid) -> BalanceOf<T> {
        match Self::auctions(cid) {
            Some(Auction{start_price, end_price, start, duration, .. }) => {
                Self::calculate_price(start_price, end_price, duration, Self::now() - start)
            },
            None => Default::default()
        }
    }

    pub fn get_current_remint_fee(cid: Cid) -> BalanceOf<T> {
        let remint = match T::ComingNFT::card_of_meta(cid) {
            Some(meta) => meta.remint,
            None => 0,
        };

        Self::calculate_remint_fee(remint)
    }

    pub fn calculate_price(
        start: BalanceOf<T>,
        end: BalanceOf<T>,
        duration: T::BlockNumber,
        passed: T::BlockNumber,
    ) -> BalanceOf<T> {
        if passed >= duration {
            return end
        }

        let start_u128: u128 = start.unique_saturated_into();
        let end_u128: u128 = end.unique_saturated_into();
        let duration_u128: u128 = duration.unique_saturated_into();
        let passed_u128: u128 = passed.unique_saturated_into();

        let total_price_change = if start_u128 > end_u128 {
            start_u128 - end_u128
        } else {
            end_u128 - start_u128
        };

        match multiply_by_rational(total_price_change, passed_u128, duration_u128) {
            Ok(current_price_change_u128) => {
                use sp_std::convert::TryFrom;
                match BalanceOf::<T>::try_from(current_price_change_u128) {
                    Ok(current_price_change) =>  {
                        if start_u128 > end_u128 {
                            start - current_price_change
                        } else {
                            start + current_price_change
                        }
                    },
                    Err(_) => start
                }
            }
            Err(_) => start
        }
    }

    pub fn calculate_fee(value: BalanceOf<T>, fee_point: u8) -> BalanceOf<T> {
        let point = BalanceOf::<T>::from(fee_point);
        let base_point = BalanceOf::<T>::from(10000u16);

        // Impossible to overflow
        value / base_point * point
    }

    pub fn calculate_remint_fee(remint: u8) -> BalanceOf<T> {
        let remint_point = BalanceOf::<T>::from(RemintPoint::<T>::get());
        let base_point = BalanceOf::<T>::from(100u16);

        let fee_ladder = BalanceOf::<T>::from(100000000u32)
            .saturating_mul(
                BalanceOf::<T>::from(
                        2u32
                            .checked_pow(remint as u32)
                            .unwrap_or(4294967295u32)
                    )
            );

        // Impossible to overflow
        fee_ladder / base_point * remint_point
    }

    pub fn balance_of(account: &T::AccountId) -> BalanceOf<T> {
        T::Currency::free_balance(account)
    }

    pub fn get_stats() -> (u64, u64, u64) {
        <Stats<T>>::get()
    }

    pub fn get_auction(cid: Cid) -> Option<Auction<T::AccountId, BalanceOf<T>, T::BlockNumber>> {
        Auctions::<T>::get(cid)
    }

    pub fn get_fee_point() -> u8 {
        Point::<T>::get()
    }
}
