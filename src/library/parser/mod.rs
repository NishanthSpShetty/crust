use library::lexeme::Type::*;
use library::lexeme::Token;
pub fn parse_if(lexeme:Vec<Token>)->Vec<String>{
	let mut i:usize =0;
	let mut cond:String = String::new();
	let mut stream:Vec<String> = Vec::new();

	while i< lexeme.len(){
		match lexeme[i].get_token_type() {
			KEYWORD_IF => stream.push("if".to_string()),
			LEFT_BRACKET => 
				{
					i+=1;
					while lexeme[i].get_token_type() != RIGHT_BRACKET {
						cond = cond+ &lexeme[i].get_token_value()[..];
						i+=1;
//						println!(" -{} ",i);
					}
					stream.push(String::from(&cond[..]));
					
					stream.push("{\n".to_string());

				},
			LEFT_CBRACE => {
					}
			RIGHT_CBRACE => {
				}
			
			_ => stream.push(lexeme[i].get_token_value()),
			}
//		println!("{}",cond);
		i+=1;
		}
		
				stream.push("\n}".to_string());
	stream


}
