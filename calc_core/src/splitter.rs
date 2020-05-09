use std::collections::VecDeque;


pub fn split(source: &str) ->VecDeque<&str> {
	
	let mut remain:usize=0;
	let mut flg:bool=false;
	let mut accum:VecDeque<&str> = VecDeque::new();

	fn is_delimiter(c:&char)->bool{*c==' '||*c=='\t'};


	for elem in source.trim().chars().chain(" ".chars()).enumerate(){

		match (remain,elem.0,is_delimiter(&elem.1),flg) {
			(_,_,true,true)=>{},
			(begin,current,true,false)=>{
				flg=true;
				if begin!=current {
					accum.push_back(&source[begin..current]);
				}
			},
			(_,current,false,true)=>{
				remain=current;
				flg=false;
			},
			(_,_,false,false)=>{},
		}
	};

	accum
}

#[cfg(test)]
mod tests {
	use super::split;
	#[test]
	fn splitter_test() {
		fn assert(source:&str,expected:Vec<&str>){
			let actual=split(source);
			assert_eq!(actual.len(),expected.len());

			for elem in actual.iter().zip(expected.iter()) {
				assert_eq!(elem.0,elem.1);
			}
		}

		assert("10 20 +",vec!["10","20","+"]);
		assert("10\t20\t+",vec!["10","20","+"]);
		assert("10    20     +",vec!["10","20","+"]);
		assert("-10 -200 +",vec!["-10","-200","+"]);
		assert("",vec![]);
		assert("         ",vec![]);
		assert("\t",vec![]);
	}
}