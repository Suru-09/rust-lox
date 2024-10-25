pub mod common {

    use nom::{
        bytes::complete::{tag, take_until},
        character::complete::char,
        combinator::rest,
        sequence::delimited,
        IResult,
    };
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    use std::process::Command;

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
                //println!("Expect Pattern Match: {}", expect.literal);
                test.expectancies.push(TestExpectTypes::Expect(expect));
            } else if let Ok((_, parse_err)) = parse_parser_error_pattern(&line) {
                // println!(
                //     "Parser Error Match: Location = '{}', Error Message = '{}'",
                //     parse_err.error_token, parse_err.error_message
                // );
                test.expectancies
                    .push(TestExpectTypes::ParseError(parse_err));
            } else if let Ok((_, runtime_err)) = parse_runtime_error_pattern(&line) {
                // println!("Runtime Error Pattern Match: {}", runtime_err.error_message);
                test.expectancies
                    .push(TestExpectTypes::RuntimeError(runtime_err));
            }
        }

        Ok(test)
    }

    pub fn interpret_file(path: &str) -> (String, String) {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("--src-path")
            .arg(path)
            .output()
            .expect("I should be able to interpret this file");

        let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8 in stderr");
        let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 in stdout");
        (stdout, stderr)
    }

    pub fn run_test(path: &str) -> bool {
        let test = match parse_file(path) {
            Ok(test) => test,
            Err(_) => return false,
        };

        let (stdout, stderr) = interpret_file(path);

        println!("Stdout: \n{}", stdout);
        println!("Stderr: \n{}", stderr);
        println!("Expectancies: \n{:?}", test.expectancies);

        // stderr might me full of cargo command output, doesn't mean an error necesarrily
        // errors are treated from stdout(we report them through error_handling).
        // if !stderr.is_empty() {
        //     return false;
        // }

        let lines: Vec<&str> = stdout.lines().collect();
        let expectancies = test.expectancies;

        if lines.len() != expectancies.len() {
            println!("The number of lines does not match the number of enum elements.");
            return false;
        }

        for (i, line) in lines.iter().enumerate() {
            match expectancies[i].clone() {
                TestExpectTypes::Expect(expect) => {
                    println!("Stdout line: {}\n", line);
                    println!("expect.literal: {}\n", expect.literal);
                    if expect.literal != *line {
                        return false;
                    }
                }
                TestExpectTypes::ParseError(parse_err) => {
                    println!("Stdout line: {}\n", line);
                    println!("parse_err.error_token: '{}' and message: {}\n", parse_err.error_token, parse_err.error_message);
                    if !(*line).contains(&parse_err.error_token) || !(*line).contains(&parse_err.error_message){
                        return false;
                    }
                },
                TestExpectTypes::RuntimeError(runtime_err) => {
                    println!("Stdout line: {}\n", line);
                    println!("runtime_err.error_message message: {}\n", runtime_err.error_message);
                    if !(*line).contains(&runtime_err.error_message){
                        return false;
                    }
                },
            }
        }

        true
    }

    #[macro_export]
    macro_rules! generate_integration_test {
        ($test_name:ident, $path:expr) => {
            #[test]
            fn $test_name() {
                let result = run_test($path);
                assert_eq!(result, true, "Test {} failed", stringify!($test_name));
            }
        };
    }
}
