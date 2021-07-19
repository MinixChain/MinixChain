#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use codec::{Decode, Encode};
use frame_support::inherent::Vec;
use frame_support::pallet_prelude::*;
use sp_runtime::traits::{Hash, StaticLookup};
use sp_core::H256;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum Status<AccountId> {
    // (secret_hash)
    WaitingToSign(H256),
    // (confirmer, new_party_b)
    WaitingToConfirm(Option<AccountId>, AccountId),
    CanTransfer,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Contract<AccountId, BlockNumber> {
    pub digest: H256,
    pub status: Status<AccountId>,
    pub updated: BlockNumber,
    pub party_a: Option<AccountId>,
    pub party_b: Option<AccountId>,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::dispatch::DispatchResult;
    use frame_system::pallet_prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// The period of validity about draft and transfer
        type Period: Get<<Self as frame_system::Config>::BlockNumber>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn contracts)]
    pub type Contracts<T: Config> =
    StorageMap<_, Blake2_128, H256, Contract<T::AccountId, T::BlockNumber>>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // (digest, drafter)
        Drafted(H256, T::AccountId),
        // (digest, signer)
        Signed(H256, T::AccountId),
        // (digest, new_party_b)
        Transferring(H256, T::AccountId),
        // (digest)
        Confirmed(H256),
        // (digest)
        Revoked(H256),
    }

    #[pallet::error]
    pub enum Error<T> {
        DigestIsExisted,
        DigestIsNotExisted,
        MismatchSecret,
        MismatchConfirmer,
        NotWaitingToSign,
        NotWaitingToConfirm,
        ExpiredToSign,
        ExpiredToConfirm,
        CannotTransfer,
        CannotRevoke,
        RequireTalentContractOwner,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1000)]
        pub fn draft(
            origin: OriginFor<T>,
            digest: H256,
            secret_hash: H256,
            is_party_a: bool
        ) -> DispatchResult {
            let drafter: T::AccountId = ensure_signed(origin)?;

            ensure!(!Contracts::<T>::contains_key(digest), Error::<T>::DigestIsExisted);

            Contracts::<T>::mutate(digest, |contract| {
                let party = if is_party_a {
                    (Some(drafter.clone()), None)
                } else {
                    (None, Some(drafter.clone()))
                };

                let new_contract = Contract{
                    digest: digest.clone(),
                    status: Status::WaitingToSign(secret_hash),
                    updated: <frame_system::Pallet<T>>::block_number(),
                    party_a: party.0,
                    party_b: party.1,
                };

                *contract = Some(new_contract);

                Self::deposit_event(Event::Drafted(digest, drafter));

                Ok(())
            })
        }

        #[pallet::weight(1000)]
        pub fn sign(
            origin: OriginFor<T>,
            digest: H256,
            secret: Vec<u8>,
        ) -> DispatchResult {
            let signer: T::AccountId = ensure_signed(origin)?;
            let secret_hash = T::Hashing::hash(&secret);

            Contracts::<T>::try_mutate_exists(digest.clone(), |contract| {
                let contract = contract
                    .as_mut()
                    .ok_or(Error::<T>::DigestIsNotExisted)?;

                match &contract.status {
                    Status::WaitingToSign(hash) => {
                        ensure!(
                            hash.as_ref() == secret_hash.as_ref(),
                            Error::<T>::MismatchSecret
                        )
                    },
                    _ => ensure!(false, Error::<T>::NotWaitingToSign),
                }

                let current = <frame_system::Pallet<T>>::block_number();
                ensure!(current <= contract.updated + T::Period::get(), Error::<T>::ExpiredToSign);

                contract.status = Status::CanTransfer;
                contract.updated = current;

                if contract.party_a.is_none() {
                    contract.party_a = Some(signer.clone());
                } else if contract.party_b.is_none() {
                    contract.party_b = Some(signer.clone());
                } else {
                    unreachable!("Never can reachable; qed");
                }

                Self::deposit_event(Event::Signed(digest, signer));

                Ok(())
            })
        }

        #[pallet::weight(1000)]
        pub fn transfer(
            origin: OriginFor<T>,
            digest: H256,
            new_party_b: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            let origin: T::AccountId = ensure_signed(origin)?;
            let new_party_b: T::AccountId = T::Lookup::lookup(new_party_b)?;

            Contracts::<T>::try_mutate_exists(digest.clone(), |contract| {
                let contract = contract
                    .as_mut()
                    .ok_or(Error::<T>::DigestIsNotExisted)?;

                let current = <frame_system::Pallet<T>>::block_number();

                match &contract.status {
                    Status::CanTransfer => {
                        ensure!(true, Error::<T>::CannotTransfer)
                    },
                    _ => ensure!(false, Error::<T>::CannotTransfer),
                }

                ensure!(
                    contract.party_a == Some(origin.clone())
                        || contract.party_b == Some(origin.clone()),
                    Error::<T>::RequireTalentContractOwner
                );

                let confirmer = if contract.party_a == Some(origin) {
                    contract.party_b.clone()
                } else {
                    contract.party_a.clone()
                };

                contract.status = Status::WaitingToConfirm(confirmer, new_party_b.clone());
                contract.updated = current;

                Self::deposit_event(Event::Transferring(digest, new_party_b));

                Ok(())
            })
        }

        #[pallet::weight(1000)]
        pub fn confirm(
            origin: OriginFor<T>,
            digest: H256,
        ) -> DispatchResult {
            let origin: T::AccountId = ensure_signed(origin)?;

            Contracts::<T>::try_mutate_exists(digest.clone(), |contract| {
                let contract = contract
                    .as_mut()
                    .ok_or(Error::<T>::DigestIsNotExisted)?;

                let current = <frame_system::Pallet<T>>::block_number();

                match &contract.status {
                    Status::WaitingToConfirm(Some(confirmer), new_party_b) => {
                        ensure!(confirmer.clone() == origin, Error::<T>::MismatchConfirmer);
                        ensure!(current <= contract.updated + T::Period::get(), Error::<T>::ExpiredToConfirm);

                        contract.party_b = Some(new_party_b.clone());
                        contract.updated = current;
                        contract.status = Status::CanTransfer;
                    },
                    _ => ensure!(false, Error::<T>::NotWaitingToConfirm),
                }

                Self::deposit_event(Event::Confirmed(digest));

                Ok(())
            })
        }

        #[pallet::weight(1000)]
        pub fn revoke(
            origin: OriginFor<T>,
            digest: H256,
        ) -> DispatchResult {
            let origin: T::AccountId = ensure_signed(origin)?;

            Contracts::<T>::try_mutate_exists(digest.clone(), |option_contract| {
                let contract = option_contract
                    .as_mut()
                    .ok_or(Error::<T>::DigestIsNotExisted)?;

                let current = <frame_system::Pallet<T>>::block_number();
                let is_expired = current <= contract.updated + T::Period::get();
                let is_owner = contract.party_a == Some(origin.clone())
                    || contract.party_b == Some(origin.clone());

                match &contract.status {
                    Status::WaitingToConfirm(_, _) if is_expired || is_owner => {
                        contract.status = Status::CanTransfer;
                        contract.updated = current;
                    },
                    Status::WaitingToSign(_) if is_expired || is_owner => {
                        *option_contract = None;
                    }
                    _ => ensure!(false, Error::<T>::CannotRevoke),
                }

                Self::deposit_event(Event::Revoked(digest));

                Ok(())
            })
        }
    }
}

impl<T: Config> Pallet<T> {
    pub fn get_contract(digest: H256) -> Option<Contract<T::AccountId, T::BlockNumber>> {
        Self::contracts(digest)
    }
}
