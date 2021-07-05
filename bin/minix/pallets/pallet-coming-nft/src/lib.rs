#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
//pub use weights::WeightInfo;

use frame_support::inherent::Vec;
use sp_runtime::traits::StaticLookup;

use pallet_coming_id::{Cid, ComingNFT};

use frame_support::pallet_prelude::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::dispatch::DispatchResult;
    use frame_system::pallet_prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Weight information for extrinsics in this pallet.
        type ComingNFT: ComingNFT<Self::AccountId>;
        //type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1000)]
        pub fn mint(origin: OriginFor<T>, cid: Cid, card: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            T::ComingNFT::mint(&who, cid, card)
        }

        #[pallet::weight(1000)]
        pub fn transfer(
            origin: OriginFor<T>,
            cid: Cid,
            recipient: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let recipient = T::Lookup::lookup(recipient)?;

            T::ComingNFT::transfer(&who, cid, &recipient)
        }
    }
}
