fn main() {
    match a {
        1 => {
            a += 1;
            func();
            b -= 1;
            /*crust in strict mod avoids declaring all variables as mutable.
             * If you are mutating any values anywhere in program please change the declaration statement as
             * let mut var_name:type=init_val;
             **/
            let a: i32 = 5;
        }
        "2" => {
            /*crust in strict mod avoids declaring all variables as mutable.
             * If you are mutating any values anywhere in program please change the declaration statement as
             * let mut var_name:type=init_val;
             **/
            let b: i32 = 25;
        }
        '5' => {
            /*crust in strict mod avoids declaring all variables as mutable.
             * If you are mutating any values anywhere in program please change the declaration statement as
             * let mut var_name:type=init_val;
             **/
            let b: i32 = 25;
        }
        _ => {}
    }
    /*crust in strict mod avoids declaring all variables as mutable.
     * If you are mutating any values anywhere in program please change the declaration statement as
     * let mut var_name:type=init_val;
     **/
    let a: i32;
    let b: i32;
    let c: i32;
    c = 100;
    b = c;
    a = b;
    a += 1;
    b += 1;
    c += 1;
    loop {
        printf(" infinite loop");
    }
    if 1 == 2 {
        loop {
            if a == 100 {
                break;
            }
            a += 1;
        }
    } else if 2 == 3 {
        l[i] = m[i] + n[i];
    } else {
        func();
    }
    /*crust in strict mod avoids declaring all variables as mutable.
     * If you are mutating any values anywhere in program please change the declaration statement as
     * let mut var_name:type=init_val;
     **/
    let i: i32 = 0;
    loop {
        l[i] = m[i] + n[i];
        i += 1;
    }
    /*crust in strict mod avoids declaring all variables as mutable.
     * If you are mutating any values anywhere in program please change the declaration statement as
     * let mut var_name:type=init_val;
     **/
    let i: i32 = 0;
    while i < 100 {
        l[i] = m[i] + n[i];
        i += 1;
    }
} // //some func
fn func() {
    // empty
    while a {
        //someshit
    }
}
