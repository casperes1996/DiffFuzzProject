#![allow(non_snake_case)]
#![no_main]
use libfuzzer_sys::fuzz_target;
use core::fmt::Debug;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
enum Call<V: Ord + Debug> {
    Insert { val: V },
    Remove { val: V },
    Get { val: V },
}

fuzz_target!(|calls: Vec<Call<u8>>| {
    let keyRange = 32u8; // Keys will range from 0..<keyRange
    let mut RBTree_1 = rb_tree::RBTree::new();
    let mut RBTree_2 = rbtset::RBTreeSet::new();
    println!("Calls: {:?}", calls);
    println!("\n{:?}\n", calls.len());
   
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
});