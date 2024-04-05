use clap::Parser;
use std::path::PathBuf;


// Test parsing of the program argument parser using the `clap` library and the Args struct defined in src/main.rs
#[test]
fn test_parse_args() {


    // Define the input arguments
    let args = vec![
        "test",
        "-i",
        "infile.csv",
        "-o",
        "outfile.csv",
        "--row",
        "1,2,3",
        "--col",
        "1,2,3",
        "--value",
        "1,2,3",
        "--format",
        "1",
    ];

    // Parse the input arguments
    let parsed_args = Args::parse_from(args);

    // Define the expected output
    let expected_args = Args {
        infile: PathBuf::from("infile.csv"),
        outfile: PathBuf::from("outfile.csv"),
        row: vec!["1".to_string(), "2".to_string(), "3".to_string()],
        col: vec!["1".to_string(), "2".to_string(), "3".to_string()],
        value: vec!["1".to_string(), "2".to_string(), "3".to_string()],
        format: 1,
    };

    // Compare the parsed arguments to the expected output
    assert_eq!(parsed_args, expected_args);
}
