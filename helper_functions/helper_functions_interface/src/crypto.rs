use bls::{AggregatePublicKey, PublicKey, PublicKeyBytes, SignatureBytes};
use ssz::DecodeError;
use types::primitives::Domain;

// ok
pub fn hash(_input: &[u8]) -> Vec<u8> {
    [].to_vec()
}

// ok
//pub fn hash_tree_root(_object : obj) -> H256 {
//    use TreeRoot derive
//}

// ok
//pub fn signed_root(_object : obj) -> H256 {
//    use SignedRoot derive
//}

// ok
pub fn bls_verify(
    _pubkey: &PublicKeyBytes,
    _message: &[u8],
    _signature: &SignatureBytes,
    _domain: Domain,
) -> Result<bool, DecodeError> {
    Ok(true)
}

// ok
pub fn bls_verify_multiple(
    _pubkeys: &[&PublicKeyBytes],
    _messages: &[&[u8]],
    _signature: &SignatureBytes,
    _domain: Domain,
) -> Result<bool, DecodeError> {
    Ok(true)
}

// ok
pub fn bls_aggregate_pubkeys(_pubkeys: &[PublicKey]) -> AggregatePublicKey {
    AggregatePublicKey::new()
}
