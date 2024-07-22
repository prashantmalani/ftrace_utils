// Code to take the input of FTRACE trace_stats from multiple CPUs and consolidate them into 1.
// This code was generated using Gemini, and then adapted/modified to suite my output requirements.
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{multispace1},
    combinator::{map_res, opt},
    sequence::{terminated},
    IResult,
};

use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use pad::PadStr;

#[derive(Debug, PartialEq)]
struct FunctionStats {
    function_name: String,
    hit_count: u64,
    time_total: f64, // microseconds (us)
    time_avg: f64,    // microseconds (us)
    time_variance: f64, // microseconds (us)
}

fn parse_function_name(input: &str) -> IResult<&str, String> {
    map_res(
        take_while1(|c: char| !c.is_whitespace()), // Parse the substring
        |s: &str| s.parse(), // Convert &str to String
    )(input)
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(take_while1(|c: char| c.is_digit(10)), |s: &str| s.parse())(input)
}

fn parse_f64(input: &str) -> IResult<&str, f64> {
    map_res(take_while1(|c: char| c.is_digit(10) || c == '.'), |s: &str| s.parse())(input)
}

fn parse_line(input: &str) -> IResult<&str, FunctionStats> {
    let (input, function_name) = parse_function_name(input)?;
    let (input, _) = multispace1(input)?;
    let (input, hit_count) = parse_u64(input)?;
    let (input, _) = multispace1(input)?;
    let (input, time_total) = terminated(parse_f64, tag(" us"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, time_avg) = parse_f64(input)?;
    let (input, _) = multispace1(input)?;
    let (input, time_variance) = opt(parse_f64)(input)?;
    let time_variance = time_variance.unwrap_or_default(); // Default to 0 if missing

    Ok((
        input,
        FunctionStats {
            function_name,
            hit_count,
            time_total,
            time_avg,
            time_variance,
        },
    ))
}

fn process_file(file_path: &Path) -> Result<HashMap<String, FunctionStats>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut function_stats = HashMap::new();

    for (line_num, line_result) in reader.lines().enumerate().skip(2) {
        let line = line_result?;
        match parse_line(line.trim_start()) {
            Ok((_, stats)) => {
                function_stats
                    .entry(stats.function_name.clone())
                    .and_modify(|e: &mut FunctionStats| {
                        e.hit_count += stats.hit_count;
                        e.time_total += stats.time_total;
                    })
                    .or_insert(stats);
            }
            Err(err) => eprintln!("Error parsing line {}: {:?}", line_num + 1, err),
        }
    }
    Ok(function_stats)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file1> <file2> ...", args[0]);
        std::process::exit(1);
    }

    let mut consolidated_stats = HashMap::new();

    for file_path in &args[1..] {
        let file_stats = process_file(Path::new(file_path))?;
        for (function_name, stats) in file_stats {
            consolidated_stats
                .entry(function_name)
                .and_modify(|e: &mut FunctionStats| {
                    e.hit_count += stats.hit_count;
                    e.time_total += stats.time_total;
                })
                .or_insert(stats);
        }
    }
    
    // Calculate Average and Variance after Consolidation
    for (_, stats) in consolidated_stats.iter_mut() {
        stats.time_avg = stats.time_total as f64 / stats.hit_count as f64;
        // To calculate variance, you'll need to store more data 
        // during the initial parsing and aggregation.
        // We skip it here for brevity, but let me know if you need it. 
    }
    
    println!("{} {} {} {}",
        "Function name".pad_to_width(40),
        "Hit Count".pad_to_width(20),
        "Time Total".pad_to_width(20),
        "Avg".pad_to_width(20));
    // Print Consolidated Results
    for (function_name, stats) in consolidated_stats {
        println!(
            "{} {} {} {}",
            function_name.pad_to_width(40), stats.hit_count.to_string().pad_to_width(20),
            (stats.time_total.to_string() + "us").pad_to_width(20),
            (stats.time_avg.to_string() + "us").pad_to_width(20)
        ); 
    }

    Ok(())
}