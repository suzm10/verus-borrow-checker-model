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
            pub owner_map: Map<String, bool>,

            #[sharding(map)]
            pub imm_map: Map<String, int>, //immutable refs

            #[sharding(map)]
            pub mut_map: Map<String, int>, // mutable refs
        }

        #[inductive(initialize)]
        fn initialize_inductive(post: Self) { }
       
        #[inductive(add_new_owner)]
        fn add_new_owner_inductive(pre: Self, post: Self, owner: String, mutability: bool) { }
       
        #[inductive(add_imm_ref)]
        fn add_imm_ref_inductive(pre: Self, post: Self, owner: String) { }
       
        #[inductive(add_mut_ref)]
        fn add_mut_ref_inductive(pre: Self, post: Self, owner: String) { }
       
        #[inductive(drop_imm_ref)]
        fn drop_imm_ref_inductive(pre: Self, post: Self, owner: String) { }

        #[inductive(drop_mut_ref)]
        fn drop_mut_ref_inductive(pre: Self, post: Self, owner: String) { }

        #[invariant]
        pub fn aliasing_xor_mutability(self) -> bool {
            forall |owner: String|
                self.owner_map.dom().contains(owner) ==>
                    (self.mut_map[owner] > 0 ==> self.imm_map[owner] == 0)
        }

        #[invariant]
        pub fn non_zero_imm_refs(self) -> bool {
            forall |owner: String|
              self.imm_map.dom().contains(owner) ==> 
                (self.imm_map[owner] >= 0)
        }

        #[invariant]
        pub fn single_mutable_refs(self) -> bool {
            forall |owner: String|
              self.mut_map.dom().contains(owner) ==> 
                (self.mut_map[owner] == 0 || self.mut_map[owner] == 1)
        }

        #[invariant]
        pub fn synchronicity(self) -> bool {
            forall |owner: String|
                self.owner_map.dom().contains(owner) <==> 
                (self.imm_map.dom().contains(owner) && 
                 self.mut_map.dom().contains(owner))
        }

        init!{
            initialize() {
                init owner_map = Map::empty();
                init imm_map = Map::empty();
                init mut_map = Map::empty();
            }
        }

        transition!{
            add_new_owner(owner: String, mutability: bool) {
                remove owner_map -= [owner => let _];
                add owner_map += [owner => mutability];

                remove mut_map -= [owner => let _];
                add mut_map += [owner => 0];

                remove imm_map -= [owner => let _];
                add imm_map += [owner => 0];
            }
        }

        transition!{
            add_imm_ref(owner: String) {
                // owner exists in owner map
                have owner_map >= [owner => let _];
                
                // no mutable refs exist (mut_map[owner] == 0)
                have mut_map >= [owner => 0];

                remove imm_map -= [owner => let curr];
                add imm_map += [owner => (curr + 1)];
            }
        }

        transition!{
            add_mut_ref(owner: String) {
                // owner exists in owner map
                have owner_map >= [owner => let _];

                // no immutable refs exist (imm_map[owner] == 0)
                have imm_map >= [owner => 0];
                
                // only add a mut ref for an owner with not mutable refs
                remove mut_map -= [owner => 0];
                add mut_map += [owner => 1];
            }
        }

        transition!{
            drop_imm_ref(owner: String) {
                have owner_map >= [owner => let _];

                remove imm_map -= [owner => let curr];
                require(curr > 0);
                add imm_map += [owner => (curr - 1)];
            }
        }

        transition!{
            drop_mut_ref(owner: String) {
                have owner_map >= [owner => let _];
                remove mut_map -= [owner => 1];
                add mut_map += [owner => 0];
            }
        }
    }
}

fn main() {}
}