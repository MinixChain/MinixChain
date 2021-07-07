
use super::*;

pub trait ComingNFT<AccountId> {
    fn mint(
        who: &AccountId,
        cid: Cid,
        card: Vec<u8>
    ) -> DispatchResult;

    fn burn(
        who: &AccountId,
        cid: Cid,
    ) -> DispatchResult;

    fn transfer(
        who: &AccountId,
        cid: Cid,
        recipient: &AccountId
    ) -> DispatchResult;

    fn cids_of_owner(owner: AccountId) -> Vec<Cid>;

    fn owner_of_cid(cid: Cid) -> Option<AccountId>;

    fn card_of_cid(cid: Cid) -> Option<Bytes>;
}
