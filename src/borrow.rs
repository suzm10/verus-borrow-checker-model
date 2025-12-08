use verus_builtin::*;
use verus_builtin_macros::*;
use verus_state_machines_macros::tokenized_state_machine;
use std::sync::Arc;
use vstd::atomic_ghost::*;
use vstd::modes::*;
use vstd::prelude::*;
use vstd::thread::*;
use vstd::{pervasive::*, *};

// mutable = true
// immutable = false

verus! {

tokenized_state_machine!{
    X {
        fields {
            #[sharding(map)]
            pub data_map: Map<String, bool>,

            #[sharding(map)]
            pub imm_map: Map<String, int>, //immutable refs

            #[sharding(map)]
            pub mut_map: Map<String, int>, // mutable refs

            #[sharding(map)]
            pub ref_map: Map<String, String>, // mutable refs
        }

        #[inductive(initialize)]
        fn initialize_inductive(post: Self) { }
       
        #[inductive(add_new_data)]
        fn add_new_data_inductive(pre: Self, post: Self, data: String, mutability: bool) { }
       
        #[inductive(add_imm_ref)]
        fn add_imm_ref_inductive(pre: Self, post: Self, reference: String, data: String) { }
       
        #[inductive(add_mut_ref)]
        fn add_mut_ref_inductive(pre: Self, post: Self, reference: String, data: String) { }
       
        #[inductive(drop_imm_ref)]
        fn drop_imm_ref_inductive(pre: Self, post: Self, reference: String) { }

        #[inductive(drop_mut_ref)]
        fn drop_mut_ref_inductive(pre: Self, post: Self, reference: String) { }
        
        #[inductive(mutate_referent)]
        fn mutate_referent_inductive(pre: Self, post: Self, data: String) { }

        #[invariant]
        pub fn exclusion_principle(self) -> bool {
            forall |data: String|
                self.data_map.dom().contains(data) ==>
                    (self.mut_map[data] == 1 ==> self.imm_map[data] == 0)
        }

        #[invariant]
        pub fn exclusion_principle_reversed(self) -> bool {
            forall |data: String|
                self.data_map.dom().contains(data) ==>
                    (self.imm_map[data] > 0 ==> self.mut_map[data] == 0)
        }

        #[invariant]
        pub fn non_zero_imm_refs(self) -> bool {
            forall |data: String|
              self.imm_map.dom().contains(data) ==> 
                (self.imm_map[data] >= 0)
        }

        #[invariant]
        pub fn single_mutable_refs(self) -> bool {
            forall |data: String|
              self.mut_map.dom().contains(data) ==> 
                (self.mut_map[data] == 0 || self.mut_map[data] == 1)
        }

        init!{
            initialize() {
                init data_map = Map::empty();
                init imm_map = Map::empty();
                init mut_map = Map::empty();
                init ref_map = Map::empty();
            }
        }

        transition!{
            add_new_data(data: String, mutability: bool) {
                remove data_map -= [data => let _];
                add data_map += [data => mutability];

                remove mut_map -= [data => let _];
                add mut_map += [data => 0];

                remove imm_map -= [data => let _];
                add imm_map += [data => 0];
            }
        }

        transition!{
            add_imm_ref(reference: String, data: String) {
                // reference not already in data_map
                remove data_map -= [reference => let _];
                // reference not already in ref_map
                remove ref_map -= [reference => let _];
                // reference not already in imm_map
                remove imm_map -= [data => let curr];
                
                // data exists in data map. you're not referencing something non-existant
                have data_map >= [data => let _];
                // not mutable references to data
                have mut_map >= [data => 0];
                
                // add immutable refrence to data map
                add data_map += [reference => false];
                // add mapping to ref_map
                add ref_map += [reference => data];
                // increment number of immutable refs for data
                add imm_map += [data => (curr + 1)];

                // // data exists in data map
                // have data_map >= [data => let _];
                
                // // no mutable refs exist (mut_map[data] == 0)
                // have mut_map >= [data => 0];

                // remove data_map -= [reference => let _];
                // add data_map += [reference => false];

                // remove ref_map -= [reference => let _];
                // add ref_map += [reference => data];

                // remove imm_map -= [data => let curr];
                // add imm_map += [data => (curr + 1)];
            }
        }

        transition!{
            add_mut_ref(reference: String, data: String) {
                // data exists in data map
                remove data_map -= [reference => let _];
                remove ref_map -= [reference => let _];
                remove mut_map -= [data => let curr];
                // only add mut ref for data with no mut refs currently
                require(curr == 0);

                have data_map >= [data => true];
                // no imm refs exist (imm_map[data] == 0)
                have imm_map >= [data => 0];

                add data_map += [reference => true];
                add ref_map += [reference => data];
                add mut_map += [data => 1];
            }
        }

        transition!{
            drop_imm_ref(reference: String) {
                remove ref_map -= [reference => let data];
                // require that data is a key in the map
                have data_map >= [data => let _];
                
                // if you try to remove an imm ref with curr == 0, it just removes it
                remove imm_map -= [data => let curr];
                require(curr > 0);
                add imm_map += [data => (curr - 1)];
            }
        }

        transition!{
            drop_mut_ref(reference: String) {
                remove ref_map -= [reference => let data];

                have data_map >= [data => let mutability];
                require(mutability == true);
                remove mut_map -= [data => 1];
                add mut_map += [data => 0];
            }
        }

        transition!{
            mutate_referent(data: String) {
                // enforce stack principle
                have data_map >= [data => let mutability];
                require(mutability == true);
                have imm_map >= [data => 0];
                have mut_map >= [data => 0];
            }
        }
    }
}

proof fn main_model() {
    // let tracked (Tracked(instance), Tracked(data_map), Tracked(imm_map), Tracked(mut_map)) =
    //     X::Instance::initialize();
    
    // // // let mut x = 7;
    // let tracked (Tracked(data_token), Tracked(imm_token_init), Tracked(mut_token_init)) = 
    //     instance.add_new_data(
    //         "x".to_string(),
    //         true,
    //         data_map.map(),
    //         imm_map.map(),
    //         mut_map.map()
    //     );
}


fn main() {}
}