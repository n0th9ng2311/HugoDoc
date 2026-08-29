// <START FILE LOC>
// hugo
// <END FILE LOC>

// !!
// add(u32, u32) -> u32
// !!
// !/!
// **Parameters**:
// 1) u32: the first parameter
// 2) u32: the second parameter
// 3) return: u32
// !/!
// w!
// `*warning the addition can overflow!*`
// !w

/* !/!
 **EXAMPLE:**
 ```rust
 fn test(){
    let some = add(5,4);
    some
 }
 ```
 !/!
*/

/*
!/!
Hopefully this works!
!/!
 */

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
