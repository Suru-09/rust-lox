pub mod common {

    use nom::{
        bytes::complete::{tag, take_until},
        character::complete::char,
        combinator::rest,
        sequence::delimited,
        IResult,
    };
    use std::io::{self, BufRead};
    use std::path::Path;
    use std::{collections::HashMap, fs::File};
    use walkdir::WalkDir;

    pub const INTEGRATION_TESTS_PATH: &str = "tests/resources/integration_tests";
    pub const EXPECT_PATTERN: &str = "// expect:";
    pub const PARSER_ERROR_PATTERN: &str = "// Error at ";
    pub const RUNTIME_ERROR_PATTERN: &str = "// expect runtime error:";

    #[derive(Clone, Debug)]
    pub struct Test {
        pub expectancies: Vec<TestExpectTypes>,
    }

    impl Test {
        pub fn new() -> Self {
            Test {
                expectancies: Vec::new(),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum TestExpectTypes {
        Expect(Expect),
        ParseError(ParseError),
        RuntimeError(RuntimeError),
    }

    #[derive(Clone, Debug)]
    pub struct Expect {
        pub literal: String,
    }

    #[derive(Clone, Debug)]
    pub struct ParseError {
        pub error_token: String,
        pub error_message: String,
    }

    #[derive(Clone, Debug)]
    pub struct RuntimeError {
        pub error_message: String,
    }

    fn parse_expect_pattern(input: &str) -> IResult<&str, Expect> {
        let (input, _) = take_until(EXPECT_PATTERN)(input)?;
        let (input, _) = tag(EXPECT_PATTERN)(input)?;
        let (input, expected_literal) = rest(input)?;
        Ok((
            input,
            Expect {
                literal: expected_literal.trim().to_string(),
            },
        ))
    }

    fn parse_parser_error_pattern(input: &str) -> IResult<&str, ParseError> {
        let (input, _) = take_until(PARSER_ERROR_PATTERN)(input)?;
        let (input, _) = tag(PARSER_ERROR_PATTERN)(input)?;
        let (input, location) = delimited(char('\''), take_until("'"), char('\''))(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, error_message) = rest(input)?;
        Ok((
            input,
            ParseError {
                error_token: location.trim().to_string(),
                error_message: error_message.trim().to_string(),
            },
        ))
    }

    fn parse_runtime_error_pattern(input: &str) -> IResult<&str, RuntimeError> {
        let (input, _) = take_until(RUNTIME_ERROR_PATTERN)(input)?;
        let (input, _) = tag(RUNTIME_ERROR_PATTERN)(input)?;
        let (input, result) = rest(input)?;
        Ok((
            input,
            RuntimeError {
                error_message: result.trim().to_string(),
            },
        ))
    }

    fn parse_file<P: AsRef<Path>>(filename: P) -> Result<Test, String> {
        let file = match File::open(filename) {
            Ok(file) => file,
            Err(err) => return Err(err.to_string()),
        };

        let reader = io::BufReader::new(file);
        let mut test: Test = Test::new();

        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(err) => return Err(err.to_string()),
            };

            if let Ok((_, expect)) = parse_expect_pattern(&line) {
                println!("Expect Pattern Match: {}", expect.literal);
                test.expectancies.push(TestExpectTypes::Expect(expect));
            } else if let Ok((_, parse_err)) = parse_parser_error_pattern(&line) {
                println!(
                    "Parser Error Match: Location = '{}', Error Message = '{}'",
                    parse_err.error_token, parse_err.error_message
                );
                test.expectancies
                    .push(TestExpectTypes::ParseError(parse_err));
            } else if let Ok((_, runtime_err)) = parse_runtime_error_pattern(&line) {
                println!("Runtime Error Pattern Match: {}", runtime_err.error_message);
                test.expectancies
                    .push(TestExpectTypes::RuntimeError(runtime_err));
            }
        }

        Ok(test)
    }

    pub fn get_all_lox_files(path: &str) -> Vec<String> {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.metadata().map(|m| m.is_file()).unwrap_or(false))
            .filter_map(|entry| {
                let path = entry.path();
                let extension = path.extension()?.to_str()?;
                if extension == "lox" {
                    Some(path.to_str()?.to_string())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn parse_tests(path: &str) -> HashMap<String, Test> {
        let mut tests: HashMap<String, Test> = HashMap::new();
        for filename in get_all_lox_files(path) {
            if !filename.contains("undefined.lox") {
                continue;
            }

            match parse_file(filename.clone()) {
                Ok(test) => {
                    tests.insert(filename, test);
                }
                Err(e) => {
                    println!("Error reading file: {}", e);
                    continue;
                }
            }
        }
        tests
    }

    macro_rules! generate_integration_test {
        ($test_name:ident, $path:expr) => {
            #[test]
            fn $test_name() {
                // Assuming `my_function` is the function you want to test, and it takes a &str as an argument.
                let result = my_function($path);
                // Define the expected result for the given test (you can customize this per test).
                let expected_result = "some_expected_result"; // Replace this with appropriate logic.
                assert_eq!(
                    result,
                    expected_result,
                    "Test {} failed",
                    stringify!($test_name)
                );
            }
        };
    }
}
