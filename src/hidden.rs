#![feature(prelude_import)]

// mutable = true
// immutable = false



//immutable refs

// mutable refs















// owner exists in owner map

// no mutable refs exist (mut_map[owner] == 0)


// owner exists in owner map

// no immutable refs exist (imm_map[owner] == 0)

// only add a mut ref for an owner with not mutable refs


// if you try to remove an imm ref with counter == 0, it just removes it


// Initialize the state machine

// let mut x = 7;
// add_new_owner takes individual tokens (owner_map, imm_map, mut_map) and returns individual tokens

// instance.add_new_owner("x".to_string(), true, &owner_map, &imm_map, &mut_map);

// let tracked sm = X::Instance::initialize();

// // // model the owner “x”
// let tracked (
//     Tracked(owner_token),
//     Tracked(mut_token),
//     Tracked(imm_token),
//     _
// ) = sm.add_new_owner("x".to_string(), true);

// // model r1 = &x
// let ghost sm = X::add_imm_ref(sm, "x".to_string());

// // model r2 = &x
// let ghost sm = X::add_imm_ref(sm, "x".to_string());

// // immutable refs go out of scope before mutable ones:
// let ghost sm = X::drop_imm_ref(sm, "x".to_string());
// let ghost sm = X::drop_imm_ref(sm, "x".to_string());

// // model r3 = &mut x
// let ghost sm = X::add_mut_ref(sm, "x".to_string());
// let ghost sm = X::drop_mut_ref(sm, "x".to_string());

// // model r4 = &mut x
// let ghost sm = X::add_mut_ref(sm, "x".to_string());
// let ghost sm = X::drop_mut_ref(sm, "x".to_string());



#![allow(internal_features)]
#![feature(stmt_expr_attributes)]
#![feature(box_patterns)]
#![feature(negative_impls)]
#![feature(rustc_attrs)]
#![feature(unboxed_closures)]
#![feature(register_tool)]
#![feature(tuple_trait)]
#![feature(custom_inner_attributes)]
#![feature(try_trait_v2)]
#![register_tool(verus)]
#![register_tool(verifier)]
#![register_tool(verusfmt)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use verus_builtin::*;
use verus_builtin_macros::*;
use verus_state_machines_macros::tokenized_state_machine;
use std::sync::Arc;
use vstd::atomic_ghost::*;
use vstd::modes::*;
use vstd::prelude::*;
use vstd::thread::*;
use vstd::{pervasive::*, *};
#[allow(unused_parens)]
pub mod State {
    use super::*;
    use ::vstd::tokens::ValueToken;
    use ::vstd::tokens::KeyValueToken;
    use ::vstd::tokens::CountToken;
    use ::vstd::tokens::MonotonicCountToken;
    use ::vstd::tokens::ElementToken;
    use ::vstd::tokens::SimpleToken;
    #[verus::internal(verus_macro)]
    #[verifier::ext_equal]
    pub struct State {
        pub owner_map: ::vstd::map::Map<String, bool>,
        pub imm_map: ::vstd::map::Map<String, int>,
        pub mut_map: ::vstd::map::Map<String, int>,
    }
    #[allow(non_camel_case_types)]
    #[verus::internal(verus_macro)]
    pub enum Step {
        add_new_owner(String, bool),
        add_imm_ref(String),
        add_mut_ref(String),
        drop_imm_ref(String),
        drop_mut_ref(String),
        dummy_to_use_type_params(State),
    }
    #[verus::internal(verus_macro)]
    impl Step {
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_1(self) -> bool {
            ::vstd::prelude::get_variant_field(self, "add_new_owner", "1")
        }
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_add_new_owner_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "add_new_owner", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_add_new_owner_1(self) -> bool {
            ::vstd::prelude::get_variant_field(self, "add_new_owner", "1")
        }
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_add_imm_ref_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "add_imm_ref", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_add_mut_ref_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "add_mut_ref", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_drop_imm_ref_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "drop_imm_ref", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_drop_mut_ref_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "drop_mut_ref", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_dummy_to_use_type_params_0(self) -> State {
            ::vstd::prelude::get_variant_field(self,
                "dummy_to_use_type_params", "0")
        }
    }
    #[verus::internal(verus_macro)]
    #[automatically_derived]
    impl Step {
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        pub fn is_add_new_owner(&self) -> bool {
            ::vstd::prelude::is_variant(self, "add_new_owner")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        pub fn get_add_new_owner_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "add_new_owner", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        pub fn get_add_new_owner_1(self) -> bool {
            ::vstd::prelude::get_variant_field(self, "add_new_owner", "1")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        pub fn is_add_imm_ref(&self) -> bool {
            ::vstd::prelude::is_variant(self, "add_imm_ref")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        pub fn get_add_imm_ref_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "add_imm_ref", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        pub fn is_add_mut_ref(&self) -> bool {
            ::vstd::prelude::is_variant(self, "add_mut_ref")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        pub fn get_add_mut_ref_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "add_mut_ref", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        pub fn is_drop_imm_ref(&self) -> bool {
            ::vstd::prelude::is_variant(self, "drop_imm_ref")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        pub fn get_drop_imm_ref_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "drop_imm_ref", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        pub fn is_drop_mut_ref(&self) -> bool {
            ::vstd::prelude::is_variant(self, "drop_mut_ref")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        pub fn get_drop_mut_ref_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "drop_mut_ref", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        pub fn is_dummy_to_use_type_params(&self) -> bool {
            ::vstd::prelude::is_variant(self, "dummy_to_use_type_params")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        pub fn get_dummy_to_use_type_params_0(self) -> State {
            ::vstd::prelude::get_variant_field(self,
                "dummy_to_use_type_params", "0")
        }
    }
    #[allow(non_camel_case_types)]
    #[verus::internal(verus_macro)]
    pub enum Config { initialize(), dummy_to_use_type_params(State), }
    #[verus::internal(verus_macro)]
    impl Config {
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_0(self) -> State {
            ::vstd::prelude::get_variant_field(self,
                "dummy_to_use_type_params", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_dummy_to_use_type_params_0(self) -> State {
            ::vstd::prelude::get_variant_field(self,
                "dummy_to_use_type_params", "0")
        }
    }
    #[verus::internal(verus_macro)]
    #[automatically_derived]
    impl Config {
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        pub fn is_initialize(&self) -> bool {
            ::vstd::prelude::is_variant(self, "initialize")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        pub fn is_dummy_to_use_type_params(&self) -> bool {
            ::vstd::prelude::is_variant(self, "dummy_to_use_type_params")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        pub fn get_dummy_to_use_type_params_0(self) -> State {
            ::vstd::prelude::get_variant_field(self,
                "dummy_to_use_type_params", "0")
        }
    }
    pub mod show {
        use super::*;
        #[verus::internal(verus_macro)]
        #[verifier::external_body]
        #[verifier::proof]
        pub fn add_new_owner(pre: super::State, post: super::State,
            owner: String, mutability: bool) {
            ::vstd::prelude::requires(super::State::add_new_owner(pre, post,
                    owner, mutability));
            ::vstd::prelude::ensures(super::State::next(pre, post));
        }
        use bool as add_new_owner;
        #[verus::internal(verus_macro)]
        #[verifier::external_body]
        #[verifier::proof]
        pub fn add_imm_ref(pre: super::State, post: super::State,
            owner: String) {
            ::vstd::prelude::requires(super::State::add_imm_ref(pre, post,
                    owner));
            ::vstd::prelude::ensures(super::State::next(pre, post));
        }
        use bool as add_imm_ref;
        #[verus::internal(verus_macro)]
        #[verifier::external_body]
        #[verifier::proof]
        pub fn add_mut_ref(pre: super::State, post: super::State,
            owner: String) {
            ::vstd::prelude::requires(super::State::add_mut_ref(pre, post,
                    owner));
            ::vstd::prelude::ensures(super::State::next(pre, post));
        }
        use bool as add_mut_ref;
        #[verus::internal(verus_macro)]
        #[verifier::external_body]
        #[verifier::proof]
        pub fn drop_imm_ref(pre: super::State, post: super::State,
            owner: String) {
            ::vstd::prelude::requires(super::State::drop_imm_ref(pre, post,
                    owner));
            ::vstd::prelude::ensures(super::State::next(pre, post));
        }
        use bool as drop_imm_ref;
        #[verus::internal(verus_macro)]
        #[verifier::external_body]
        #[verifier::proof]
        pub fn drop_mut_ref(pre: super::State, post: super::State,
            owner: String) {
            ::vstd::prelude::requires(super::State::drop_mut_ref(pre, post,
                    owner));
            ::vstd::prelude::ensures(super::State::next(pre, post));
        }
        use bool as drop_mut_ref;
        #[verus::internal(verus_macro)]
        #[verifier::external_body]
        #[verifier::proof]
        pub fn initialize(post: super::State) {
            ::vstd::prelude::requires(super::State::initialize(post));
            ::vstd::prelude::ensures(super::State::init(post));
        }
        use bool as initialize;
    }
    pub mod take_step {
        use super::*;
        #[verus::internal(verus_macro)]
        #[verifier::external_body]
        #[verifier::proof]
        pub fn initialize() -> super::State {
            ::vstd::prelude::requires(super::State::initialize_enabled());
            ::vstd::prelude::ensures(|post: super::State|
                    super::State::initialize(post) && post.invariant());
            ::vstd::prelude::extra_dependency(State::initialize_inductive);
            loop {}
        }
        use bool as initialize;
        #[verus::internal(verus_macro)]
        #[verifier::external_body]
        #[verifier::proof]
        pub fn add_new_owner(pre: super::State, owner: String,
            mutability: bool) -> super::State {
            ::vstd::prelude::requires(super::State::add_new_owner_enabled(pre,
                        owner, mutability) && pre.invariant());
            ::vstd::prelude::ensures(|post: super::State|
                    super::State::add_new_owner_strong(pre, post, owner,
                            mutability) && post.invariant());
            ::vstd::prelude::extra_dependency(State::add_new_owner_inductive);
            ::vstd::prelude::extra_dependency(State::add_new_owner_asserts);
            loop {}
        }
        use bool as add_new_owner;
        #[verus::internal(verus_macro)]
        #[verifier::external_body]
        #[verifier::proof]
        pub fn add_imm_ref(pre: super::State, owner: String) -> super::State {
            ::vstd::prelude::requires(super::State::add_imm_ref_enabled(pre,
                        owner) && pre.invariant());
            ::vstd::prelude::ensures(|post: super::State|
                    super::State::add_imm_ref_strong(pre, post, owner) &&
                        post.invariant());
            ::vstd::prelude::extra_dependency(State::add_imm_ref_inductive);
            ::vstd::prelude::extra_dependency(State::add_imm_ref_asserts);
            loop {}
        }
        use bool as add_imm_ref;
        #[verus::internal(verus_macro)]
        #[verifier::external_body]
        #[verifier::proof]
        pub fn add_mut_ref(pre: super::State, owner: String) -> super::State {
            ::vstd::prelude::requires(super::State::add_mut_ref_enabled(pre,
                        owner) && pre.invariant());
            ::vstd::prelude::ensures(|post: super::State|
                    super::State::add_mut_ref_strong(pre, post, owner) &&
                        post.invariant());
            ::vstd::prelude::extra_dependency(State::add_mut_ref_inductive);
            ::vstd::prelude::extra_dependency(State::add_mut_ref_asserts);
            loop {}
        }
        use bool as add_mut_ref;
        #[verus::internal(verus_macro)]
        #[verifier::external_body]
        #[verifier::proof]
        pub fn drop_imm_ref(pre: super::State, owner: String)
            -> super::State {
            ::vstd::prelude::requires(super::State::drop_imm_ref_enabled(pre,
                        owner) && pre.invariant());
            ::vstd::prelude::ensures(|post: super::State|
                    super::State::drop_imm_ref_strong(pre, post, owner) &&
                        post.invariant());
            ::vstd::prelude::extra_dependency(State::drop_imm_ref_inductive);
            ::vstd::prelude::extra_dependency(State::drop_imm_ref_asserts);
            loop {}
        }
        use bool as drop_imm_ref;
        #[verus::internal(verus_macro)]
        #[verifier::external_body]
        #[verifier::proof]
        pub fn drop_mut_ref(pre: super::State, owner: String)
            -> super::State {
            ::vstd::prelude::requires(super::State::drop_mut_ref_enabled(pre,
                        owner) && pre.invariant());
            ::vstd::prelude::ensures(|post: super::State|
                    super::State::drop_mut_ref_strong(pre, post, owner) &&
                        post.invariant());
            ::vstd::prelude::extra_dependency(State::drop_mut_ref_inductive);
            ::vstd::prelude::extra_dependency(State::drop_mut_ref_asserts);
            loop {}
        }
        use bool as drop_mut_ref;
    }
    #[verifier::proof]
    #[allow(non_camel_case_types)]
    pub struct Instance {
        #[verifier::spec]
        send_sync: ::vstd::state_machine_internal::SyncSendIfSyncSend<()>,
        #[verifier::spec]
        state: ::core::option::Option<::vstd::prelude::Ghost<State>>,
        #[verifier::spec]
        location: ::vstd::prelude::int,
    }
    #[verifier::proof]
    #[allow(non_camel_case_types)]
    pub struct owner_map {
        #[verifier::proof]
        dummy_instance: Instance,
        no_copy: ::vstd::state_machine_internal::NoCopy,
    }
    #[verus::internal(verus_macro)]
    impl owner_map { }
    #[verus::internal(verus_macro)]
    #[verus::internal(verus_macro)]
    impl ::vstd::tokens::KeyValueToken<String, bool> for owner_map {
        #[verifier::spec]
        #[verifier::external_body]
        fn instance_id(&self) -> ::vstd::tokens::InstanceId {
            ::core::panicking::panic("not implemented")
        }
        #[verifier::spec]
        #[verifier::external_body]
        fn key(&self) -> String {
            ::core::panicking::panic("not implemented")
        }
        #[verifier::spec]
        #[verifier::external_body]
        fn value(&self) -> bool {
            ::core::panicking::panic("not implemented")
        }
        #[verifier::proof]
        #[verifier::external_body]
        fn agree(#[verifier::proof] &self, #[verifier::proof] other: &Self) {
            ::core::panicking::panic("not implemented");
        }
        #[verifier::proof]
        #[verifier::external_body]
        #[verifier::returns(proof)]
        fn arbitrary() -> Self {
            ::core::panicking::panic("not implemented");
        }
    }
    #[verus::internal(verus_macro)]
    #[verus::internal(verus_macro)]
    impl ::vstd::tokens::UniqueKeyValueToken<String, bool> for owner_map {
        #[verifier::external_body]
        #[verifier::proof]
        fn unique(#[verifier::proof] &mut self,
            #[verifier::proof] other: &Self) {
            ::core::panicking::panic("not implemented");
        }
    }
    #[allow(type_alias_bounds)]
    #[allow(non_camel_case_types)]
    pub type owner_map_map =
        ::vstd::tokens::MapToken<String, bool, owner_map>;
    #[verifier::proof]
    #[allow(non_camel_case_types)]
    pub struct imm_map {
        #[verifier::proof]
        dummy_instance: Instance,
        no_copy: ::vstd::state_machine_internal::NoCopy,
    }
    #[verus::internal(verus_macro)]
    impl imm_map { }
    #[verus::internal(verus_macro)]
    #[verus::internal(verus_macro)]
    impl ::vstd::tokens::KeyValueToken<String, int> for imm_map {
        #[verifier::spec]
        #[verifier::external_body]
        fn instance_id(&self) -> ::vstd::tokens::InstanceId {
            ::core::panicking::panic("not implemented")
        }
        #[verifier::spec]
        #[verifier::external_body]
        fn key(&self) -> String {
            ::core::panicking::panic("not implemented")
        }
        #[verifier::spec]
        #[verifier::external_body]
        fn value(&self) -> int { ::core::panicking::panic("not implemented") }
        #[verifier::proof]
        #[verifier::external_body]
        fn agree(#[verifier::proof] &self, #[verifier::proof] other: &Self) {
            ::core::panicking::panic("not implemented");
        }
        #[verifier::proof]
        #[verifier::external_body]
        #[verifier::returns(proof)]
        fn arbitrary() -> Self {
            ::core::panicking::panic("not implemented");
        }
    }
    #[verus::internal(verus_macro)]
    #[verus::internal(verus_macro)]
    impl ::vstd::tokens::UniqueKeyValueToken<String, int> for imm_map {
        #[verifier::external_body]
        #[verifier::proof]
        fn unique(#[verifier::proof] &mut self,
            #[verifier::proof] other: &Self) {
            ::core::panicking::panic("not implemented");
        }
    }
    #[allow(type_alias_bounds)]
    #[allow(non_camel_case_types)]
    pub type imm_map_map = ::vstd::tokens::MapToken<String, int, imm_map>;
    #[verifier::proof]
    #[allow(non_camel_case_types)]
    pub struct mut_map {
        #[verifier::proof]
        dummy_instance: Instance,
        no_copy: ::vstd::state_machine_internal::NoCopy,
    }
    #[verus::internal(verus_macro)]
    impl mut_map { }
    #[verus::internal(verus_macro)]
    #[verus::internal(verus_macro)]
    impl ::vstd::tokens::KeyValueToken<String, int> for mut_map {
        #[verifier::spec]
        #[verifier::external_body]
        fn instance_id(&self) -> ::vstd::tokens::InstanceId {
            ::core::panicking::panic("not implemented")
        }
        #[verifier::spec]
        #[verifier::external_body]
        fn key(&self) -> String {
            ::core::panicking::panic("not implemented")
        }
        #[verifier::spec]
        #[verifier::external_body]
        fn value(&self) -> int { ::core::panicking::panic("not implemented") }
        #[verifier::proof]
        #[verifier::external_body]
        fn agree(#[verifier::proof] &self, #[verifier::proof] other: &Self) {
            ::core::panicking::panic("not implemented");
        }
        #[verifier::proof]
        #[verifier::external_body]
        #[verifier::returns(proof)]
        fn arbitrary() -> Self {
            ::core::panicking::panic("not implemented");
        }
    }
    #[verus::internal(verus_macro)]
    #[verus::internal(verus_macro)]
    impl ::vstd::tokens::UniqueKeyValueToken<String, int> for mut_map {
        #[verifier::external_body]
        #[verifier::proof]
        fn unique(#[verifier::proof] &mut self,
            #[verifier::proof] other: &Self) {
            ::core::panicking::panic("not implemented");
        }
    }
    #[allow(type_alias_bounds)]
    #[allow(non_camel_case_types)]
    pub type mut_map_map = ::vstd::tokens::MapToken<String, int, mut_map>;
    #[verus::internal(verus_macro)]
    impl Instance {
        #[verus::internal(verus_macro)]
        #[verifier::proof]
        #[verifier::external_body]
        #[verifier::returns(proof)]
        pub fn clone(#[verifier::proof] &self) -> Self {
            ensures(|s: Self| ::vstd::prelude::equal(*self, s));
            ::core::panicking::panic("not implemented");
        }
        #[verifier::spec]
        #[verifier::external_body]
        pub fn id(&self) -> ::vstd::tokens::InstanceId {
            ::core::panicking::panic("not implemented")
        }
        #[verifier::external_body]
        #[verus::internal(verus_macro)]
        #[verus::internal(returns(proof))]
        #[verus::internal(proof)]
        pub fn initialize()
            ->
                (::vstd::prelude::Tracked<Instance>,
                ::vstd::prelude::Tracked<::vstd::tokens::MapToken<String,
                bool, owner_map>>,
                ::vstd::prelude::Tracked<::vstd::tokens::MapToken<String, int,
                imm_map>>,
                ::vstd::prelude::Tracked<::vstd::tokens::MapToken<String, int,
                mut_map>>) {
            ::vstd::prelude::ensures(|tmp_tuple|
                    [::vstd::prelude::constrain_type(tmp_tuple,
                                Self::initialize()),
                            ({
                                    let (instance, param_token_owner_map, param_token_imm_map,
                                            param_token_mut_map) = tmp_tuple;
                                    let instance = instance.view();
                                    let param_token_owner_map = param_token_owner_map.view();
                                    let param_token_imm_map = param_token_imm_map.view();
                                    let param_token_mut_map = param_token_mut_map.view();
                                    (::vstd::prelude::equal(Map::empty(),
                                                        (param_token_owner_map).map()) &&
                                                    ::vstd::prelude::equal((param_token_owner_map).instance_id(),
                                                        instance.id())) &&
                                            (::vstd::prelude::equal(Map::empty(),
                                                        (param_token_imm_map).map()) &&
                                                    ::vstd::prelude::equal((param_token_imm_map).instance_id(),
                                                        instance.id())) &&
                                        (::vstd::prelude::equal(Map::empty(),
                                                    (param_token_mut_map).map()) &&
                                                ::vstd::prelude::equal((param_token_mut_map).instance_id(),
                                                    instance.id()))
                                })]);
            ::vstd::prelude::extra_dependency(State::initialize_inductive);
            ::core::panicking::panic("not implemented");
        }
        #[verifier::external_body]
        #[verus::internal(verus_macro)]
        #[verus::internal(returns(proof))]
        #[verus::internal(proof)]
        pub fn add_new_owner(#[verus::internal(proof)] &self, owner: String,
            mutability: bool,
            #[verus::internal(proof)] param_token_0_owner_map: owner_map,
            #[verus::internal(proof)] param_token_4_imm_map: imm_map,
            #[verus::internal(proof)] param_token_2_mut_map: mut_map)
            ->
                (::vstd::prelude::Tracked<owner_map>,
                ::vstd::prelude::Tracked<imm_map>,
                ::vstd::prelude::Tracked<mut_map>) {
            ::vstd::prelude::requires([(::vstd::prelude::equal(param_token_0_owner_map.instance_id(),
                                (*self).id())),
                        (::vstd::prelude::equal(param_token_4_imm_map.instance_id(),
                                (*self).id())),
                        (::vstd::prelude::equal(param_token_2_mut_map.instance_id(),
                                (*self).id())),
                        (::vstd::prelude::equal(param_token_0_owner_map.key(),
                                owner)),
                        (::vstd::prelude::equal(param_token_2_mut_map.key(),
                                owner)),
                        (::vstd::prelude::equal(param_token_4_imm_map.key(),
                                owner))]);
            ::vstd::prelude::ensures(|tmp_tuple|
                    [::vstd::prelude::constrain_type(tmp_tuple,
                                self.add_new_owner(owner, mutability,
                                    param_token_0_owner_map, param_token_4_imm_map,
                                    param_token_2_mut_map)),
                            ({
                                    let (param_token_1_owner_map, param_token_5_imm_map,
                                            param_token_3_mut_map) = tmp_tuple;
                                    let param_token_1_owner_map =
                                        param_token_1_owner_map.view();
                                    let param_token_5_imm_map = param_token_5_imm_map.view();
                                    let param_token_3_mut_map = param_token_3_mut_map.view();
                                    (::vstd::prelude::equal(param_token_1_owner_map.instance_id(),
                                                                (*self).id())) &&
                                                        (::vstd::prelude::equal(param_token_5_imm_map.instance_id(),
                                                                (*self).id())) &&
                                                    (::vstd::prelude::equal(param_token_3_mut_map.instance_id(),
                                                            (*self).id())) &&
                                                (((::vstd::prelude::equal(param_token_1_owner_map.key(),
                                                                    owner)) &&
                                                            (::vstd::prelude::equal(param_token_1_owner_map.value(),
                                                                    mutability)))) &&
                                            (((::vstd::prelude::equal(param_token_3_mut_map.key(),
                                                                owner)) &&
                                                        (::vstd::prelude::equal(param_token_3_mut_map.value(),
                                                                ::vstd::prelude::spec_literal_integer("0"))))) &&
                                        (((::vstd::prelude::equal(param_token_5_imm_map.key(),
                                                            owner)) &&
                                                    (::vstd::prelude::equal(param_token_5_imm_map.value(),
                                                            ::vstd::prelude::spec_literal_integer("0")))))
                                })]);
            ::vstd::prelude::extra_dependency(State::add_new_owner_inductive);
            ::vstd::prelude::extra_dependency(State::add_new_owner_asserts);
            ::core::panicking::panic("not implemented");
        }
        #[verifier::external_body]
        #[verus::internal(verus_macro)]
        #[verus::internal(returns(proof))]
        #[verus::internal(proof)]
        pub fn add_imm_ref(#[verus::internal(proof)] &self, owner: String,
            #[verus::internal(proof)] param_token_0_owner_map: &owner_map,
            #[verus::internal(proof)] param_token_2_imm_map: imm_map,
            #[verus::internal(proof)] param_token_1_mut_map: &mut_map)
            -> imm_map {
            ::vstd::prelude::requires([(::vstd::prelude::equal(param_token_0_owner_map.instance_id(),
                                (*self).id())),
                        (::vstd::prelude::equal(param_token_2_imm_map.instance_id(),
                                (*self).id())),
                        (::vstd::prelude::equal(param_token_1_mut_map.instance_id(),
                                (*self).id())),
                        (::vstd::prelude::equal(param_token_0_owner_map.key(),
                                owner)),
                        (((::vstd::prelude::equal(param_token_1_mut_map.key(),
                                            owner)) &&
                                    (::vstd::prelude::equal(param_token_1_mut_map.value(),
                                            ::vstd::prelude::spec_literal_integer("0"))))),
                        (::vstd::prelude::equal(param_token_2_imm_map.key(),
                                owner))]);
            ::vstd::prelude::ensures(|param_token_3_imm_map|
                    [::vstd::prelude::constrain_type(param_token_3_imm_map,
                                self.add_imm_ref(owner, param_token_0_owner_map,
                                    param_token_2_imm_map, param_token_1_mut_map)),
                            (::vstd::prelude::equal(param_token_3_imm_map.instance_id(),
                                    (*self).id())),
                            ({
                                    let curr = param_token_2_imm_map.value();
                                    ((::vstd::prelude::equal(param_token_3_imm_map.key(),
                                                    owner)) &&
                                            (::vstd::prelude::equal(param_token_3_imm_map.value(),
                                                    ((curr).spec_add(::vstd::prelude::spec_literal_nat("1"))))))
                                })]);
            ::vstd::prelude::extra_dependency(State::add_imm_ref_inductive);
            ::vstd::prelude::extra_dependency(State::add_imm_ref_asserts);
            ::core::panicking::panic("not implemented");
        }
        #[verifier::external_body]
        #[verus::internal(verus_macro)]
        #[verus::internal(returns(proof))]
        #[verus::internal(proof)]
        pub fn add_mut_ref(#[verus::internal(proof)] &self, owner: String,
            #[verus::internal(proof)] param_token_0_owner_map: &owner_map,
            #[verus::internal(proof)] param_token_1_imm_map: &imm_map,
            #[verus::internal(proof)] param_token_2_mut_map: mut_map)
            -> mut_map {
            ::vstd::prelude::requires([(::vstd::prelude::equal(param_token_0_owner_map.instance_id(),
                                (*self).id())),
                        (::vstd::prelude::equal(param_token_1_imm_map.instance_id(),
                                (*self).id())),
                        (::vstd::prelude::equal(param_token_2_mut_map.instance_id(),
                                (*self).id())),
                        (::vstd::prelude::equal(param_token_0_owner_map.key(),
                                owner)),
                        (((::vstd::prelude::equal(param_token_1_imm_map.key(),
                                            owner)) &&
                                    (::vstd::prelude::equal(param_token_1_imm_map.value(),
                                            ::vstd::prelude::spec_literal_integer("0"))))),
                        (((::vstd::prelude::equal(param_token_2_mut_map.key(),
                                            owner)) &&
                                    (::vstd::prelude::equal(param_token_2_mut_map.value(),
                                            ::vstd::prelude::spec_literal_integer("0")))))]);
            ::vstd::prelude::ensures(|param_token_3_mut_map|
                    [::vstd::prelude::constrain_type(param_token_3_mut_map,
                                self.add_mut_ref(owner, param_token_0_owner_map,
                                    param_token_1_imm_map, param_token_2_mut_map)),
                            (::vstd::prelude::equal(param_token_3_mut_map.instance_id(),
                                    (*self).id())),
                            (((::vstd::prelude::equal(param_token_3_mut_map.key(),
                                                owner)) &&
                                        (::vstd::prelude::equal(param_token_3_mut_map.value(),
                                                ::vstd::prelude::spec_literal_integer("1")))))]);
            ::vstd::prelude::extra_dependency(State::add_mut_ref_inductive);
            ::vstd::prelude::extra_dependency(State::add_mut_ref_asserts);
            ::core::panicking::panic("not implemented");
        }
        #[verifier::external_body]
        #[verus::internal(verus_macro)]
        #[verus::internal(returns(proof))]
        #[verus::internal(proof)]
        pub fn drop_imm_ref(#[verus::internal(proof)] &self, owner: String,
            #[verus::internal(proof)] param_token_0_owner_map: &owner_map,
            #[verus::internal(proof)] param_token_1_imm_map: imm_map)
            -> imm_map {
            ::vstd::prelude::requires([(::vstd::prelude::equal(param_token_0_owner_map.instance_id(),
                                (*self).id())),
                        (::vstd::prelude::equal(param_token_1_imm_map.instance_id(),
                                (*self).id())),
                        (::vstd::prelude::equal(param_token_0_owner_map.key(),
                                owner)),
                        (::vstd::prelude::equal(param_token_1_imm_map.key(),
                                owner)),
                        ({
                                let curr = param_token_1_imm_map.value();
                                ((curr).spec_gt(::vstd::prelude::spec_literal_nat("0")))
                            })]);
            ::vstd::prelude::ensures(|param_token_2_imm_map|
                    [::vstd::prelude::constrain_type(param_token_2_imm_map,
                                self.drop_imm_ref(owner, param_token_0_owner_map,
                                    param_token_1_imm_map)),
                            (::vstd::prelude::equal(param_token_2_imm_map.instance_id(),
                                    (*self).id())),
                            ({
                                    let curr = param_token_1_imm_map.value();
                                    ((::vstd::prelude::equal(param_token_2_imm_map.key(),
                                                    owner)) &&
                                            (::vstd::prelude::equal(param_token_2_imm_map.value(),
                                                    ((curr).spec_sub(::vstd::prelude::spec_literal_nat("1"))))))
                                })]);
            ::vstd::prelude::extra_dependency(State::drop_imm_ref_inductive);
            ::vstd::prelude::extra_dependency(State::drop_imm_ref_asserts);
            ::core::panicking::panic("not implemented");
        }
        #[verifier::external_body]
        #[verus::internal(verus_macro)]
        #[verus::internal(returns(proof))]
        #[verus::internal(proof)]
        pub fn drop_mut_ref(#[verus::internal(proof)] &self, owner: String,
            #[verus::internal(proof)] param_token_0_owner_map: &owner_map,
            #[verus::internal(proof)] param_token_1_mut_map: mut_map)
            -> mut_map {
            ::vstd::prelude::requires([(::vstd::prelude::equal(param_token_0_owner_map.instance_id(),
                                (*self).id())),
                        (::vstd::prelude::equal(param_token_1_mut_map.instance_id(),
                                (*self).id())),
                        (::vstd::prelude::equal(param_token_0_owner_map.key(),
                                owner)),
                        (((::vstd::prelude::equal(param_token_1_mut_map.key(),
                                            owner)) &&
                                    (::vstd::prelude::equal(param_token_1_mut_map.value(),
                                            ::vstd::prelude::spec_literal_integer("1")))))]);
            ::vstd::prelude::ensures(|param_token_2_mut_map|
                    [::vstd::prelude::constrain_type(param_token_2_mut_map,
                                self.drop_mut_ref(owner, param_token_0_owner_map,
                                    param_token_1_mut_map)),
                            (::vstd::prelude::equal(param_token_2_mut_map.instance_id(),
                                    (*self).id())),
                            (((::vstd::prelude::equal(param_token_2_mut_map.key(),
                                                owner)) &&
                                        (::vstd::prelude::equal(param_token_2_mut_map.value(),
                                                ::vstd::prelude::spec_literal_integer("0")))))]);
            ::vstd::prelude::extra_dependency(State::drop_mut_ref_inductive);
            ::vstd::prelude::extra_dependency(State::drop_mut_ref_asserts);
            ::core::panicking::panic("not implemented");
        }
    }
    #[verus::internal(verus_macro)]
    #[verus::internal(verus_macro)]
    impl ::core::clone::Clone for Instance {
        #[verifier::external_body]
        fn clone(&self) -> Self {
            ::core::panicking::panic("not implemented");
        }
    }
    #[verus::internal(verus_macro)]
    #[verus::internal(verus_macro)]
    impl ::core::marker::Copy for Instance { }
    #[verus::internal(verus_macro)]
    impl State {
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        #[verus::internal(open)]
        pub fn initialize(post: Self) -> ::core::primitive::bool {
            {
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    Map::empty();
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    Map::empty();
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    Map::empty();
                (#[verifier::custom_err("cannot prove that final value of field `mut_map` has this updated value")] (::vstd::prelude::equal(post.mut_map,
                                update_tmp_mut_map)) &&
                        (#[verifier::custom_err("cannot prove that final value of field `imm_map` has this updated value")] (::vstd::prelude::equal(post.imm_map,
                                        update_tmp_imm_map)) &&
                                #[verifier::custom_err("cannot prove that final value of field `owner_map` has this updated value")] (::vstd::prelude::equal(post.owner_map,
                                        update_tmp_owner_map))))
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        #[verus::internal(open)]
        pub fn initialize_enabled() -> ::core::primitive::bool { { true } }
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        #[verus::internal(open)]
        pub fn add_new_owner(pre: Self, post: Self, owner: String,
            mutability: bool) -> ::core::primitive::bool {
            {
                let tmp_assert: ::core::primitive::bool = true;
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_owner_map).dom().contains(owner))
                        &&
                        {
                            let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                                (update_tmp_owner_map).remove(owner);
                            let tmp_assert: ::core::primitive::bool =
                                tmp_assert &&
                                    (!(update_tmp_owner_map).dom().contains(owner));
                            let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                                (update_tmp_owner_map).insert(owner, mutability);
                            (#[verifier::custom_err("cannot prove this condition holds")] ({
                                            ::vstd::prelude::imply(tmp_assert,
                                                (update_tmp_mut_map).dom().contains(owner))
                                        }) &&
                                    {
                                        let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                                            (update_tmp_mut_map).remove(owner);
                                        let tmp_assert: ::core::primitive::bool =
                                            tmp_assert && (!(update_tmp_mut_map).dom().contains(owner));
                                        let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                                            (update_tmp_mut_map).insert(owner,
                                                ::vstd::prelude::spec_literal_integer("0"));
                                        (#[verifier::custom_err("cannot prove this condition holds")] ({
                                                        ::vstd::prelude::imply(tmp_assert,
                                                            (update_tmp_imm_map).dom().contains(owner))
                                                    }) &&
                                                {
                                                    let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                                                        (update_tmp_imm_map).remove(owner);
                                                    let tmp_assert: ::core::primitive::bool =
                                                        tmp_assert && (!(update_tmp_imm_map).dom().contains(owner));
                                                    let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                                                        (update_tmp_imm_map).insert(owner,
                                                            ::vstd::prelude::spec_literal_integer("0"));
                                                    (#[verifier::custom_err("cannot prove that final value of field `imm_map` has this updated value")] ({
                                                                    ::vstd::prelude::imply(tmp_assert,
                                                                        ::vstd::prelude::equal(post.imm_map, update_tmp_imm_map))
                                                                }) &&
                                                            (#[verifier::custom_err("cannot prove that final value of field `mut_map` has this updated value")] ({
                                                                            ::vstd::prelude::imply(tmp_assert,
                                                                                ::vstd::prelude::equal(post.mut_map, update_tmp_mut_map))
                                                                        }) &&
                                                                    #[verifier::custom_err("cannot prove that final value of field `owner_map` has this updated value")] ({
                                                                            ::vstd::prelude::imply(tmp_assert,
                                                                                ::vstd::prelude::equal(post.owner_map,
                                                                                    update_tmp_owner_map))
                                                                        })))
                                                })
                                    })
                        })
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        #[verus::internal(open)]
        pub fn add_new_owner_strong(pre: Self, post: Self, owner: String,
            mutability: bool) -> ::core::primitive::bool {
            {
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_owner_map).dom().contains(owner))
                        &&
                        {
                            let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                                (update_tmp_owner_map).remove(owner);
                            (#[verifier::custom_err("cannot prove this 'assert' holds")] (!(update_tmp_owner_map).dom().contains(owner))
                                    &&
                                    {
                                        let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                                            (update_tmp_owner_map).insert(owner, mutability);
                                        (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_mut_map).dom().contains(owner))
                                                &&
                                                {
                                                    let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                                                        (update_tmp_mut_map).remove(owner);
                                                    (#[verifier::custom_err("cannot prove this 'assert' holds")] (!(update_tmp_mut_map).dom().contains(owner))
                                                            &&
                                                            {
                                                                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                                                                    (update_tmp_mut_map).insert(owner,
                                                                        ::vstd::prelude::spec_literal_integer("0"));
                                                                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_imm_map).dom().contains(owner))
                                                                        &&
                                                                        {
                                                                            let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                                                                                (update_tmp_imm_map).remove(owner);
                                                                            (#[verifier::custom_err("cannot prove this 'assert' holds")] (!(update_tmp_imm_map).dom().contains(owner))
                                                                                    &&
                                                                                    {
                                                                                        let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                                                                                            (update_tmp_imm_map).insert(owner,
                                                                                                ::vstd::prelude::spec_literal_integer("0"));
                                                                                        (#[verifier::custom_err("cannot prove that final value of field `imm_map` has this updated value")] (::vstd::prelude::equal(post.imm_map,
                                                                                                        update_tmp_imm_map)) &&
                                                                                                (#[verifier::custom_err("cannot prove that final value of field `mut_map` has this updated value")] (::vstd::prelude::equal(post.mut_map,
                                                                                                                update_tmp_mut_map)) &&
                                                                                                        #[verifier::custom_err("cannot prove that final value of field `owner_map` has this updated value")] (::vstd::prelude::equal(post.owner_map,
                                                                                                                update_tmp_owner_map))))
                                                                                    })
                                                                        })
                                                            })
                                                })
                                    })
                        })
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        #[verus::internal(open)]
        pub fn add_new_owner_enabled(pre: Self, owner: String,
            mutability: bool) -> ::core::primitive::bool {
            {
                let tmp_assert: ::core::primitive::bool = true;
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_owner_map).dom().contains(owner))
                        &&
                        {
                            let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                                (update_tmp_owner_map).remove(owner);
                            let tmp_assert: ::core::primitive::bool =
                                tmp_assert &&
                                    (!(update_tmp_owner_map).dom().contains(owner));
                            (#[verifier::custom_err("cannot prove this condition holds")] ({
                                            ::vstd::prelude::imply(tmp_assert,
                                                (update_tmp_mut_map).dom().contains(owner))
                                        }) &&
                                    {
                                        let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                                            (update_tmp_mut_map).remove(owner);
                                        let tmp_assert: ::core::primitive::bool =
                                            tmp_assert && (!(update_tmp_mut_map).dom().contains(owner));

                                        #[verifier::custom_err("cannot prove this condition holds")]
                                        ({
                                                ::vstd::prelude::imply(tmp_assert,
                                                    (update_tmp_imm_map).dom().contains(owner))
                                            })
                                    })
                        })
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::proof]
        pub fn add_new_owner_asserts(pre: State, owner: String,
            mutability: bool) {
            ::vstd::prelude::assume_(pre.invariant());
            {
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                ::vstd::prelude::assume_((update_tmp_owner_map).dom().contains(owner));
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    (update_tmp_owner_map).remove(owner);
                ::vstd::state_machine_internal::assert_add_map(!(update_tmp_owner_map).dom().contains(owner));
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    (update_tmp_owner_map).insert(owner, mutability);
                ::vstd::prelude::assume_((update_tmp_mut_map).dom().contains(owner));
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    (update_tmp_mut_map).remove(owner);
                ::vstd::state_machine_internal::assert_add_map(!(update_tmp_mut_map).dom().contains(owner));
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    (update_tmp_mut_map).insert(owner,
                        ::vstd::prelude::spec_literal_integer("0"));
                ::vstd::prelude::assume_((update_tmp_imm_map).dom().contains(owner));
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    (update_tmp_imm_map).remove(owner);
                ::vstd::state_machine_internal::assert_add_map(!(update_tmp_imm_map).dom().contains(owner));
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    (update_tmp_imm_map).insert(owner,
                        ::vstd::prelude::spec_literal_integer("0"));
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        #[verus::internal(open)]
        pub fn add_imm_ref(pre: Self, post: Self, owner: String)
            -> ::core::primitive::bool {
            {
                let tmp_assert: ::core::primitive::bool = true;
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_owner_map).dom().contains(owner))
                        &&
                        (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_mut_map).contains_pair(owner,
                                        ::vstd::prelude::spec_literal_integer("0"))) &&
                                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_imm_map).dom().contains(owner))
                                        &&
                                        (({
                                                        let curr = update_tmp_imm_map.index(owner);
                                                        let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                                                            (update_tmp_imm_map).remove(owner);
                                                        let tmp_assert: ::core::primitive::bool =
                                                            tmp_assert && (!(update_tmp_imm_map).dom().contains(owner));
                                                        let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                                                            (update_tmp_imm_map).insert(owner,
                                                                ((curr).spec_add(::vstd::prelude::spec_literal_nat("1"))));

                                                        #[verifier::custom_err("cannot prove that final value of field `imm_map` has this updated value")]
                                                        ({
                                                                ::vstd::prelude::imply(tmp_assert,
                                                                    ::vstd::prelude::equal(post.imm_map, update_tmp_imm_map))
                                                            })
                                                    }) &&
                                                ({
                                                        let tmp_assert =
                                                            ({
                                                                    let curr = update_tmp_imm_map.index(owner);
                                                                    let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                                                                        ((update_tmp_imm_map).remove(owner));
                                                                    let tmp_assert: ::core::primitive::bool =
                                                                        (tmp_assert &&
                                                                                (!(update_tmp_imm_map).dom().contains(owner)));
                                                                    let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                                                                        ((update_tmp_imm_map).insert(owner,
                                                                                ((curr).spec_add(::vstd::prelude::spec_literal_nat("1")))));
                                                                    tmp_assert
                                                                });
                                                        (#[verifier::custom_err("cannot prove that the field `mut_map` is preserved")] ({
                                                                        ::vstd::prelude::imply(tmp_assert,
                                                                            ::vstd::prelude::equal(post.mut_map, update_tmp_mut_map))
                                                                    }) &&
                                                                #[verifier::custom_err("cannot prove that the field `owner_map` is preserved")] ({
                                                                        ::vstd::prelude::imply(tmp_assert,
                                                                            ::vstd::prelude::equal(post.owner_map,
                                                                                update_tmp_owner_map))
                                                                    }))
                                                    })))))
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        #[verus::internal(open)]
        pub fn add_imm_ref_strong(pre: Self, post: Self, owner: String)
            -> ::core::primitive::bool {
            {
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_owner_map).dom().contains(owner))
                        &&
                        (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_mut_map).contains_pair(owner,
                                        ::vstd::prelude::spec_literal_integer("0"))) &&
                                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_imm_map).dom().contains(owner))
                                        &&
                                        (({
                                                        let curr = update_tmp_imm_map.index(owner);
                                                        let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                                                            (update_tmp_imm_map).remove(owner);
                                                        (#[verifier::custom_err("cannot prove this 'assert' holds")] (!(update_tmp_imm_map).dom().contains(owner))
                                                                &&
                                                                {
                                                                    let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                                                                        (update_tmp_imm_map).insert(owner,
                                                                            ((curr).spec_add(::vstd::prelude::spec_literal_nat("1"))));

                                                                    #[verifier::custom_err("cannot prove that final value of field `imm_map` has this updated value")]
                                                                    (::vstd::prelude::equal(post.imm_map, update_tmp_imm_map))
                                                                })
                                                    }) &&
                                                ((#[verifier::custom_err("cannot prove that the field `mut_map` is preserved")] (::vstd::prelude::equal(post.mut_map,
                                                                    update_tmp_mut_map)) &&
                                                            #[verifier::custom_err("cannot prove that the field `owner_map` is preserved")] (::vstd::prelude::equal(post.owner_map,
                                                                    update_tmp_owner_map))))))))
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        #[verus::internal(open)]
        pub fn add_imm_ref_enabled(pre: Self, owner: String)
            -> ::core::primitive::bool {
            {
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_owner_map).dom().contains(owner))
                        &&
                        (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_mut_map).contains_pair(owner,
                                        ::vstd::prelude::spec_literal_integer("0"))) &&
                                #[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_imm_map).dom().contains(owner))))
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::proof]
        pub fn add_imm_ref_asserts(pre: State, owner: String) {
            ::vstd::prelude::assume_(pre.invariant());
            {
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                ::vstd::prelude::assume_((update_tmp_owner_map).dom().contains(owner));
                ::vstd::prelude::assume_((update_tmp_mut_map).contains_pair(owner,
                        ::vstd::prelude::spec_literal_integer("0")));
                ::vstd::prelude::assume_((update_tmp_imm_map).dom().contains(owner));
                {
                    let curr = update_tmp_imm_map.index(owner);
                    let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                        (update_tmp_imm_map).remove(owner);
                    ::vstd::state_machine_internal::assert_add_map(!(update_tmp_imm_map).dom().contains(owner));
                    let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                        (update_tmp_imm_map).insert(owner,
                            ((curr).spec_add(::vstd::prelude::spec_literal_nat("1"))));
                }
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        #[verus::internal(open)]
        pub fn add_mut_ref(pre: Self, post: Self, owner: String)
            -> ::core::primitive::bool {
            {
                let tmp_assert: ::core::primitive::bool = true;
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_owner_map).dom().contains(owner))
                        &&
                        (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_imm_map).contains_pair(owner,
                                        ::vstd::prelude::spec_literal_integer("0"))) &&
                                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_mut_map).contains_pair(owner,
                                                ::vstd::prelude::spec_literal_integer("0"))) &&
                                        {
                                            let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                                                (update_tmp_mut_map).remove(owner);
                                            let tmp_assert: ::core::primitive::bool =
                                                tmp_assert && (!(update_tmp_mut_map).dom().contains(owner));
                                            let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                                                (update_tmp_mut_map).insert(owner,
                                                    ::vstd::prelude::spec_literal_integer("1"));
                                            (#[verifier::custom_err("cannot prove that final value of field `mut_map` has this updated value")] ({
                                                            ::vstd::prelude::imply(tmp_assert,
                                                                ::vstd::prelude::equal(post.mut_map, update_tmp_mut_map))
                                                        }) &&
                                                    (#[verifier::custom_err("cannot prove that the field `imm_map` is preserved")] ({
                                                                    ::vstd::prelude::imply(tmp_assert,
                                                                        ::vstd::prelude::equal(post.imm_map, update_tmp_imm_map))
                                                                }) &&
                                                            #[verifier::custom_err("cannot prove that the field `owner_map` is preserved")] ({
                                                                    ::vstd::prelude::imply(tmp_assert,
                                                                        ::vstd::prelude::equal(post.owner_map,
                                                                            update_tmp_owner_map))
                                                                })))
                                        })))
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        #[verus::internal(open)]
        pub fn add_mut_ref_strong(pre: Self, post: Self, owner: String)
            -> ::core::primitive::bool {
            {
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_owner_map).dom().contains(owner))
                        &&
                        (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_imm_map).contains_pair(owner,
                                        ::vstd::prelude::spec_literal_integer("0"))) &&
                                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_mut_map).contains_pair(owner,
                                                ::vstd::prelude::spec_literal_integer("0"))) &&
                                        {
                                            let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                                                (update_tmp_mut_map).remove(owner);
                                            (#[verifier::custom_err("cannot prove this 'assert' holds")] (!(update_tmp_mut_map).dom().contains(owner))
                                                    &&
                                                    {
                                                        let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                                                            (update_tmp_mut_map).insert(owner,
                                                                ::vstd::prelude::spec_literal_integer("1"));
                                                        (#[verifier::custom_err("cannot prove that final value of field `mut_map` has this updated value")] (::vstd::prelude::equal(post.mut_map,
                                                                        update_tmp_mut_map)) &&
                                                                (#[verifier::custom_err("cannot prove that the field `imm_map` is preserved")] (::vstd::prelude::equal(post.imm_map,
                                                                                update_tmp_imm_map)) &&
                                                                        #[verifier::custom_err("cannot prove that the field `owner_map` is preserved")] (::vstd::prelude::equal(post.owner_map,
                                                                                update_tmp_owner_map))))
                                                    })
                                        })))
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        #[verus::internal(open)]
        pub fn add_mut_ref_enabled(pre: Self, owner: String)
            -> ::core::primitive::bool {
            {
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_owner_map).dom().contains(owner))
                        &&
                        (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_imm_map).contains_pair(owner,
                                        ::vstd::prelude::spec_literal_integer("0"))) &&
                                #[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_mut_map).contains_pair(owner,
                                        ::vstd::prelude::spec_literal_integer("0")))))
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::proof]
        pub fn add_mut_ref_asserts(pre: State, owner: String) {
            ::vstd::prelude::assume_(pre.invariant());
            {
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                ::vstd::prelude::assume_((update_tmp_owner_map).dom().contains(owner));
                ::vstd::prelude::assume_((update_tmp_imm_map).contains_pair(owner,
                        ::vstd::prelude::spec_literal_integer("0")));
                ::vstd::prelude::assume_((update_tmp_mut_map).contains_pair(owner,
                        ::vstd::prelude::spec_literal_integer("0")));
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    (update_tmp_mut_map).remove(owner);
                ::vstd::state_machine_internal::assert_add_map(!(update_tmp_mut_map).dom().contains(owner));
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    (update_tmp_mut_map).insert(owner,
                        ::vstd::prelude::spec_literal_integer("1"));
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        #[verus::internal(open)]
        pub fn drop_imm_ref(pre: Self, post: Self, owner: String)
            -> ::core::primitive::bool {
            {
                let tmp_assert: ::core::primitive::bool = true;
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_owner_map).dom().contains(owner))
                        &&
                        (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_imm_map).dom().contains(owner))
                                &&
                                (({
                                                let curr = update_tmp_imm_map.index(owner);
                                                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                                                    (update_tmp_imm_map).remove(owner);
                                                (#[verifier::custom_err("cannot prove this condition holds")] ({
                                                                ::vstd::prelude::imply(tmp_assert,
                                                                    ((curr).spec_gt(::vstd::prelude::spec_literal_nat("0"))))
                                                            }) &&
                                                        {
                                                            let tmp_assert: ::core::primitive::bool =
                                                                tmp_assert && (!(update_tmp_imm_map).dom().contains(owner));
                                                            let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                                                                (update_tmp_imm_map).insert(owner,
                                                                    ((curr).spec_sub(::vstd::prelude::spec_literal_nat("1"))));

                                                            #[verifier::custom_err("cannot prove that final value of field `imm_map` has this updated value")]
                                                            ({
                                                                    ::vstd::prelude::imply(tmp_assert,
                                                                        ::vstd::prelude::equal(post.imm_map, update_tmp_imm_map))
                                                                })
                                                        })
                                            }) &&
                                        ({
                                                let tmp_assert =
                                                    ({
                                                            let curr = update_tmp_imm_map.index(owner);
                                                            let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                                                                ((update_tmp_imm_map).remove(owner));
                                                            let tmp_assert: ::core::primitive::bool =
                                                                (tmp_assert &&
                                                                        (!(update_tmp_imm_map).dom().contains(owner)));
                                                            let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                                                                ((update_tmp_imm_map).insert(owner,
                                                                        ((curr).spec_sub(::vstd::prelude::spec_literal_nat("1")))));
                                                            tmp_assert
                                                        });
                                                (#[verifier::custom_err("cannot prove that the field `mut_map` is preserved")] ({
                                                                ::vstd::prelude::imply(tmp_assert,
                                                                    ::vstd::prelude::equal(post.mut_map, update_tmp_mut_map))
                                                            }) &&
                                                        #[verifier::custom_err("cannot prove that the field `owner_map` is preserved")] ({
                                                                ::vstd::prelude::imply(tmp_assert,
                                                                    ::vstd::prelude::equal(post.owner_map,
                                                                        update_tmp_owner_map))
                                                            }))
                                            }))))
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        #[verus::internal(open)]
        pub fn drop_imm_ref_strong(pre: Self, post: Self, owner: String)
            -> ::core::primitive::bool {
            {
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_owner_map).dom().contains(owner))
                        &&
                        (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_imm_map).dom().contains(owner))
                                &&
                                (({
                                                let curr = update_tmp_imm_map.index(owner);
                                                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                                                    (update_tmp_imm_map).remove(owner);
                                                (#[verifier::custom_err("cannot prove this condition holds")] (((curr).spec_gt(::vstd::prelude::spec_literal_nat("0"))))
                                                        &&
                                                        (#[verifier::custom_err("cannot prove this 'assert' holds")] (!(update_tmp_imm_map).dom().contains(owner))
                                                                &&
                                                                {
                                                                    let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                                                                        (update_tmp_imm_map).insert(owner,
                                                                            ((curr).spec_sub(::vstd::prelude::spec_literal_nat("1"))));

                                                                    #[verifier::custom_err("cannot prove that final value of field `imm_map` has this updated value")]
                                                                    (::vstd::prelude::equal(post.imm_map, update_tmp_imm_map))
                                                                }))
                                            }) &&
                                        ((#[verifier::custom_err("cannot prove that the field `mut_map` is preserved")] (::vstd::prelude::equal(post.mut_map,
                                                            update_tmp_mut_map)) &&
                                                    #[verifier::custom_err("cannot prove that the field `owner_map` is preserved")] (::vstd::prelude::equal(post.owner_map,
                                                            update_tmp_owner_map)))))))
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        #[verus::internal(open)]
        pub fn drop_imm_ref_enabled(pre: Self, owner: String)
            -> ::core::primitive::bool {
            {
                let tmp_assert: ::core::primitive::bool = true;
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_owner_map).dom().contains(owner))
                        &&
                        (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_imm_map).dom().contains(owner))
                                &&
                                {
                                    let curr = update_tmp_imm_map.index(owner);

                                    #[verifier::custom_err("cannot prove this condition holds")]
                                    ({
                                            ::vstd::prelude::imply(tmp_assert,
                                                ((curr).spec_gt(::vstd::prelude::spec_literal_nat("0"))))
                                        })
                                }))
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::proof]
        pub fn drop_imm_ref_asserts(pre: State, owner: String) {
            ::vstd::prelude::assume_(pre.invariant());
            {
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                ::vstd::prelude::assume_((update_tmp_owner_map).dom().contains(owner));
                ::vstd::prelude::assume_((update_tmp_imm_map).dom().contains(owner));
                {
                    let curr = update_tmp_imm_map.index(owner);
                    let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                        (update_tmp_imm_map).remove(owner);
                    ::vstd::prelude::assume_(((curr).spec_gt(::vstd::prelude::spec_literal_nat("0"))));
                    ::vstd::state_machine_internal::assert_add_map(!(update_tmp_imm_map).dom().contains(owner));
                    let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                        (update_tmp_imm_map).insert(owner,
                            ((curr).spec_sub(::vstd::prelude::spec_literal_nat("1"))));
                }
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        #[verus::internal(open)]
        pub fn drop_mut_ref(pre: Self, post: Self, owner: String)
            -> ::core::primitive::bool {
            {
                let tmp_assert: ::core::primitive::bool = true;
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_owner_map).dom().contains(owner))
                        &&
                        (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_mut_map).contains_pair(owner,
                                        ::vstd::prelude::spec_literal_integer("1"))) &&
                                {
                                    let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                                        (update_tmp_mut_map).remove(owner);
                                    let tmp_assert: ::core::primitive::bool =
                                        tmp_assert && (!(update_tmp_mut_map).dom().contains(owner));
                                    let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                                        (update_tmp_mut_map).insert(owner,
                                            ::vstd::prelude::spec_literal_integer("0"));
                                    (#[verifier::custom_err("cannot prove that final value of field `mut_map` has this updated value")] ({
                                                    ::vstd::prelude::imply(tmp_assert,
                                                        ::vstd::prelude::equal(post.mut_map, update_tmp_mut_map))
                                                }) &&
                                            (#[verifier::custom_err("cannot prove that the field `imm_map` is preserved")] ({
                                                            ::vstd::prelude::imply(tmp_assert,
                                                                ::vstd::prelude::equal(post.imm_map, update_tmp_imm_map))
                                                        }) &&
                                                    #[verifier::custom_err("cannot prove that the field `owner_map` is preserved")] ({
                                                            ::vstd::prelude::imply(tmp_assert,
                                                                ::vstd::prelude::equal(post.owner_map,
                                                                    update_tmp_owner_map))
                                                        })))
                                }))
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        #[verus::internal(open)]
        pub fn drop_mut_ref_strong(pre: Self, post: Self, owner: String)
            -> ::core::primitive::bool {
            {
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_owner_map).dom().contains(owner))
                        &&
                        (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_mut_map).contains_pair(owner,
                                        ::vstd::prelude::spec_literal_integer("1"))) &&
                                {
                                    let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                                        (update_tmp_mut_map).remove(owner);
                                    (#[verifier::custom_err("cannot prove this 'assert' holds")] (!(update_tmp_mut_map).dom().contains(owner))
                                            &&
                                            {
                                                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                                                    (update_tmp_mut_map).insert(owner,
                                                        ::vstd::prelude::spec_literal_integer("0"));
                                                (#[verifier::custom_err("cannot prove that final value of field `mut_map` has this updated value")] (::vstd::prelude::equal(post.mut_map,
                                                                update_tmp_mut_map)) &&
                                                        (#[verifier::custom_err("cannot prove that the field `imm_map` is preserved")] (::vstd::prelude::equal(post.imm_map,
                                                                        update_tmp_imm_map)) &&
                                                                #[verifier::custom_err("cannot prove that the field `owner_map` is preserved")] (::vstd::prelude::equal(post.owner_map,
                                                                        update_tmp_owner_map))))
                                            })
                                }))
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        #[verus::internal(open)]
        pub fn drop_mut_ref_enabled(pre: Self, owner: String)
            -> ::core::primitive::bool {
            {
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                (#[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_owner_map).dom().contains(owner))
                        &&
                        #[verifier::custom_err("cannot prove this condition holds")] ((update_tmp_mut_map).contains_pair(owner,
                                ::vstd::prelude::spec_literal_integer("1"))))
            }
        }
        #[verus::internal(verus_macro)]
        #[verifier::proof]
        pub fn drop_mut_ref_asserts(pre: State, owner: String) {
            ::vstd::prelude::assume_(pre.invariant());
            {
                let update_tmp_owner_map: ::vstd::map::Map<String, bool> =
                    pre.owner_map;
                let update_tmp_imm_map: ::vstd::map::Map<String, int> =
                    pre.imm_map;
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    pre.mut_map;
                ::vstd::prelude::assume_((update_tmp_owner_map).dom().contains(owner));
                ::vstd::prelude::assume_((update_tmp_mut_map).contains_pair(owner,
                        ::vstd::prelude::spec_literal_integer("1")));
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    (update_tmp_mut_map).remove(owner);
                ::vstd::state_machine_internal::assert_add_map(!(update_tmp_mut_map).dom().contains(owner));
                let update_tmp_mut_map: ::vstd::map::Map<String, int> =
                    (update_tmp_mut_map).insert(owner,
                        ::vstd::prelude::spec_literal_integer("0"));
            }
        }
        #[verifier::opaque]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        pub fn next_by(pre: State, post: State, step: Step)
            -> ::core::primitive::bool {
            match step {
                Step::add_new_owner(owner, mutability) =>
                    Self::add_new_owner(pre, post, owner, mutability),
                Step::add_imm_ref(owner) =>
                    Self::add_imm_ref(pre, post, owner),
                Step::add_mut_ref(owner) =>
                    Self::add_mut_ref(pre, post, owner),
                Step::drop_imm_ref(owner) =>
                    Self::drop_imm_ref(pre, post, owner),
                Step::drop_mut_ref(owner) =>
                    Self::drop_mut_ref(pre, post, owner),
                Step::dummy_to_use_type_params(_) => false,
            }
        }
        #[verifier::opaque]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        pub fn next(pre: State, post: State) -> ::core::primitive::bool {
            ::vstd::prelude::exists(|step: Step|
                    Self::next_by(pre, post, step))
        }
        #[verifier::opaque]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        pub fn next_strong_by(pre: State, post: State, step: Step)
            -> ::core::primitive::bool {
            match step {
                Step::add_new_owner(owner, mutability) =>
                    Self::add_new_owner_strong(pre, post, owner, mutability),
                Step::add_imm_ref(owner) =>
                    Self::add_imm_ref_strong(pre, post, owner),
                Step::add_mut_ref(owner) =>
                    Self::add_mut_ref_strong(pre, post, owner),
                Step::drop_imm_ref(owner) =>
                    Self::drop_imm_ref_strong(pre, post, owner),
                Step::drop_mut_ref(owner) =>
                    Self::drop_mut_ref_strong(pre, post, owner),
                Step::dummy_to_use_type_params(_) => false,
            }
        }
        #[verifier::opaque]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        pub fn next_strong(pre: State, post: State)
            -> ::core::primitive::bool {
            ::vstd::prelude::exists(|step: Step|
                    Self::next_strong_by(pre, post, step))
        }
        #[verifier::opaque]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        pub fn init_by(post: State, step: Config) -> ::core::primitive::bool {
            match step {
                Config::initialize() => Self::initialize(post),
                Config::dummy_to_use_type_params(_) => false,
            }
        }
        #[verifier::opaque]
        #[verus::internal(open)]
        #[verus::internal(verus_macro)]
        #[verifier::spec]
        pub fn init(post: State) -> ::core::primitive::bool {
            ::vstd::prelude::exists(|step: Config| Self::init_by(post, step))
        }
        #[verifier::spec]
        #[verus::internal(verus_macro)]
        #[verus::internal(open)]
        pub fn invariant(&self) -> ::core::primitive::bool {
            self.aliasing_xor_mutability() && self.non_zero_imm_refs() &&
                    self.single_mutable_refs() && self.synchronicity()
        }
        #[verus::internal(verus_macro)]
        #[verus::internal(open)]
        #[verus::internal(spec)]
        pub fn aliasing_xor_mutability(self) -> bool {
            ::vstd::prelude::forall(|owner: String|
                    ::vstd::prelude::imply(self.owner_map.dom().contains(owner),
                        (::vstd::prelude::imply((self.mut_map.spec_index(owner)).spec_gt(::vstd::prelude::spec_literal_nat("0")),
                                ::vstd::prelude::spec_eq(self.imm_map.spec_index(owner),
                                    ::vstd::prelude::spec_literal_nat("0"))))))
        }
        #[verus::internal(verus_macro)]
        #[verus::internal(open)]
        #[verus::internal(spec)]
        pub fn non_zero_imm_refs(self) -> bool {
            ::vstd::prelude::forall(|owner: String|
                    ::vstd::prelude::imply(self.imm_map.dom().contains(owner),
                        ((self.imm_map.spec_index(owner)).spec_ge(::vstd::prelude::spec_literal_nat("0")))))
        }
        #[verus::internal(verus_macro)]
        #[verus::internal(open)]
        #[verus::internal(spec)]
        pub fn single_mutable_refs(self) -> bool {
            ::vstd::prelude::forall(|owner: String|
                    ::vstd::prelude::imply(self.mut_map.dom().contains(owner),
                        (::vstd::prelude::spec_eq(self.mut_map.spec_index(owner),
                                    ::vstd::prelude::spec_literal_nat("0")) ||
                                ::vstd::prelude::spec_eq(self.mut_map.spec_index(owner),
                                    ::vstd::prelude::spec_literal_nat("1")))))
        }
        #[verus::internal(verus_macro)]
        #[verus::internal(open)]
        #[verus::internal(spec)]
        pub fn synchronicity(self) -> bool {
            ::vstd::prelude::forall(|owner: String|
                    (self.owner_map.dom().contains(owner)) ==
                        ((self.imm_map.dom().contains(owner) &&
                                    self.mut_map.dom().contains(owner))))
        }
        #[verifier::custom_req_err("could not show invariant `aliasing_xor_mutability` on the `post` state")]
        #[verifier::external_body]
        #[verus::internal(verus_macro)]
        #[verifier::proof]
        fn lemma_msg_aliasing_xor_mutability(s: State) {
            ::vstd::prelude::requires(s.aliasing_xor_mutability());
            ::vstd::prelude::ensures(s.aliasing_xor_mutability());
        }
        #[verifier::custom_req_err("could not show invariant `non_zero_imm_refs` on the `post` state")]
        #[verifier::external_body]
        #[verus::internal(verus_macro)]
        #[verifier::proof]
        fn lemma_msg_non_zero_imm_refs(s: State) {
            ::vstd::prelude::requires(s.non_zero_imm_refs());
            ::vstd::prelude::ensures(s.non_zero_imm_refs());
        }
        #[verifier::custom_req_err("could not show invariant `single_mutable_refs` on the `post` state")]
        #[verifier::external_body]
        #[verus::internal(verus_macro)]
        #[verifier::proof]
        fn lemma_msg_single_mutable_refs(s: State) {
            ::vstd::prelude::requires(s.single_mutable_refs());
            ::vstd::prelude::ensures(s.single_mutable_refs());
        }
        #[verifier::custom_req_err("could not show invariant `synchronicity` on the `post` state")]
        #[verifier::external_body]
        #[verus::internal(verus_macro)]
        #[verifier::proof]
        fn lemma_msg_synchronicity(s: State) {
            ::vstd::prelude::requires(s.synchronicity());
            ::vstd::prelude::ensures(s.synchronicity());
        }
        #[verus::internal(verus_macro)]
        #[verus::internal(proof)]
        fn initialize_inductive(post: Self) {
            ::vstd::prelude::requires(Self::initialize(post));
            ::vstd::prelude::ensures(post.invariant());
            {}
            Self::lemma_msg_aliasing_xor_mutability(post);
            Self::lemma_msg_non_zero_imm_refs(post);
            Self::lemma_msg_single_mutable_refs(post);
            Self::lemma_msg_synchronicity(post);
        }
        #[verus::internal(verus_macro)]
        #[verus::internal(proof)]
        fn add_new_owner_inductive(pre: Self, post: Self, owner: String,
            mutability: bool) {
            ::vstd::prelude::requires(pre.invariant() &&
                    State::add_new_owner_strong(pre, post, owner, mutability));
            ::vstd::prelude::ensures(post.invariant());
            {}
            Self::lemma_msg_aliasing_xor_mutability(post);
            Self::lemma_msg_non_zero_imm_refs(post);
            Self::lemma_msg_single_mutable_refs(post);
            Self::lemma_msg_synchronicity(post);
        }
        #[verus::internal(verus_macro)]
        #[verus::internal(proof)]
        fn add_imm_ref_inductive(pre: Self, post: Self, owner: String) {
            ::vstd::prelude::requires(pre.invariant() &&
                    State::add_imm_ref_strong(pre, post, owner));
            ::vstd::prelude::ensures(post.invariant());
            {}
            Self::lemma_msg_aliasing_xor_mutability(post);
            Self::lemma_msg_non_zero_imm_refs(post);
            Self::lemma_msg_single_mutable_refs(post);
            Self::lemma_msg_synchronicity(post);
        }
        #[verus::internal(verus_macro)]
        #[verus::internal(proof)]
        fn add_mut_ref_inductive(pre: Self, post: Self, owner: String) {
            ::vstd::prelude::requires(pre.invariant() &&
                    State::add_mut_ref_strong(pre, post, owner));
            ::vstd::prelude::ensures(post.invariant());
            {}
            Self::lemma_msg_aliasing_xor_mutability(post);
            Self::lemma_msg_non_zero_imm_refs(post);
            Self::lemma_msg_single_mutable_refs(post);
            Self::lemma_msg_synchronicity(post);
        }
        #[verus::internal(verus_macro)]
        #[verus::internal(proof)]
        fn drop_imm_ref_inductive(pre: Self, post: Self, owner: String) {
            ::vstd::prelude::requires(pre.invariant() &&
                    State::drop_imm_ref_strong(pre, post, owner));
            ::vstd::prelude::ensures(post.invariant());
            {}
            Self::lemma_msg_aliasing_xor_mutability(post);
            Self::lemma_msg_non_zero_imm_refs(post);
            Self::lemma_msg_single_mutable_refs(post);
            Self::lemma_msg_synchronicity(post);
        }
        #[verus::internal(verus_macro)]
        #[verus::internal(proof)]
        fn drop_mut_ref_inductive(pre: Self, post: Self, owner: String) {
            ::vstd::prelude::requires(pre.invariant() &&
                    State::drop_mut_ref_strong(pre, post, owner));
            ::vstd::prelude::ensures(post.invariant());
            {}
            Self::lemma_msg_aliasing_xor_mutability(post);
            Self::lemma_msg_non_zero_imm_refs(post);
            Self::lemma_msg_single_mutable_refs(post);
            Self::lemma_msg_synchronicity(post);
        }
    }
}
#[verus::internal(verus_macro)]
#[verus::internal(proof)]
fn main_model() {
    #[verus::internal(proof)]
    let (verus_tmp_instance, verus_tmp_owner_map, verus_tmp_imm_map,
            verus_tmp_mut_map) = State::Instance::initialize();
    #[verus::internal(proof)]
    let instance = verus_tmp_instance.get();
    #[verus::internal(proof)]
    let owner_map = verus_tmp_owner_map.get();
    #[verus::internal(proof)]
    let imm_map = verus_tmp_imm_map.get();
    #[verus::internal(proof)]
    let mut_map = verus_tmp_mut_map.get();
    #[verus::internal(proof)]
    let (verus_tmp_owner_token, verus_tmp_imm_token_init,
            verus_tmp_mut_token_init) =
        instance.add_new_owner("x".to_string(), true, owner_map, imm_map,
            mut_map);
    #[verus::internal(proof)]
    let owner_token = verus_tmp_owner_token.get();
    #[verus::internal(proof)]
    let imm_token_init = verus_tmp_imm_token_init.get();
    #[verus::internal(proof)]
    let mut_token_init = verus_tmp_mut_token_init.get();
}
#[verus::internal(verus_macro)]
fn main() {}
#![feature(prelude_import)]

// mutable = true
// immutable = false



//immutable refs

// mutable refs















// owner exists in owner map

// no mutable refs exist (mut_map[owner] == 0)


// owner exists in owner map

// no immutable refs exist (imm_map[owner] == 0)

// only add a mut ref for an owner with not mutable refs


// if you try to remove an imm ref with counter == 0, it just removes it


// Initialize the state machine

// let mut x = 7;
// add_new_owner takes individual tokens (owner_map, imm_map, mut_map) and returns individual tokens

// instance.add_new_owner("x".to_string(), true, &owner_map, &imm_map, &mut_map);

// let tracked sm = X::Instance::initialize();

// // // model the owner “x”
// let tracked (
//     Tracked(owner_token),
//     Tracked(mut_token),
//     Tracked(imm_token),
//     _
// ) = sm.add_new_owner("x".to_string(), true);

// // model r1 = &x
// let ghost sm = X::add_imm_ref(sm, "x".to_string());

// // model r2 = &x
// let ghost sm = X::add_imm_ref(sm, "x".to_string());

// // immutable refs go out of scope before mutable ones:
// let ghost sm = X::drop_imm_ref(sm, "x".to_string());
// let ghost sm = X::drop_imm_ref(sm, "x".to_string());

// // model r3 = &mut x
// let ghost sm = X::add_mut_ref(sm, "x".to_string());
// let ghost sm = X::drop_mut_ref(sm, "x".to_string());

// // model r4 = &mut x
// let ghost sm = X::add_mut_ref(sm, "x".to_string());
// let ghost sm = X::drop_mut_ref(sm, "x".to_string());



#![allow(internal_features)]
#![feature(stmt_expr_attributes)]
#![feature(box_patterns)]
#![feature(negative_impls)]
#![feature(rustc_attrs)]
#![feature(unboxed_closures)]
#![feature(register_tool)]
#![feature(tuple_trait)]
#![feature(custom_inner_attributes)]
#![feature(try_trait_v2)]
#![register_tool(verus)]
#![register_tool(verifier)]
#![register_tool(verusfmt)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use verus_builtin::*;
use verus_builtin_macros::*;
use verus_state_machines_macros::tokenized_state_machine;
use std::sync::Arc;
use vstd::atomic_ghost::*;
use vstd::modes::*;
use vstd::prelude::*;
use vstd::thread::*;
use vstd::{pervasive::*, *};
#[allow(unused_parens)]
pub mod State {
    use super::*;
    use ::vstd::tokens::ValueToken;
    use ::vstd::tokens::KeyValueToken;
    use ::vstd::tokens::CountToken;
    use ::vstd::tokens::MonotonicCountToken;
    use ::vstd::tokens::ElementToken;
    use ::vstd::tokens::SimpleToken;
    #[verus::internal(verus_macro)]
    #[verifier::ext_equal]
    pub struct State {
        pub owner_map: ::vstd::map::Map<String, bool>,
        pub imm_map: ::vstd::map::Map<String, int>,
        pub mut_map: ::vstd::map::Map<String, int>,
    }
    #[allow(non_camel_case_types)]
    #[verus::internal(verus_macro)]
    pub enum Step {
        add_new_owner(String, bool),
        add_imm_ref(String),
        add_mut_ref(String),
        drop_imm_ref(String),
        drop_mut_ref(String),
        dummy_to_use_type_params(State),
    }
    #[verus::internal(verus_macro)]
    impl Step {
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_1(self) -> bool {
            ::vstd::prelude::get_variant_field(self, "add_new_owner", "1")
        }
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_add_new_owner_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "add_new_owner", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_add_new_owner_1(self) -> bool {
            ::vstd::prelude::get_variant_field(self, "add_new_owner", "1")
        }
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_add_imm_ref_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "add_imm_ref", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_add_mut_ref_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "add_mut_ref", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_drop_imm_ref_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "drop_imm_ref", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_drop_mut_ref_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "drop_mut_ref", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_dummy_to_use_type_params_0(self) -> State {
            ::vstd::prelude::get_variant_field(self,
                "dummy_to_use_type_params", "0")
        }
    }
    #[verus::internal(verus_macro)]
    #[automatically_derived]
    impl Step {
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn is_add_new_owner(&self) -> bool {
            ::vstd::prelude::is_variant(self, "add_new_owner")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn get_add_new_owner_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "add_new_owner", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn get_add_new_owner_1(self) -> bool {
            ::vstd::prelude::get_variant_field(self, "add_new_owner", "1")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn is_add_imm_ref(&self) -> bool {
            ::vstd::prelude::is_variant(self, "add_imm_ref")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn get_add_imm_ref_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "add_imm_ref", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn is_add_mut_ref(&self) -> bool {
            ::vstd::prelude::is_variant(self, "add_mut_ref")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn get_add_mut_ref_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "add_mut_ref", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn is_drop_imm_ref(&self) -> bool {
            ::vstd::prelude::is_variant(self, "drop_imm_ref")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn get_drop_imm_ref_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "drop_imm_ref", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn is_drop_mut_ref(&self) -> bool {
            ::vstd::prelude::is_variant(self, "drop_mut_ref")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn get_drop_mut_ref_0(self) -> String {
            ::vstd::prelude::get_variant_field(self, "drop_mut_ref", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn is_dummy_to_use_type_params(&self) -> bool {
            ::vstd::prelude::is_variant(self, "dummy_to_use_type_params")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn get_dummy_to_use_type_params_0(self) -> State {
            ::vstd::prelude::get_variant_field(self,
                "dummy_to_use_type_params", "0")
        }
    }
    #[allow(non_camel_case_types)]
    #[verus::internal(verus_macro)]
    pub enum Config { initialize(), dummy_to_use_type_params(State), }
    #[verus::internal(verus_macro)]
    impl Config {
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_0(self) -> State {
            ::vstd::prelude::get_variant_field(self,
                "dummy_to_use_type_params", "0")
        }
        #[allow(non_snake_case)]
        #[verus::internal(verus_macro)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn arrow_dummy_to_use_type_params_0(self) -> State {
            ::vstd::prelude::get_variant_field(self,
                "dummy_to_use_type_params", "0")
        }
    }
    #[verus::internal(verus_macro)]
    #[automatically_derived]
    impl Config {
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn is_initialize(&self) -> bool {
            ::vstd::prelude::is_variant(self, "initialize")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn is_dummy_to_use_type_params(&self) -> bool {
            ::vstd::prelude::is_variant(self, "dummy_to_use_type_params")
        }
        #[allow(non_snake_case)]
        #[verus::internal(spec)]
        #[verifier::inline]
        #[verus::internal(open)]
        pub fn get_dummy_to_use_type_params_0(self) -> State {
            ::vstd::prelude::get_variant_field(self,
                "dummy_to_use_type_params", "0")
        }
    }
    pub mod show {
        use super::*;
        use bool as add_new_owner;
        use bool as add_imm_ref;
        use bool as add_mut_ref;
        use bool as drop_imm_ref;
        use bool as drop_mut_ref;
        use bool as initialize;
    }
    pub mod take_step {
        use super::*;
        use bool as initialize;
        use bool as add_new_owner;
        use bool as add_imm_ref;
        use bool as add_mut_ref;
        use bool as drop_imm_ref;
        use bool as drop_mut_ref;
    }
    #[verifier::proof]
    #[allow(non_camel_case_types)]
    pub struct Instance {
        #[verifier::spec]
        send_sync: ::vstd::state_machine_internal::SyncSendIfSyncSend<()>,
        #[verifier::spec]
        state: ::core::option::Option<::vstd::prelude::Ghost<State>>,
        #[verifier::spec]
        location: ::vstd::prelude::int,
    }
    #[verifier::proof]
    #[allow(non_camel_case_types)]
    pub struct owner_map {
        #[verifier::proof]
        dummy_instance: Instance,
        no_copy: ::vstd::state_machine_internal::NoCopy,
    }
    #[verus::internal(verus_macro)]
    impl owner_map { }
    #[verus::internal(verus_macro)]
    #[verus::internal(verus_macro)]
    impl ::vstd::tokens::KeyValueToken<String, bool> for owner_map {
        #[verifier::spec]
        #[verifier::external_body]
        fn instance_id(&self) -> ::vstd::tokens::InstanceId {
            ::core::panicking::panic("not implemented")
        }
        #[verifier::spec]
        #[verifier::external_body]
        fn key(&self) -> String {
            ::core::panicking::panic("not implemented")
        }
        #[verifier::spec]
        #[verifier::external_body]
        fn value(&self) -> bool {
            ::core::panicking::panic("not implemented")
        }
        #[verifier::proof]
        #[verifier::external_body]
        fn agree(#[verifier::proof] &self, #[verifier::proof] other: &Self) {
            ::core::panicking::panic("not implemented");
        }
        #[verifier::proof]
        #[verifier::external_body]
        #[verifier::returns(proof)]
        fn arbitrary() -> Self {
            ::core::panicking::panic("not implemented");
        }
    }
    #[verus::internal(verus_macro)]
    #[verus::internal(verus_macro)]
    impl ::vstd::tokens::UniqueKeyValueToken<String, bool> for owner_map {
        #[verifier::external_body]
        #[verifier::proof]
        fn unique(#[verifier::proof] &mut self,
            #[verifier::proof] other: &Self) {
            ::core::panicking::panic("not implemented");
        }
    }
    #[allow(type_alias_bounds)]
    #[allow(non_camel_case_types)]
    pub type owner_map_map =
        ::vstd::tokens::MapToken<String, bool, owner_map>;
    #[verifier::proof]
    #[allow(non_camel_case_types)]
    pub struct imm_map {
        #[verifier::proof]
        dummy_instance: Instance,
        no_copy: ::vstd::state_machine_internal::NoCopy,
    }
    #[verus::internal(verus_macro)]
    impl imm_map { }
    #[verus::internal(verus_macro)]
    #[verus::internal(verus_macro)]
    impl ::vstd::tokens::KeyValueToken<String, int> for imm_map {
        #[verifier::spec]
        #[verifier::external_body]
        fn instance_id(&self) -> ::vstd::tokens::InstanceId {
            ::core::panicking::panic("not implemented")
        }
        #[verifier::spec]
        #[verifier::external_body]
        fn key(&self) -> String {
            ::core::panicking::panic("not implemented")
        }
        #[verifier::spec]
        #[verifier::external_body]
        fn value(&self) -> int { ::core::panicking::panic("not implemented") }
        #[verifier::proof]
        #[verifier::external_body]
        fn agree(#[verifier::proof] &self, #[verifier::proof] other: &Self) {
            ::core::panicking::panic("not implemented");
        }
        #[verifier::proof]
        #[verifier::external_body]
        #[verifier::returns(proof)]
        fn arbitrary() -> Self {
            ::core::panicking::panic("not implemented");
        }
    }
    #[verus::internal(verus_macro)]
    #[verus::internal(verus_macro)]
    impl ::vstd::tokens::UniqueKeyValueToken<String, int> for imm_map {
        #[verifier::external_body]
        #[verifier::proof]
        fn unique(#[verifier::proof] &mut self,
            #[verifier::proof] other: &Self) {
            ::core::panicking::panic("not implemented");
        }
    }
    #[allow(type_alias_bounds)]
    #[allow(non_camel_case_types)]
    pub type imm_map_map = ::vstd::tokens::MapToken<String, int, imm_map>;
    #[verifier::proof]
    #[allow(non_camel_case_types)]
    pub struct mut_map {
        #[verifier::proof]
        dummy_instance: Instance,
        no_copy: ::vstd::state_machine_internal::NoCopy,
    }
    #[verus::internal(verus_macro)]
    impl mut_map { }
    #[verus::internal(verus_macro)]
    #[verus::internal(verus_macro)]
    impl ::vstd::tokens::KeyValueToken<String, int> for mut_map {
        #[verifier::spec]
        #[verifier::external_body]
        fn instance_id(&self) -> ::vstd::tokens::InstanceId {
            ::core::panicking::panic("not implemented")
        }
        #[verifier::spec]
        #[verifier::external_body]
        fn key(&self) -> String {
            ::core::panicking::panic("not implemented")
        }
        #[verifier::spec]
        #[verifier::external_body]
        fn value(&self) -> int { ::core::panicking::panic("not implemented") }
        #[verifier::proof]
        #[verifier::external_body]
        fn agree(#[verifier::proof] &self, #[verifier::proof] other: &Self) {
            ::core::panicking::panic("not implemented");
        }
        #[verifier::proof]
        #[verifier::external_body]
        #[verifier::returns(proof)]
        fn arbitrary() -> Self {
            ::core::panicking::panic("not implemented");
        }
    }
    #[verus::internal(verus_macro)]
    #[verus::internal(verus_macro)]
    impl ::vstd::tokens::UniqueKeyValueToken<String, int> for mut_map {
        #[verifier::external_body]
        #[verifier::proof]
        fn unique(#[verifier::proof] &mut self,
            #[verifier::proof] other: &Self) {
            ::core::panicking::panic("not implemented");
        }
    }
    #[allow(type_alias_bounds)]
    #[allow(non_camel_case_types)]
    pub type mut_map_map = ::vstd::tokens::MapToken<String, int, mut_map>;
    #[verus::internal(verus_macro)]
    impl Instance {
        #[verifier::spec]
        #[verifier::external_body]
        pub fn id(&self) -> ::vstd::tokens::InstanceId {
            ::core::panicking::panic("not implemented")
        }
    }
    #[verus::internal(verus_macro)]
    #[verus::internal(verus_macro)]
    impl ::core::clone::Clone for Instance {
        #[verifier::external_body]
        fn clone(&self) -> Self {
            ::core::panicking::panic("not implemented");
        }
    }
    #[verus::internal(verus_macro)]
    #[verus::internal(verus_macro)]
    impl ::core::marker::Copy for Instance { }
    #[verus::internal(verus_macro)]
    impl State { }
}
#[allow(unused_imports)]
fn main_model() { ::core::panicking::panic("not implemented") }
fn main() {}
