 fn main ( ) { 
/*Avoid using mutable variables unless it is necessary to do so
 */
 let mut i : i32 = 0 ; 
/*Avoid using mutable variables unless it is necessary to do so
 */
 let mut a : [i32;4] = [ 1 , 2 , 3 , 4 ] , b [ 4 ] = [ 1 , 2 , 3 , 4 ] ; 
/*Avoid using mutable variables unless it is necessary to do so
 */
 let mut c : [i32;4] ; while i < 4 { c [ a ] = a [ i ] + b [ i ] + 100 ; i +=1 ; } }