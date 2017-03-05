struct A {
    a: i32,
    b: i32, 
/* Rust structures do not support constructors
 * Please handle them with static builder functions
>>>>>>>> A ( ) { a = 5 ; b = 6 ; } */
}
impl A {
    fn getfloat(&self) -> f32 {
        /* Crust tries to identify return statement and replace with rust equivalent
         * shorthand notation. If error found in this line, Please replace shorthand notation
         * with return statement
         **/
        1.23
    }
    pub fn getInt(&self, a: i32) -> i32 {
        /* Crust tries to identify return statement and replace with rust equivalent
         * shorthand notation. If error found in this line, Please replace shorthand notation
         * with return statement
         **/
        a
    }
}
fn main() {
    /* Declaration of a structure should be completed with initialization of it's fields
     * Parser may miss the generation of initialization statements.
     * It should be in the following format
     * let variable:struct_name = struct_name { member1:value1, member2:value2,..}
     */
    let a = A {};
}
