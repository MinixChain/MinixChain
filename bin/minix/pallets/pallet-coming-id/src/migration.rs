use super::*;
use frame_support::pallet_prelude::{ValueQuery, StorageValue};

pub struct __OldAdminKey<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> frame_support::traits::StorageInstance for __OldAdminKey<T> {
    fn pallet_prefix() -> &'static str { "ComingId" }
    const STORAGE_PREFIX: &'static str = "Key";
}

pub type OldAdminKey<T> = StorageValue<
    __OldAdminKey<T>, <T as frame_system::Config>::AccountId, ValueQuery
>;

/// A storage migration that remove old admin key and set new admin keys
pub fn update_keys<T: Config>(
    high_key: T::AccountId,
    medium_key: T::AccountId,
    low_key: T::AccountId
) -> Weight {
    let mut writes = 0;
    let mut reads = 0;

    let is_exists = OldAdminKey::<T>::get() != T::AccountId::default();
    log::info!("ComingId: old key is exists? {}", is_exists);

    if is_exists {
        log::info!("ComingId: update high key {:?}", high_key.clone());
        HighKey::<T>::put(high_key);
        log::info!("ComingId: update medium key {:?}", medium_key.clone());
        MediumKey::<T>::put(medium_key);
        log::info!("ComingId: update low key {:?}", low_key.clone());
        LowKey::<T>::put(low_key);

        log::info!("ComingId: kill old key");
        OldAdminKey::<T>::kill();

        writes += 3;
    }

    reads += 1;

    T::DbWeight::get().writes(writes) + T::DbWeight::get().reads(reads)
}

pub fn high_key<AccountId: Decode + Default>() -> AccountId {
    AccountId::decode(
        &mut &b"fc4ea146bf1f19bc7b828c19be1f7d764c55108c8aaf6075d00c9fa7da1eca75"[..]
    ).unwrap_or_default()
}

pub fn medium_key<AccountId: Decode + Default>() -> AccountId {
    AccountId::decode(
        &mut & b"74092de518c6394d5ec2d8915c22822d0d62cc699ce8d9177c38e812a3ed3565"[..]
    ).unwrap_or_default()
}

pub fn low_key<AccountId: Decode + Default>() -> AccountId {
    AccountId::decode(
        &mut &b"f412fd28e2835691047a49d83608c19249711b36d09c61c634566c003b3bc660"[..]
    ).unwrap_or_default()
}
