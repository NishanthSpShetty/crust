fn main() {
    /*Crust with Strict Mode enabled, declares all variables as immutable.
     * If you are mutating the below variable anywhere in program, please change the declaration statement as
     * let mut var_name:type=init_val;
     **/
    let a: i32 = 3;
    /*Crust with Strict Mode enabled, declares all variables as immutable.
     * If you are mutating the below variable anywhere in program, please change the declaration statement as
     * let mut var_name:type=init_val;
     **/
    let b: i32;
    if a == b {
        /* Crust tries to identify return statement and replace with rust equivalent
         * shorthand notation. If error found in this line, Please replace shorthand notation
         * with return statement
         **/
        100
    }
    b = func(32, a);
}
fn func(a: i32, b: i32) -> i32 {
    /* Crust tries to identify return statement and replace with rust equivalent
     * shorthand notation. If error found in this line, Please replace shorthand notation
     * with return statement
     **/
    a + b
}
