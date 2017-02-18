static a: i32 = 100;
fn main() {
    if a == 100 {
        //shadow a
        let ab: i32 = 100087;
        let ab: i32 = 100087;
    }
}
fn another_func(ba: i32) -> i32 {
    println!(" Got a {} ", ba);
    return 100;
}
