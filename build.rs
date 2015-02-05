#![feature(core, unicode, path, io, env)]

use std::env;
use std::old_io::{fs, File, BufferedReader};
use std::collections::HashMap;

fn main() {
    let target_dir      = Path::new(env::var_string("OUT_DIR").unwrap());
    let mut target_file = File::create(&target_dir.join("generated_glue.rs")).unwrap();

    let source_code_path = Path::new(file!()).join_many(&["..", "src/"]);

    let source_files = fs::readdir(&source_code_path).unwrap().into_iter()
        .filter(|path| {
            match path.str_components().last() {
                Some(Some(filename))  => filename.split('.').last() == Some("rs"),
                _                     => false
            }
        });

    let mut implementations = HashMap::new();

    for source_file_path in source_files {
        let relative_path = source_file_path.path_relative_from(&source_code_path).unwrap();
        let source_file_name = relative_path.as_str().unwrap();

        implementations.insert(source_file_name.to_string(), vec![]);
        let mut file_implementations = &mut implementations[*source_file_name];

        let mut source_file = BufferedReader::new(File::open(&source_file_path).unwrap());

        for line in source_file.lines() {
            let line_str = match line {
                Ok(line_str) => line_str,
                Err(_)       => break,
            };

            if line_str.starts_with("impl Methods for Methods_") {
                const PREFIX_LEN: usize = 25;

                let number_len = line_str[PREFIX_LEN..].chars().take_while(|chr| {
                    chr.is_digit(10)
                }).count();

                let number: i32 = line_str[PREFIX_LEN..(PREFIX_LEN + number_len)].parse().unwrap();
                file_implementations.push(number);
            }
        }
    }

    writeln!(&mut target_file, "use super::Methods;").unwrap();

    for (source_file_name, impls) in &implementations {
        let module_name = match source_file_name.split('.').next() {
            Some("main") => "super",
            Some(name)   => name,
            None         => panic!(),
        };

        for impl_number in impls {
            writeln!(&mut target_file, "use {}::Methods_{};", module_name, impl_number).unwrap();
        }
    }

    let all_impls = implementations.values().flat_map(|impls| impls.iter());

    writeln!(&mut target_file, "
pub struct Object;

impl Object {{
    pub fn new(impl_number: i32) -> Box<Methods + 'static> {{
        match impl_number {{
    ").unwrap();

    for impl_number in all_impls {
        writeln!(&mut target_file,
"           {} => Box::new(Methods_{}),", impl_number, impl_number).unwrap();
    }

    writeln!(&mut target_file, "
           _ => panic!(\"Unknown impl number: {{}}\", impl_number),
        }}
    }}
}}").unwrap();
}
