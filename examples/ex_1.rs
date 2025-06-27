use merkle_blake3::{ MerkleTree, Hasher };



pub fn main() {
    let x = vec![b"rollup_state_X", b"rollup_state_Y", b"rollup_state_Z"];

    let leaves = x.iter().map(|x| Hasher::hash(x.as_ref())).collect::<Vec<_>>();

    let merkle_tree = MerkleTree::create_from_leaves(&leaves);

    println!("raw root: {:?}", merkle_tree.get_root());
    println!("hex root: {:?}", hex::encode(merkle_tree.get_root()));
}