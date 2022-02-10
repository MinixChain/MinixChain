#![cfg_attr(not(feature = "std"), no_std)]
#![feature(exclusive_range_pattern)]
#![allow(clippy::unused_unit)]

pub use pallet::*;

use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use sp_runtime::traits::StaticLookup;
pub use pallet_coming_id::{Cid, Distributed, Error as ComingIdError};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
//
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// pub mod nft;
// pub mod weights;

#[derive(Clone, Eq, PartialEq, Encode, Decode, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct ReputationGrade {
    pub key1: u32,
    pub key2: u32,
    pub key3: u32,
}


#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::dispatch::DispatchResult;
    use frame_system::pallet_prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config+ pallet_coming_id::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::type_value]
    pub fn DefaultReputationGrade() -> ReputationGrade {
        ReputationGrade {
            key1:Default::default(),
            key2:Default::default(),
            key3:Default::default(),
        }
    }

    #[pallet::storage]
    #[pallet::getter(fn cid_grade)]
    pub type CidReputationGrade<T: Config> =
        StorageMap<_, Blake2_128Concat, Cid, ReputationGrade,ValueQuery,DefaultReputationGrade>;

    /// The pallet admin key.
    #[pallet::storage]
    #[pallet::getter(fn admin_key)]
    pub type Admin<T: Config> = StorageValue<_, T::AccountId>;

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
        // recipient, cid
        UpReputationGrade(Cid,ReputationGrade),
    }

    #[pallet::error]
    pub enum Error<T> {
        RequireAdmin,
        CannotDowngrade,
        InvalidCid,
        UndistributedCid,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn up_grade(
            origin: OriginFor<T>,
            cid: Cid,
            grade: u32
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_admin(who), Error::<T>::RequireAdmin);
            Self::check_cid_grade(cid,grade)?;
            let new_reputation_grade = ReputationGrade{
                key1: grade,
                key2: 0,
                key3: 0
            };
            CidReputationGrade::<T>::mutate(cid,|old_grade| *old_grade = new_reputation_grade.clone());
            Self::deposit_event(Event::UpReputationGrade(
                cid,
                new_reputation_grade
            ));
            Ok(())
        }
        #[pallet::weight(0)]
        pub fn set_admin(
            origin: OriginFor<T>,
            new_admin: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let new_admin = T::Lookup::lookup(new_admin)?;

            Admin::<T>::mutate(|admin| *admin = Some(new_admin));

            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    pub fn get_grade(cid:Cid) -> Option<ReputationGrade>{
        Some(CidReputationGrade::<T>::get(cid))
    }
    fn is_admin(who: T::AccountId) -> bool {
        matches!(Admin::<T>::get(), Some(admin) if admin == who)
    }
    fn check_cid_grade(cid: Cid, grade: u32) -> DispatchResult {
        ensure!(Distributed::<T>::contains_key(cid),Error::<T>::UndistributedCid);
        let old_grade = CidReputationGrade::<T>::get(cid);
        ensure!(
            old_grade.key1 <= grade,
            Error::<T>::CannotDowngrade
        );
        Ok(())
    }

}
