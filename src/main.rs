// use regex::Regex;

mod typeOf {
    use std::any::type_name;

    pub fn value<T>(_: &T) -> &'static str {
        type_name::<T>()
    }
    
    pub fn print<T>(item: &T) {
        println!("Type:  {}", value(item));
    }
    
}

fn main() {
    use compiler::{parse, find_variables};
    println!("Hello, world!");
    let program = String::from("let var1 = 2;\n//Comment\n/*\nMulti\nLine\nComment*/let var2 = 3;");
    let cleaned = clean(program);
    
    // println!("Program: {}", program);

    println!("Parsed Result:");
    // for line in parsed.iter() {
    //     println!("{}", line);
    // }
    println!("{}", cleaned);

    find_variables(&cleaned);
}

mod compiler {
    enum Scope {
        Function(&String),
        Global(&String),
        Loop(&String),
        None(&String),
    }
    use std::borrow::Cow;
    pub fn clean(file: String) -> String {        
        // println!("Parsed sees: '{}'", &file);
        let multi_line_comment = regex::Regex::new(r"(?ms)/\*.*\*/").unwrap();      
        let single_line_comment = regex::Regex::new(r"(?m)//.*\n").unwrap();
        let new_line = regex::Regex::new(r"(?m)\n").unwrap();       
        // single_line_comment.split(file).into_iter().collect()
        let file = multi_line_comment.replace(&file, "");
        let file = single_line_comment.replace(&file, "");
        let file = match file {
            Cow::Owned(string) => string,
            Cow::Borrowed(string) => string.to_string(),
        };
        file
    }

    pub fn find_variables(file: &String) -> Vec<&str> {
        let declarations = regex::Regex::new(r"(?ms)let (.*?) = (.*?);").unwrap();
        let capture_groups = declarations.captures_iter(file);
        for capture in capture_groups {
            for item in capture.iter() {
                println!("{:#?}", item);
                // let res = item.unwrap();
                // println!("{:#?}", res.as_str());
            }
        }
        // capture_groups.collect()
        let res : Vec<&str> = Vec::new();
        res
    }

    // pub fn process(file: &String) {
    //     let type = scope_type(file);
    // }

    // fn scope_type(block: &String) -> Scope {
    //     let block_format = regex::Regex::new(r"(?ms).*{.*}").unwrap();
    //     if (block_format.is_match(block)) {
    //         Scope::Function(block)
    //     } else {
    //         Scope::None(block)
    //     }
    // }
}