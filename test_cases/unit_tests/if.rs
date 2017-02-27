fn main() {
    /*Crust with Strict Mode enabled, declares all variables as immutable.
     * If you are mutating the below variable anywhere in program, please change the declaration statement as
     * let mut var_name:type=init_val;
     **/
    let a: [i32; 3] = [5, 4, 65];
    /*Crust with Strict Mode enabled, declares all variables as immutable.
     * If you are mutating the below variable anywhere in program, please change the declaration statement as
     * let mut var_name:type=init_val;
     **/
    let b: i32 = 34;
    a[2];
    b = a;
    func();
    b = func;
}
fn func() -> i32 {
    return 0;
}
