use crate::hasher::Hasher;

pub struct MerkleTree {
    pub depth: usize,
    nodes: Vec<[u8; 32]>,
}


impl MerkleTree {
    pub fn create_from_leaves(leaves: &[[u8; 32]]) -> Self {
        let n = leaves.len().next_power_of_two();
        let depth = n.trailing_zeros() as usize;

        let mut nodes = vec![[0u8; 32]; 2 * n];
        
        for i in 0..n {
            nodes[n + i] = if i < leaves.len() {
                leaves[i]
            } else {
                leaves[leaves.len() - 1]
            };
        }

        for i in (1..n).rev() {
            let left = nodes[2 * i];
            let right = nodes[2 * i + 1];

            nodes[i] = Hasher::combine(&left, &right);
        }

        MerkleTree { depth, nodes }
    }

    pub fn get_root(&self) -> [u8; 32] {
        self.nodes[1]
    }

}