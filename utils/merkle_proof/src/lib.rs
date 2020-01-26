use eth2_hashing::hash;
use ethereum_types::H256;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub enum MerkleProofError {
    /// Params of not equal length were given
    InvalidParamLength { len_first: usize, len_second: usize },
}

#[macro_use]
// logoritmic function
macro_rules! log_of {
    ($val:expr, $base:expr, $type:ty) => {
        ($val as f32).log($base) as $type
    };
}

//concats 2 vectors
fn concat(mut vec1: Vec<u8>, mut vec2: Vec<u8>) -> Vec<u8> {
    vec1.append(&mut vec2);
    vec1
}

// concats and then hashes 2 vectors
fn hash_and_concat(h1: H256, h2: H256) -> H256 {
    H256::from_slice(&hash(&concat(
        h1.as_bytes().to_vec(),
        h2.as_bytes().to_vec(),
    )))
}

//Testing functions
fn fill_zero_hashes() -> Vec<H256> {
    let mut zerohashes = vec![H256::zero()];
    for layer in 1..100 {
        zerohashes.push(hash_and_concat(
            zerohashes[layer - 1],
            zerohashes[layer - 1],
        ));
    }
    zerohashes
}

fn calc_merkle_tree_from_leaves(leaves: &[H256], layer_count: usize) -> Vec<H256> {
    let mut leaves_list = leaves.to_vec();
    let mut tree = leaves_list.clone();
    let zerohashes = fill_zero_hashes();
    for item in zerohashes.iter().take(layer_count) {
        if leaves_list.len() % 2 == 1 {
            let zerohash = item.clone();
            leaves_list.push(zerohash);
        }
        leaves_list = get_concated_list(&leaves_list);
        tree.append(&mut leaves_list.clone());
    }
    tree
}

fn get_concated_list(nodes: &[H256]) -> Vec<H256> {
    let mut nodes_answer: Vec<H256> = vec![];
    for x in (0..nodes.len()).step_by(2) {
        nodes_answer.push(hash_and_concat(nodes[x], nodes[x + 1]));
    }
    nodes_answer
}

fn get_merkle_root(leaves: &[H256], pad_to: usize) -> H256 {
    let zerohashes = fill_zero_hashes();
    if pad_to == 0_usize {
        zerohashes[0_usize]
    } else if leaves.is_empty() {
        let layer_count = log_of!(pad_to, 2., usize);
        zerohashes[layer_count]
    } else {
        let layer_count = log_of!(pad_to, 2., usize);
        let answer = calc_merkle_tree_from_leaves(leaves, layer_count);
        answer[answer.len() - 1]
    }
}

fn e(mut v: u8) -> H256 {
    if v == 0 {
        v += 1;
    }
    H256::repeat_byte(v)
}

fn z(i: usize) -> H256 {
    let zerohashes = fill_zero_hashes();
    zerohashes[i]
}

fn h(h1: H256, h2: H256) -> H256 {
    H256::from_slice(&hash(&concat(
        h1.as_bytes().to_vec(),
        h2.as_bytes().to_vec(),
    )))
}

fn return_chunks(count: u8) -> Vec<H256> {
    let mut chunks: Vec<H256> = vec![];
    for x in 0..count {
        chunks.push(e(x));
    }
    chunks
}

fn test_get_merkle_root(count: u8, limit: usize, value: H256) -> bool {
    let chunks = return_chunks(count);
    get_merkle_root(&chunks, limit) == value
}

//returns previous power of 2
fn get_previous_power_of_two(x: usize) -> usize {
    if x <= 2 {
        x
    } else {
        2 * get_previous_power_of_two(x / 2)
    }
}

//returns next power of 2
fn get_next_power_of_two(x: usize) -> usize {
    if x <= 2 {
        x
    } else {
        2 * get_next_power_of_two((x + 1) / 2)
    }
}

// length of path
fn get_generalized_index_length(index: usize) -> usize {
    log_of!(index, 2., usize)
}

const fn get_generalized_index_bit(index: usize, position: usize) -> bool {
    (index & (0x01 << position)) > 0
}

//get index sibling
const fn generalized_index_sibling(index: usize) -> usize {
    index ^ 1
}

// get index child
fn generalized_index_child(index: usize, right_side: bool) -> usize {
    let is_right = if right_side { 1 } else { 0 };
    index * 2 + is_right
}

//get index parent
const fn generalized_index_parent(index: usize) -> usize {
    index / 2
}

// get indices of sister chunks
fn get_branch_indices(tree_index: usize) -> Vec<usize> {
    let mut branch = vec![generalized_index_sibling(tree_index)];
    while branch.last() > Some(&1_usize) {
        let index = branch.last().cloned().expect("Something is wrong");
        let mut next_index = vec![generalized_index_sibling(generalized_index_parent(index))];
        branch.append(&mut next_index);
    }
    branch
}

// get path indices
fn get_path_indices(tree_index: usize) -> Vec<usize> {
    let mut path = vec![tree_index];
    while path.last() > Some(&1_usize) {
        let index = path.last().cloned().expect("Something is wrong");
        path.append(&mut vec![generalized_index_parent(index)]);
    }
    path
}

//get all indices of all indices needed for the proof
fn get_helper_indices(indices: &[usize]) -> Vec<usize> {
    let mut all_helper_indices: Vec<usize> = vec![];
    let mut all_path_indices: Vec<usize> = vec![];
    for index in indices.iter() {
        all_helper_indices.append(&mut get_branch_indices(*index).clone());
        all_path_indices.append(&mut get_path_indices(*index).clone());
    }

    let pre_answer = hashset(&all_helper_indices);
    let pre_answer_2 = hashset(&all_path_indices);

    let mut hash_answer: HashSet<usize> = pre_answer.difference(&pre_answer_2).cloned().collect();
    let mut vector_answer: Vec<usize> = Vec::with_capacity(hash_answer.len());

    for i in hash_answer.drain() {
        vector_answer.push(i);
    }

    vector_answer.sort();
    reverse_vector(&vector_answer)
}

//reverts the vector
fn reverse_vector(data: &[usize]) -> Vec<usize> {
    data.iter().rev().cloned().collect()
}

//vector to hashset
fn hashset(data: &[usize]) -> HashSet<usize> {
    HashSet::from_iter(data.iter().cloned())
}

// merkle proof
pub fn verify_merkle_proof(
    leaf: H256,
    proof: &[H256],
    _depth: usize, // not needed
    index: usize,
    root: H256,
) -> Result<bool, MerkleProofError> {
    match calculate_merkle_root(leaf, proof, index) {
        Ok(calculated_root) => Ok(calculated_root == root),
        Err(err) => Err(err),
    }
}

fn calculate_merkle_root(
    leaf: H256,
    proof: &[H256],
    index: usize,
) -> Result<H256, MerkleProofError> {
    if proof.len() != get_generalized_index_length(index) {
        return Err(MerkleProofError::InvalidParamLength {
            len_first: proof.len(),
            len_second: get_generalized_index_length(index),
        });
    }
    let mut root = leaf;

    for (i, &proof_step) in proof.iter().enumerate() {
        if get_generalized_index_bit(index, i) {
            //select how leaf's are concated
            root = hash_and_concat(proof_step, root);
        } else {
            root = hash_and_concat(root, proof_step);
        }
    }
    Ok(root)
}

pub fn verify_merkle_multiproof(
    leaves: &[H256],
    proof: &[H256],
    indices: &[usize],
    root: H256,
) -> Result<bool, MerkleProofError> {
    match calculate_multi_merkle_root(leaves, proof, indices) {
        Ok(calculated_root) => Ok(calculated_root == root),
        Err(err) => Err(err),
    }
}

fn calculate_multi_merkle_root(
    leaves: &[H256],
    proof: &[H256],
    indices: &[usize],
) -> Result<H256, MerkleProofError> {
    let mut index_leave_map = HashMap::new();
    let mut helper_proof_map = HashMap::new();

    if leaves.len() != indices.len() {
        return Err(MerkleProofError::InvalidParamLength {
            len_first: leaves.len(),
            len_second: indices.len(),
        });
    }

    let helper_indices = get_helper_indices(indices);

    for (index, leave) in indices.iter().zip(leaves.iter()) {
        index_leave_map.insert(*index, *leave);
    }

    for (helper_step, proof_step) in helper_indices.iter().zip(proof.iter()) {
        helper_proof_map.insert(*helper_step, *proof_step);
    }

    index_leave_map.extend(helper_proof_map);

    let mut keys: Vec<usize> = vec![];

    for key in index_leave_map.keys() {
        keys.push(key.clone());
    }

    keys.sort();
    keys = reverse_vector(&keys);
    let mut biggest: usize = *keys.get(0_usize).clone().expect("No keys");

    while biggest > 0 {
        if !keys.contains(&biggest) {
            keys.push(biggest);
        }
        biggest -= 1;
    }

    keys.sort();
    keys = reverse_vector(&keys);

    let mut position = 1_usize;

    while position < keys.len() {
        // Safe because keys vector is filled above.
        let k = keys[position];
        let contains_itself: bool = index_leave_map.contains_key(&k);
        let contains_sibling: bool = index_leave_map.contains_key(&(k ^ 1));
        let contains_parent: bool = index_leave_map.contains_key(&(k / 2));

        if contains_itself && contains_sibling && !contains_parent {
            let index_first: usize = (k | 1) ^ 1; //right
            let index_second: usize = k | 1; //left

            index_leave_map.insert(
                k / 2,
                hash_and_concat(
                    index_leave_map[&index_first],
                    index_leave_map[&index_second],
                ),
            );
        }
        position += 1;
    }

    // Safe because keys vector is full and value is inserted in those indeces.
    // index_leave_map.remove(&1usize);
    Ok(index_leave_map[&1_usize])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_merkle_root_test() {
        assert_eq!(test_get_merkle_root(0, 0, z(0)), true);
        assert_eq!(test_get_merkle_root(0, 1, z(0)), true);
        assert_eq!(test_get_merkle_root(1, 1, e(0)), true);
        assert_eq!(test_get_merkle_root(0, 2, h(z(0), z(0))), true);
        assert_eq!(test_get_merkle_root(1, 2, h(e(0), z(0))), true);
        assert_eq!(test_get_merkle_root(2, 2, h(e(0), e(1))), true);
        assert_eq!(test_get_merkle_root(0, 4, h(h(z(0), z(0)), z(1))), true);
        assert_eq!(test_get_merkle_root(1, 4, h(h(e(0), z(0)), z(1))), true);
        assert_eq!(test_get_merkle_root(2, 4, h(h(e(0), e(1)), z(1))), true);
        assert_eq!(
            test_get_merkle_root(3, 4, h(h(e(0), e(1)), h(e(2), z(0)))),
            true
        );
        assert_eq!(
            test_get_merkle_root(4, 4, h(h(e(0), e(1)), h(e(2), e(3)))),
            true
        );
        assert_eq!(
            test_get_merkle_root(0, 8, h(h(h(z(0), z(0)), z(1)), z(2))),
            true
        );
        assert_eq!(
            test_get_merkle_root(1, 8, h(h(h(e(0), z(0)), z(1)), z(2))),
            true
        );
    }

    #[test]
    fn test_get_merkle_root_test_2() {
        assert_eq!(
            test_get_merkle_root(4, 16, h(h(h(h(e(0), e(1)), h(e(2), e(3))), z(2)), z(3))),
            true
        );
        assert_eq!(
            test_get_merkle_root(
                5,
                16,
                h(
                    h(h(h(e(0), e(1)), h(e(2), e(3))), h(h(e(4), z(0)), z(1))),
                    z(3)
                )
            ),
            true
        );
        assert_eq!(
            test_get_merkle_root(
                6,
                16,
                h(
                    h(
                        h(h(e(0), e(1)), h(e(2), e(3))),
                        h(h(e(4), e(5)), h(z(0), z(0)))
                    ),
                    z(3)
                )
            ),
            true
        );
        assert_eq!(
            test_get_merkle_root(
                7,
                16,
                h(
                    h(
                        h(h(e(0), e(1)), h(e(2), e(3))),
                        h(h(e(4), e(5)), h(e(6), z(0)))
                    ),
                    z(3)
                )
            ),
            true
        );
        assert_eq!(
            test_get_merkle_root(
                8,
                16,
                h(
                    h(
                        h(h(e(0), e(1)), h(e(2), e(3))),
                        h(h(e(4), e(5)), h(e(6), e(7)))
                    ),
                    z(3)
                )
            ),
            true
        );
        assert_eq!(
            test_get_merkle_root(
                9,
                16,
                h(
                    h(
                        h(h(e(0), e(1)), h(e(2), e(3))),
                        h(h(e(4), e(5)), h(e(6), e(7)))
                    ),
                    h(h(h(e(8), z(0)), z(1)), z(2))
                )
            ),
            true
        );
    }

    #[test]
    fn test_get_merkle_root_test_3() {
        assert_eq!(
            test_get_merkle_root(2, 8, h(h(h(e(0), e(1)), z(1)), z(2))),
            true
        );
        assert_eq!(
            test_get_merkle_root(3, 8, h(h(h(e(0), e(1)), h(e(2), z(0))), z(2))),
            true
        );
        assert_eq!(
            test_get_merkle_root(4, 8, h(h(h(e(0), e(1)), h(e(2), e(3))), z(2))),
            true
        );
        assert_eq!(
            test_get_merkle_root(
                5,
                8,
                h(h(h(e(0), e(1)), h(e(2), e(3))), h(h(e(4), z(0)), z(1)))
            ),
            true
        );
        assert_eq!(
            test_get_merkle_root(
                6,
                8,
                h(
                    h(h(e(0), e(1)), h(e(2), e(3))),
                    h(h(e(4), e(5)), h(z(0), z(0)))
                )
            ),
            true
        );
        assert_eq!(
            test_get_merkle_root(
                7,
                8,
                h(
                    h(h(e(0), e(1)), h(e(2), e(3))),
                    h(h(e(4), e(5)), h(e(6), z(0)))
                )
            ),
            true
        );
        assert_eq!(
            test_get_merkle_root(
                8,
                8,
                h(
                    h(h(e(0), e(1)), h(e(2), e(3))),
                    h(h(e(4), e(5)), h(e(6), e(7)))
                )
            ),
            true
        );
        assert_eq!(
            test_get_merkle_root(0, 16, h(h(h(h(z(0), z(0)), z(1)), z(2)), z(3))),
            true
        );
        assert_eq!(
            test_get_merkle_root(1, 16, h(h(h(h(e(0), z(0)), z(1)), z(2)), z(3))),
            true
        );
        assert_eq!(
            test_get_merkle_root(2, 16, h(h(h(h(e(0), e(1)), z(1)), z(2)), z(3))),
            true
        );
        assert_eq!(
            test_get_merkle_root(3, 16, h(h(h(h(e(0), e(1)), h(e(2), z(0))), z(2)), z(3))),
            true
        );
    }

    #[test]
    fn get_previous_power_of_two_test() {
        let x: usize = 3;
        assert_eq!(get_previous_power_of_two(x), 2);
    }

    #[test]
    fn get_next_power_of_two_test() {
        let x: usize = 3;
        assert_eq!(get_next_power_of_two(x), 4);
    }

    #[test]
    fn get_generalized_index_length_test() {
        assert_eq!(get_generalized_index_length(4), 2);
        assert_eq!(get_generalized_index_length(7), 2);
        assert_eq!(get_generalized_index_length(9), 3);
    }

    #[test]
    fn get_generalized_index_bit_test() {
        assert_eq!(true, get_generalized_index_bit(2_usize, 1_usize));
        assert_eq!(false, get_generalized_index_bit(3, 2));
    }

    #[test]
    fn generalized_index_sibling_test() {
        assert_eq!(generalized_index_sibling(3), 2);
    }

    #[test]
    fn generalized_index_child_test() {
        assert_ne!(generalized_index_child(3, false), 7);
        assert_eq!(generalized_index_child(5, true), 11);
    }

    #[test]
    fn get_branch_indices_test() {
        assert_eq!(get_branch_indices(5_usize), vec!(4_usize, 3_usize, 0_usize));
        assert_eq!(
            get_branch_indices(9_usize),
            vec!(8_usize, 5_usize, 3_usize, 0_usize)
        );
    }

    #[test]
    fn get_path_indices_test() {
        assert_eq!(
            get_path_indices(9_usize),
            vec!(9_usize, 4_usize, 2_usize, 1_usize)
        );
        assert_eq!(
            get_path_indices(10_usize),
            vec!(10_usize, 5_usize, 2_usize, 1_usize)
        );
    }

    #[test]
    fn get_helper_indices_test() {
        assert_eq!(
            get_helper_indices(&[9_usize, 4_usize, 2_usize, 1_usize]),
            vec!(8_usize, 5_usize, 3_usize, 0_usize)
        );
        assert_eq!(
            get_helper_indices(&[10_usize, 5_usize, 2_usize, 1_usize]),
            vec!(11_usize, 4_usize, 3_usize, 0_usize)
        );
    }

    #[test]
    fn verify_merkle_proof_test() {
        let fourth = H256::random();
        let fifth = H256::random();
        let sixth = H256::random();
        let seventh = H256::random();

        let third = hash_and_concat(fourth, fifth);
        let second = hash_and_concat(sixth, seventh);

        let root = hash_and_concat(third, second);

        assert_eq!(
            verify_merkle_proof(fourth, &[fifth, second], 0, 4, root)
                .expect("verification failed!"),
            true
        );

        assert_eq!(
            verify_merkle_proof(fifth, &[fourth, second], 0, 5, second),
            Ok(false)
        );

        assert_eq!(
            verify_merkle_proof(fifth, &[fifth, fourth, second], 0, 5, second),
            Err(MerkleProofError::InvalidParamLength {
                len_first: 3,
                len_second: 2
            })
        );

        assert_eq!(
            verify_merkle_proof(fifth, &[fifth], 0, 5, second),
            Err(MerkleProofError::InvalidParamLength {
                len_first: 1,
                len_second: 2
            })
        );

        assert_eq!(
            verify_merkle_proof(fourth, &[second, fifth], 0, 4, root)
                .expect("verification failed!"),
            false
        );

        assert_eq!(
            verify_merkle_proof(fifth, &[fourth, second], 0, 5, root)
                .expect("verification failed!"),
            true
        );

        assert_eq!(
            verify_merkle_proof(sixth, &[seventh, third], 0, 6, root)
                .expect("verification failed!"),
            true
        );

        assert_eq!(
            verify_merkle_proof(seventh, &[sixth, third], 0, 7, root)
                .expect("verification failed!"),
            true
        );

        assert_eq!(
            verify_merkle_proof(seventh, &[sixth], 0, 3, second).expect("verification failed!"),
            true
        );

        assert_eq!(
            verify_merkle_proof(fifth, &[], 0, 1, root).expect("verification failed!"),
            false
        );

        assert_eq!(
            verify_merkle_proof(fifth, &[second, fourth], 0, 5, root)
                .expect("verification failed!"),
            false
        );

        assert_eq!(
            verify_merkle_proof(fifth, &[fourth], 0, 2, root).expect("verification failed!"),
            false
        );

        assert_eq!(
            verify_merkle_proof(fifth, &[fourth, second], 0, 4, root)
                .expect("verification failed!"),
            false
        );

        assert_eq!(
            verify_merkle_proof(fifth, &[fourth, second], 0, 5, second)
                .expect("verification failed!"),
            false
        );
    }

    #[test]
    fn verify_merkle_multiproof_first_test() {
        let fourth = H256::random();
        let fifth = H256::random();
        let sixth = H256::random();
        let seventh = H256::random();

        let third = hash_and_concat(fourth, fifth);
        let second = hash_and_concat(sixth, seventh);

        let root = hash_and_concat(third, second);

        assert_eq!(
            verify_merkle_multiproof(
                &[fourth, fifth, seventh],
                &[sixth, second],
                &[4, 5, 7],
                root
            )
            .expect("verification of multiproof failed!"),
            true
        );

        assert_eq!(
            verify_merkle_multiproof(&[fourth, fifth, sixth, seventh], &[], &[4, 5, 6, 7], root)
                .expect("verification of multiproof failed!"),
            true
        );

        assert_eq!(
            verify_merkle_multiproof(&[fourth, fifth, sixth], &[sixth, second], &[4, 5, 7], root)
                .expect("verification of multiproof failed!"),
            false
        );

        assert_eq!(
            verify_merkle_multiproof(
                &[fourth, sixth, fifth],
                &[seventh, second],
                &[4, 5, 6],
                root
            )
            .expect("verification of multiproof failed!"),
            false
        );

        assert_eq!(
            verify_merkle_multiproof(
                &[fourth, fifth, sixth],
                &[seventh, second],
                &[4, 5, 6],
                root
            )
            .expect("verification of multiproof failed!"),
            true
        );

        assert_eq!(
            verify_merkle_multiproof(&[fourth, fifth], &[second, second], &[4, 5], root)
                .expect("verification of multiproof failed!"),
            true
        );

        assert_eq!(
            verify_merkle_multiproof(&[fourth], &[fifth, second], &[4], root)
                .expect("verification of multiproof failed!"),
            true
        );

        assert_eq!(
            verify_merkle_multiproof(&[fifth], &[fourth, second], &[5], root)
                .expect("verification of multiproof failed!"),
            true
        );
    }

    #[test]
    fn verify_merkle_multiproof_second_test() {
        let fourth = H256::random();
        let fifth = H256::random();
        let sixth = H256::random();
        let seventh = H256::random();

        let third = hash_and_concat(fourth, fifth);
        let second = hash_and_concat(sixth, seventh);

        let root = hash_and_concat(third, second);

        assert_eq!(
            verify_merkle_multiproof(&[sixth], &[seventh, third], &[6], root)
                .expect("verification of multiproof failed!"),
            true
        );

        assert_eq!(
            verify_merkle_multiproof(&[seventh], &[sixth, third], &[7], root)
                .expect("verification of multiproof failed!"),
            true
        );

        assert_eq!(
            verify_merkle_multiproof(&[seventh], &[sixth], &[3], second)
                .expect("verification of multiproof failed!"),
            true
        );

        assert_eq!(
            verify_merkle_multiproof(&[fifth], &[], &[1], root)
                .expect("verification of multiproof failed!"),
            false
        );

        assert_eq!(
            verify_merkle_multiproof(&[fifth], &[second, fourth], &[5], root)
                .expect("verification of multiproof failed!"),
            false
        );

        assert_eq!(
            verify_merkle_multiproof(&[fifth], &[fourth], &[2], root)
                .expect("verification of multiproof failed!"),
            false
        );

        assert_eq!(
            verify_merkle_multiproof(&[fifth], &[fourth, second], &[4], root)
                .expect("verification of multiproof failed!"),
            false
        );

        assert_eq!(
            verify_merkle_multiproof(&[fifth], &[fourth, second], &[5], second)
                .expect("verification of multiproof failed!"),
            false
        );

        assert_eq!(
            verify_merkle_multiproof(&[fifth, third], &[fourth, second], &[5], second),
            Err(MerkleProofError::InvalidParamLength {
                len_first: 2,
                len_second: 1
            })
        );

        assert_eq!(
            verify_merkle_multiproof(&[seventh, sixth], &[third], &[7, 6], root),
            Ok(true)
        );
    }

    #[test]
    fn verify_merkle_proof_bigger_test() {
        let eighth = H256::random();
        let ninth = H256::random();
        let tenth = H256::random();
        let eleventh = H256::random();

        let fourth = hash_and_concat(eighth, ninth);
        let fifth = hash_and_concat(tenth, eleventh);

        let twelfth = H256::random();
        let thirteenth = H256::random();
        let fourteenth = H256::random();
        let fifteenth = H256::random();

        let sixth = hash_and_concat(twelfth, thirteenth);
        let seventh = hash_and_concat(fourteenth, fifteenth);

        let second = hash_and_concat(fourth, fifth);
        let third = hash_and_concat(sixth, seventh);

        let root = hash_and_concat(second, third);

        assert_eq!(
            get_path_indices(15_usize),
            vec!(15_usize, 7_usize, 3_usize, 1_usize)
        );

        assert_eq!(
            verify_merkle_proof(eighth, &[ninth, fifth, third], 0, 8, root),
            Ok(true)
        );

        assert_eq!(
            verify_merkle_proof(eighth, &[ninth, fifth, third], 0, 9, root),
            Ok(false)
        );

        assert_eq!(
            verify_merkle_proof(eighth, &[ninth, fifth, third, fourth], 0, 9, root),
            Err(MerkleProofError::InvalidParamLength {
                len_first: 4,
                len_second: 3
            })
        );
    }
}
