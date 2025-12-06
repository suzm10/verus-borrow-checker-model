// use std::collections::HashMap;

// m, lifetime 'm' defined here
// fn get_default_mut<'m, K, V>(map: &'m mut HashMap<K, V>, key: &K) -> &'m mut V
// where
//     K: Eq + Hash + Clone,
//     V: Default,
// {
//     // The value is only borrowed in one of the branches (match on `Some(&mut V)`)
//     // but the borrow produced by `map.get_mut(key)` is considered "in scope"
//     // even in the "None" branch where no reference is available
//     // returning this value requires that `*map` is borrowed for 'm'
//     match map.get_mut(key) {
//         Some(value) => value,
//         None => {
//             map.insert(key.clone(), V::default());
//             map.get_mut(&key).unwrap();
//         }
//     }
// }

// fn main() {
//     let mut x = 7;

//     let r1 = &x;
//     let r2 = &x;
//     // immutable borrows used here
//     println!("r1: {}, r2: {}", r1, r2);

//     // mutable borrow happens after immutable ones are done
//     let r3 = &mut x;
//     *r3 += 3;

//     let r4 = &mut x;
//     *r4 += 3;

//     println!("x = {}", x);
// }

fn main() {
    // let mut v = vec![10, 11];
    // let vptr = &mut v[1];
    // v.push(12);
    // println!("v[1] = {}", *vptr);

    // let mut local = 42;
    // let x = &mut local;
    // let shared1 = &*x; // Derive two shared references...
    // let shared2 = &*x; // ...from the same mutable reference, x.
    // let val = *x; // Use all *three* references...
    // let val = *shared1; // ...interchangeably...
    // let val = *shared2; // ...for read accesses.
    // *x += 17; // Use x again for a write access.
    // let val = *shared1; // Error! shared1 used after x got mutated.

    let mut local = 6;
    let x = &local;
    let result = example2(x, |inner_x| {
        retag inner_x;
        let raw_pointer: *mut i32= unsafe { mem::transmute(inner_x) };
        unsafe { *raw_pointer = 15; }
    });
    println!("{}", result); // Prints "5" (aka 15/3).
}