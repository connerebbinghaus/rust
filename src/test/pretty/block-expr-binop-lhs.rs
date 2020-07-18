// compile-flags: --crate-type=lib
// pretty-mode:expanded
// pp-exact:block-expr-binop-lhs.pp


// Tests that block expressions on the left hand side of block expressions
// get parentheses.

macro_rules! unwrap_or_ret_false {
    ($v:expr) => {
        match $v {
            Some(val) => val,
            None => return false,
        }
    }
}

fn test(v: Option<i32>) -> bool {
    unwrap_or_ret_false!(v) == 42
}