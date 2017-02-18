static a: i32 = 100;
fn main() {
    if a == 100 {
        /* this is inside if
		this comment 
		//shadow of a 
		alone panic's
		*/
        let a: i32 = 9999;
        let a: i32 = 9999;
    } /*hopefully if compiles*/
    println!("workd");
}
fn another(a: i32) -> i32 {
    println!(" got a : {}", a);
}
