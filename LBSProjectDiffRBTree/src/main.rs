#![allow(non_snake_case)]
use arbitrary::*;
use std::fmt::Debug;

#[derive(Arbitrary, Debug)]
enum Call<V: Ord + Debug> {
    Insert { val: V },
    Remove { val: V },
    Get { val: V },
}

fn main() -> Result<()> {
    let keyRange = 32u8; // Keys will range from 0..<keyRange
    let mut iterations = 0;
    loop {
        if iterations % 1000 == 0 {
            println!("Iteration number: {}", iterations);
        }
        let buffer = generate_random_buffer(65536); // 2^16 - Should be plenty to fill up to 1.5k long vecs of Call<u8> + getting a num.
        let mut u = Unstructured::new(&buffer);

        let number_of_calls = usize::arbitrary(&mut u)? % 1500;
        let mut calls = Vec::<Call<u8>>::with_capacity(number_of_calls);
        for _ in 0..=number_of_calls {
            calls.push(Call::<u8>::arbitrary(&mut u)?);
        }

        let mut RBTree_1 = rb_tree::RBTree::new();
        let mut RBTree_2 = rbtset::RBTreeSet::new();

        for c in calls {
            match c {
                Call::Insert { val } => {
                    let val = val % keyRange;
                    let ins_res_1_bool = RBTree_1.insert(val); //returns bool, false if val is already in the tree, true if val is not already in the tree
                    let ins_res_2 = RBTree_2.insert(val); //returns optional, None if val is already in the true, Some(node(val)) otherwise
                    let ins_res_2_bool = if ins_res_2 == None { false } else { true };
                    assert_eq!(ins_res_1_bool, ins_res_2_bool);
                    assert_eq!(
                        RBTree_1.len(), //returns usize
                        RBTree_2.len()  //returns usize
                    );
                }
                Call::Remove { val } => {
                    let val = val % keyRange;
                    assert_eq!(
                        RBTree_1.remove(&val), // returns bool
                        RBTree_2.remove(&val)  // returns bool
                    );
                    assert_eq!(
                        RBTree_1.len(), //returns usize
                        RBTree_2.len()  //returns usize
                    );
                }
                Call::Get { val } => {
                    let val = val % keyRange;
                    let ins_res_1 = RBTree_1.get(&val); // returns optional
                    let ins_res_2 = RBTree_2.get(&val); // returns optional
                    let ins_res_1_bool = if ins_res_1 == None { false } else { true };
                    let ins_res_2_bool = if ins_res_2 == None { false } else { true };
                    assert_eq!(ins_res_1_bool, ins_res_2_bool);
                    assert_eq!(
                        RBTree_1.len(), //returns usize
                        RBTree_2.len()  //returns usize
                    );
                }
            }

            let pointer_vec = RBTree_1.iter().collect::<Vec<&u8>>();
            let mut non_pointer_vec = Vec::new();
            for p in pointer_vec {
                non_pointer_vec.push(*p);
            }
            assert_eq!(non_pointer_vec, RBTree_2.values().collect::<Vec<u8>>());
        }
        iterations += 1;
    }
}

pub fn generate_random_buffer(size: usize) -> Vec<u8> {
    let mut rng = urandom::new();
    let mut buff = vec![0u8; size];
    //println!("{:?}", buff);
    rng.fill_bytes(&mut buff);
    //println!("{:?}", buff);
    return buff;
}
