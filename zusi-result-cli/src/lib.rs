pub mod cli;

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use colored::Colorize;
use crate::cli::AnalyseFilesArgs;
use glob::{glob, PatternError};
use thiserror::Error;
use zusi_result_lib::result_analyser::{AnalyseError, PureAverageSpeedAlgorithm, ResultAnalyser};
use zusi_result_lib::result_analyser_group::{CreateAnalyserGroupError, ResultAnalyserGroup};
use zusi_xml_lib::xml::zusi::result::ZusiResult;
use zusi_xml_lib::xml::zusi::{DeError, FromXML, Zusi, ZusiValue};

#[derive(Error, Debug)]
pub enum AnalyseFilesError {
    #[error("The given pattern was invalid: {0}")]
    PatternError(#[from] PatternError),

    #[error("The given pattern was invalid: {0}")]
    PrintAnalysisError(#[from] PrintAnalysisError),
}

pub fn analyse_files(args: AnalyseFilesArgs) -> Result<(), AnalyseFilesError> {
    println!("Analyse files by pattern: {}", args.pattern);

    let mut results: Vec<ZusiResult> = vec![];

    for entry in glob(&args.pattern)? {
        match entry {
            Ok(path) => {
                match read_result(&path) {
                    Ok(result) => {
                        if args.debug {
                            println!("{:?}", path.display())
                        }
                        results.push(result);
                    }
                    Err(e) => eprintln!("{}", e.message(path)),
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }

    println!();
    print_analysis(results)?;
    Ok(())
}

#[derive(Error, Debug)]
enum ReadResultError {
    #[error("An IO error occoured: {0}")]
    IOError(#[from] io::Error),

    #[error("A deserialization error occoured: {0}")]
    DeError(#[from] DeError),

    #[error("The file must contain at least one result.")]
    NoResult,
}

impl ReadResultError {
    fn message(&self, path: PathBuf) -> String {
        match self {
            ReadResultError::IOError(e) => {
                format!("Error reading file '{:?}': {:?}", path, e)
            }
            ReadResultError::DeError(e) => {
                format!("Error during deserialization of '{:?}': {:?}", path, e)
            }
            ReadResultError::NoResult => {
                format!("The file '{:?}' does not contain a result.", path)
            }
        }
    }
}

fn read_result(path: &PathBuf) -> Result<ZusiResult, ReadResultError> {
    let mut input_file = File::open(path)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;
    let zusi = Zusi::from_xml(&contents)?;
    match zusi.value {
        ZusiValue::Result(result) => Ok(result),
        _ => Err(ReadResultError::NoResult)
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum PrintAnalysisError {
    #[error("Couldn't create the analyser group: {0}")]
    CreateAnalyserGroupError(#[from] CreateAnalyserGroupError),

    #[error("Couldn't analyse: {0}")]
    AnalyseError(#[from] AnalyseError),
}

fn print_analysis(results: Vec<ZusiResult>) -> Result<(), PrintAnalysisError> {
    if results.len() == 1 {
        let analyser = ResultAnalyser::new(results.first().unwrap());
        let schedule = analyser.schedule()?;

        println!("{}", "Schedule:".bold());
        println!();

        println!("{schedule}");
    }

    let analyser_group: ResultAnalyserGroup<_, _> = results.try_into()?;

    println!("{}", "Analysis results:".bold());
    println!();

    println!("total distance:          {} m", analyser_group.total_distance()?);

    println!("average distance:        {} m", analyser_group.average_distance()?);

    let average_speed = analyser_group.average_speed()?;
    println!("average speed:           {} m/s = {} km/h", average_speed, average_speed * 3.6);

    let pure_average_speed = analyser_group.pure_average_speed(PureAverageSpeedAlgorithm::default())?;
    println!("pure average speed:      {} m/s = {} km/h", pure_average_speed, pure_average_speed * 3.6);

    println!("total driving time:      {}", analyser_group.total_driving_time()?);

    println!("total pure driving time: {}", analyser_group.total_pure_driving_time()?);

    Ok(())
}
