fn main() {
    /*Avoid using mutable variables unless it is necessary to do so
     */
    let mut a: i32 = 3;
    /*Avoid using mutable variables unless it is necessary to do so
     */
    let mut b: i32;
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
