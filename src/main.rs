use std::fmt;

struct AnalysisResult {
	six_count: u64,
	less_than_two_count: u64,
	even_count: u64,
}

impl fmt::Display for AnalysisResult {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Analysis Result: \
				   No. of 6: {} \
				   No. of <= 2: {} \
				   No. of even: {}"
		, self.six_count
		, self.less_than_two_count, 
		self.even_count)
	}
}

fn main() {
	let v: Vec<u64> = vec![1,2,3,4,5,6];
	
    let result = process_numbers(v);
	
	println!("{}", result.to_string());
}

fn process_numbers(numbers: Vec<u64>) -> AnalysisResult {
	let mut result = AnalysisResult {
		six_count: 0,
		less_than_two_count: 0,
		even_count: 0		
	};
	
	for curr in numbers {
		println!("Processint {}", curr);
		if curr % 2 == 0 {
			result.even_count += 1;
		}
		if curr <= 2 {
			result.less_than_two_count += 1;
		}
		if curr == 6 {
			result.six_count += 1;
		}		
	}
		
	return result;
}