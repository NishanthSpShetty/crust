int a=100;


int main(){
	if(a==100){
		/* this is inside if
		this comment 
		//shadow of a 
		alone panic's
		*/
		int a = 9999; 
		another(a);
	}
	/*hopefully if compiles*/
	println!("workd");
}

int another(int a){
println!(" got a : {}",a);
}
