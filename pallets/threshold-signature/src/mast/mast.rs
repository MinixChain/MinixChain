#![allow(dead_code)]
#![allow(clippy::module_inception)]

use core::ops::AddAssign;

use super::error::MastError;
use super::XOnly;
use super::{
    error::Result, pmt::PartialMerkleTree, serialize, LeafNode, MerkleNode, TapBranchHash,
    TapLeafHash, TapTweakHash, VarInt,
};
#[cfg(not(feature = "std"))]
use alloc::{string::String, vec, vec::Vec};
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::scalar::Scalar;
use frame_support::sp_tracing::debug;
use hashes::{
    hex::{FromHex, ToHex},
    Hash,
};
use schnorrkel::PublicKey;

use sp_core::sp_std::ops::Deref;

/// Data structure that represents a partial mast tree
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Mast {
    /// All leaf nodes of the mast tree
    pub pubkeys: Vec<XOnly>,
}

impl Mast {
    /// Create a mast instance
    pub fn new(pubkeys: Vec<XOnly>) -> Self {
        Mast { pubkeys }
    }

    /// calculate merkle root
    pub fn calc_root(&self) -> Result<MerkleNode> {
        let leaf_nodes = self
            .pubkeys
            .iter()
            .map(tagged_leaf)
            .collect::<Result<Vec<_>>>()?;

        let mut matches = vec![true];

        if self.pubkeys.len() < 2 {
            return Err(MastError::MastBuildError);
        }
        matches.extend(&vec![false; self.pubkeys.len() - 1]);
        let pmt = PartialMerkleTree::from_leaf_nodes(&leaf_nodes, &matches)?;
        let mut matches_vec: Vec<LeafNode> = vec![];
        let mut indexes_vec: Vec<u32> = vec![];
        pmt.extract_matches(&mut matches_vec, &mut indexes_vec)
    }

    /// generate merkle proof
    pub fn generate_merkle_proof(&self, pubkey: &XOnly) -> Result<Vec<MerkleNode>> {
        let proof = {
            assert!(self.pubkeys.iter().any(|s| *s == *pubkey));
            let mut matches = vec![];
            let mut index = 9999;
            for (i, s) in self.pubkeys.iter().enumerate() {
                if *s == *pubkey {
                    matches.push(true);
                    index = i;
                } else {
                    matches.push(false)
                }
            }
            let leaf_nodes = self
                .pubkeys
                .iter()
                .map(tagged_leaf)
                .collect::<Result<Vec<_>>>()?;
            let filter_proof = MerkleNode::from_inner(leaf_nodes[index].into_inner());
            Ok(PartialMerkleTree::from_leaf_nodes(&leaf_nodes, &matches)?
                .collected_hashes(filter_proof))
        };

        if let Err(e) = proof {
            debug!("Mast genegerate address meet err: {:?}", e);
            Err(MastError::MastGenProofError)
        } else {
            proof
        }
    }

    /// generate threshold signature address
    pub fn generate_tweak_pubkey(&self, inner_pubkey: &XOnly) -> Result<Vec<u8>> {
        let root = self.calc_root()?;
        tweak_pubkey(inner_pubkey, &root)
    }
}

/// Calculate the leaf nodes from the pubkey
///
/// tagged_hash("TapLeaf", bytes([leaf_version]) + ser_size(pubkey))
pub fn tagged_leaf(pubkey: &XOnly) -> Result<LeafNode> {
    let mut x: Vec<u8> = vec![];
    x.extend(hex::decode("c0")?.iter());
    let ser_len = serialize(&VarInt(32u64))?;
    x.extend(&ser_len);
    x.extend(pubkey.deref());
    Ok(LeafNode::from_hex(&TapLeafHash::hash(&x).to_hex())?)
}

/// Calculate branch nodes from left and right children
///
/// tagged_hash("TapBranch", left + right)). The left and right nodes are lexicographic order
pub fn tagged_branch(left_node: MerkleNode, right_node: MerkleNode) -> Result<MerkleNode> {
    // If the hash of the left and right leaves is the same, it means that the total number of leaves is odd
    //
    // In this case, the parent hash is computed without copying
    // Note: `TapLeafHash` will replace the `TapBranchHash`
    if left_node != right_node {
        let mut x: Vec<u8> = vec![];
        let (left_node, right_node) = lexicographical_compare(left_node, right_node);
        x.extend(left_node.to_vec().iter());
        x.extend(right_node.to_vec().iter());

        Ok(MerkleNode::from_hex(&TapBranchHash::hash(&x).to_hex())?)
    } else {
        Ok(left_node)
    }
}

/// Lexicographic order of left and right nodes
fn lexicographical_compare(
    left_node: MerkleNode,
    right_node: MerkleNode,
) -> (MerkleNode, MerkleNode) {
    if right_node.to_vec() < left_node.to_vec() {
        (right_node, left_node)
    } else {
        (left_node, right_node)
    }
}

/// Compute tweak public key
pub fn tweak_pubkey(inner_pubkey: &[u8; 32], root: &MerkleNode) -> Result<Vec<u8>> {
    // P + hash_tweak(P||root)G
    let mut x: Vec<u8> = vec![];
    x.extend(inner_pubkey);
    x.extend(&root.to_vec());
    let tweak_key = TapTweakHash::hash(&x);
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&tweak_key[..]);

    let scalar = Scalar::from_bytes_mod_order(bytes);
    let base_point = RISTRETTO_BASEPOINT_POINT;

    let mut point = base_point * scalar;

    let inner_pubkey = PublicKey::from_bytes(inner_pubkey)?;
    point.add_assign(inner_pubkey.as_point());
    Ok(point.compress().as_bytes().to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::convert::TryFrom;
    use hashes::hex::ToHex;

    #[test]
    fn mast_generate_root_should_work() {
        let pubkey_a = XOnly::try_from(
            hex::decode("D69C3509BB99E412E68B0FE8544E72837DFA30746D8BE2AA65975F29D22DC7B9")
                .unwrap(),
        )
        .unwrap();
        let pubkey_b = XOnly::try_from(
            hex::decode("EEFDEA4CDB677750A420FEE807EACF21EB9898AE79B9768766E4FAA04A2D4A34")
                .unwrap(),
        )
        .unwrap();
        let pubkey_c = XOnly::try_from(
            hex::decode("DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659")
                .unwrap(),
        )
        .unwrap();
        let pubkeys = vec![pubkey_a, pubkey_b, pubkey_c];
        let mast = Mast { pubkeys };
        let root = mast.calc_root().unwrap();

        assert_eq!(
            "4ac28f45b41d96319f16141ec8433362f35cadb1a44a0e40aea424a5ef34d828",
            root.to_hex()
        );
    }

    #[test]
    fn mast_generate_merkle_proof_should_work() {
        let pubkey_a = XOnly::try_from(
            hex::decode("D69C3509BB99E412E68B0FE8544E72837DFA30746D8BE2AA65975F29D22DC7B9")
                .unwrap(),
        )
        .unwrap();
        let pubkey_b = XOnly::try_from(
            hex::decode("EEFDEA4CDB677750A420FEE807EACF21EB9898AE79B9768766E4FAA04A2D4A34")
                .unwrap(),
        )
        .unwrap();
        let pubkey_c = XOnly::try_from(
            hex::decode("DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659")
                .unwrap(),
        )
        .unwrap();
        let pubkeys = vec![pubkey_a, pubkey_b, pubkey_c];
        let mast = Mast { pubkeys };
        let proof = mast.generate_merkle_proof(&pubkey_a).unwrap();

        assert_eq!(
            proof.iter().map(|p| p.to_hex()).collect::<Vec<_>>(),
            vec![
                "f49b4c19bf53dfcdd50bc565ccca5cfc64226ef20301502f2264b25e2f0adb3a",
                "aa4bc1ce7be6887fad68d95fcf8b0d19788640ead71837ed43a2b518e673ba2f",
            ]
        )
    }

    #[test]
    fn test_final_addr() {
        let internal_key = XOnly::try_from(
            hex::decode("881102cd9cf2ee389137a99a2ad88447b9e8b60c350cda71aff049233574c768")
                .unwrap(),
        )
        .unwrap();

        let pubkey_a = XOnly::try_from(
            hex::decode("7c9a72882718402bf909b3c1693af60501c7243d79ecc8cf030fa253eb136861")
                .unwrap(),
        )
        .unwrap();
        let pubkey_b = XOnly::try_from(
            hex::decode("b69af178463918a181a8549d2cfbe77884852ace9d8b299bddf69bedc33f6356")
                .unwrap(),
        )
        .unwrap();
        let pubkey_c = XOnly::try_from(
            hex::decode("a20c839d955cb10e58c6cbc75812684ad3a1a8f24a503e1c07f5e4944d974d3b")
                .unwrap(),
        )
        .unwrap();
        let pubkeys = vec![pubkey_a, pubkey_b, pubkey_c];
        let mast = Mast { pubkeys };

        let addr = mast.generate_tweak_pubkey(&internal_key).unwrap();
        assert_eq!(
            "001604bef08d1fe4cefb2e75a2b786287821546f6acbe89570acc5d5a9bd5049",
            hex::encode(addr)
        );
    }
}
