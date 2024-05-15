// Read a table (from a text file) of data in normalized form and cross-tab it,
// allowing multiple data columns to be crosstabbed.

// use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use polars::prelude::*;

/// Read a table (from a text file) of data in normalized form and cross-tab it,
/// allowing multiple data columns to be crosstabbed.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Required arguments

    #[arg(short, long, required = true, help = "The name of the input file from which to read data. This must be a text file, with data in a normalized format. The first line of the file must contain column names.")]
    infile: std::path::PathBuf,
    #[arg(short, long, required = true, help="The name of the output file to create. The output file will be created as a .csv file.")]
    outfile: std::path::PathBuf,
    #[arg(short, long, required = true, help = "A comma-separated list of one or more column names to use as row headers in the crosstab. Unique values of these columns will appear at the beginning of every output line.")]
    row_headers: Vec<String>,
    #[arg(short, long, required = true, help="A comma-separated list of one or more column names to use as column headers in the crosstab. Unique values of these columns will appear at the beginning of every output line.")]
    col_headers: Vec<String>,
    #[arg(short, long, required = true, help="One or more column names with values to be used to fill the cells of the cross-table.  If n columns names are specified, then there will be n columns in the output table for each of the column headers corresponding to values of the -c argument.  The column names specified with the -v argument will be appended to the output column headers created from values of the -c argument.  There should be only one value of the -v column(s) for each combination of the -r and -c columns; if there is more than one, a warning will be printed and only the first value will appear in the output.  (That is, values are not combined in any way when there are multiple values for each output cell.)")]
    values: Vec<String>,

    // Optional arguments

    #[arg(short, long, default_value = "1", help="Controls the format of the column headers. The four possible values are: 1) One row of column headers, with elements joined by underscores to facilitate parsing by other programs; 2) Two rows of column headers.  The first row contains values of the columns specified by the -c argument, and the second row contains the column names specified by the -v argument; 3) One header row for each of the values of the columns specified by the -c argument, plus one row with the column names specified by the -v argument; 4) Like 3, but the values of the columns specified by the -c argument are labeled with (preceded by) the column names.")]
    format: u8,
}


fn main() {
    let args = Args::parse();

    // Print all of the arguments
    println!("Before processing arguments:");
    println!("  infile: {}", args.infile.display());
    println!("  outfile: {}", args.outfile.display());
    println!("  row_headers: {:?}", args.row_headers);
    println!("  col_headers: {:?}", args.col_headers);
    println!("  cell_values: {:?}", args.values);
    println!("  format: {}", args.format);

    // Store the input file as path string. We will read from the file at a later step.
    // If the file does not exist, print an error message and exit the program
    let infile: PathBuf = args.infile;
    if !&infile.exists() {
        println!("Error: The input file does not exist: {}", &infile.display());
        std::process::exit(1);
    }
    // Store the output file as a string. We will write to the file using a buffered writer at a later step
    let outfile: String = args.outfile.to_str().unwrap().to_string();
    // Check if the output file is a .csv file. If it is not, print an error message and exit the program
    if !&outfile.ends_with(".csv") {
        println!("Error: The output file must be a .csv file: {}", &outfile);
        std::process::exit(1);
    }
    // Split the row vector by commas and assign each element to a new vector
    let row_headers: Vec<&str> = args.row_headers[0].split(',').collect();
    let col_headers: Vec<&str> = args.col_headers[0].split(',').collect();
    let cell_values: Vec<&str> = args.values[0].split(',').collect();
    // Convert the format argument from a string to an i8 integer. If the value cannot be converted, print an error message
    let format: i8 = match args.format {
        1 => 1,
        // Not implemented
        // 2 => 2,
        // 3 => 3,
        // 4 => 4,
        _ => {
            // println!("Error: The format argument must be an integer between 1 and 4");
            println!("Error: The format argument must be 1");
            std::process::exit(1);
        }
    };

    // Print all of the formatted arguments
    println!("After processing arguments:");
    println!("  infile: {}", infile.display());
    println!("  outfile: {}", outfile);
    println!("  row_headers: {:?}", row_headers);
    println!("  col_headers: {:?}", col_headers);
    println!("  cell_values: {:?}", cell_values);
    println!("  format: {}", format);

    xtab(infile, outfile, row_headers, col_headers, cell_values, format);
}

fn read_csv(file: PathBuf) -> PolarsResult<DataFrame> {
    // Read the input file into a DataFrame.
    CsvReader::from_path(&file)?
            .has_header(true)
            .finish()
}

fn xtab(infile: PathBuf, outfile: String, row_headers: Vec<&str>, col_headers: Vec<&str>, cell_values: Vec<&str>, format: i8) {
    // Create the crosstab.

    // Create a boolean to check if there are multiple values for each output cell.
    let mut multiple_vals: bool = false;
    // Create boolean to flag if there are any reportable errors.
    let mut reportable_errors: bool = false;

    // Read the input file into a DataFrame.
    // If there is an issue reading the file, print an error message and exit the program
    let mut df = DataFrame::empty();
    match read_csv(infile) {
        Ok(x) => df = x,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Print the DataFrame
    println!("{:?}", df);

    // Print the column names from the dataframe
    let col_names = df.get_column_names();

    // Check if the row headers are in the DataFrame. If they are not, print a generic error message and exit the program
    for i in 0..row_headers.len() {
        println!("{:?}", row_headers[i]);
        // if !df.columns().iter().any(|x| x.name() == row_headers[i]) {
        //     println!("Error: The row header column {} is not in the input file", row_headers[i]);
        //     reportable_errors = true;
        // }
    }

    for i in 0..col_headers.len() {
        println!("{:?}", col_headers[i]);
    }

    // Write the header row to the output file
    let mut header_row: Vec<String> = Vec::new();
    if format == 1 {
        // Append the row headers to the header row vector
        for i in 0..row_headers.len() {
            header_row.push(row_headers[i].to_string());
        }
        // Combine the row and column headers with an underscore and append to the header row vector
        for i in 0..col_headers.len() {
            for j in 0..cell_values.len() {
                header_row.push(format!("{}_{}", col_headers[i], cell_values[j]));
            }
        }
    }

    println!("{:?}", header_row);
    // Write the header row to the output file
    let mut writer = csv::Writer::from_path(outfile).unwrap();
    writer.write_record(&header_row).unwrap();

}
