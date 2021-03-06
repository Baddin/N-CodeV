


trait OOP {

	fn get_classes(code: &String) -> Vec<String /* later it will be "class" struct*/>;

	fn get_objects(code: &String) -> Vec<String>;
}

trait Functional {
	
	unimplemented!();
}

trait Language {

	fn get_functions(code: &String) -> Vec<String>;

	fn get_variables(code: &String) -> Vec<String>;
}




impl Language for PythonCodeParser {

	fn get_functions(code: &String) -> Vec<String>{
		unimplemented!();
	}

	fn get_variables(code: &String) -> Vec<String>{
		unimplemented!();
	}
}


impl OOP for PythonCodeParser {
	fn get_classes(code: &String) -> Vec<String /* later it will be "class" struct*/>{
		unimplemented!();
	}

	fn get_objects(code: &String) -> Vec<String> {
		unimplemented!()
	}
}

#[derive(Debug)]
struct PythonCodeParser {
	code: String,
}


#[derive(Debug)]
struct File {
	file_name: String,
	file_path: String, // for now
	file_type: String // ex: html, pdf..
}

#[derive(Debug)]
struct CodeStruct {
	functions: Vec<String>,
	variables: Vec<String>,
}

impl CodeStruct {
	fn draw(&self) -> Result<File, String> {
		unimplemented!();
	}

	//fn link_code(&self)
}


//TODO: abstract that code
fn gen_code_structure<T: Language>(language_parser: T) -> CodeStruct{
	parser = language_parser::new();
	let functions = parser.get_functions();
	let variables = parser.get_variables();
	//TODO: get namespaces and link the related code
	//TODO: add more python tokens
	

	CodeStruct {
		functions: functions,
		variables: variables,
	}
}


fn main() {
	println!("Nothing implemented yet :)");
}
