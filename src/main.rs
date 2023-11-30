struct AnalysisResult {
	six_count: u64,
	less_than_two_count: u64,
	even_count: u64,
}

fn main() {
	let v: Vec<u64> = vec![1,2,3];
	
    let _result = process_numbers(v);	
}

fn process_numbers(_numbers: Vec<u64>) -> AnalysisResult {
	let result = AnalysisResult{
		six_count: 0,
		less_than_two_count: 0,
		even_count: 0		
	};
	
	return result;
}