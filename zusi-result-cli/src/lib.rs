pub mod cli;

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use crate::cli::AnalyseFilesArgs;
use glob::{glob, PatternError};
use zusi_result_lib::result_analyser::{AnalyseError, PureAverageSpeedAlgorithm, ResultAnalyser};
use zusi_result_lib::result_analyser_group::{CreateAnalyserGroupError, ResultAnalyserGroup};
use zusi_xml_lib::xml::zusi::result::ZusiResult;
use zusi_xml_lib::xml::zusi::{DeError, Zusi, ZusiValue};

#[derive(Debug)]
pub enum AnalyseFilesError {
    PatternError(PatternError),
    PrintAnalysisError(PrintAnalysisError),
}

impl From<PatternError> for AnalyseFilesError {
    fn from(value: PatternError) -> Self {
        AnalyseFilesError::PatternError(value)
    }
}

impl From<PrintAnalysisError> for AnalyseFilesError {
    fn from(value: PrintAnalysisError) -> Self {
        AnalyseFilesError::PrintAnalysisError(value)
    }
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

#[derive(Debug)]
enum ReadResultError {
    IOError(io::Error),
    DeError(DeError),
    NoResult,
}

impl From<io::Error> for ReadResultError {
    fn from(value: io::Error) -> Self {
        ReadResultError::IOError(value)
    }
}

impl From<DeError> for ReadResultError {
    fn from(value: DeError) -> Self {
        ReadResultError::DeError(value)
    }
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
    for value in zusi.value {
        if let ZusiValue::Result(result) = value {
            return Ok(result);
        }
    }
    Err(ReadResultError::NoResult)
}

#[derive(Debug)]
pub enum PrintAnalysisError {
    CreateAnalyserGroupError(CreateAnalyserGroupError),
    AnalyseError(AnalyseError),
}

impl From<AnalyseError> for PrintAnalysisError {
    fn from(value: AnalyseError) -> Self {
        PrintAnalysisError::AnalyseError(value)
    }
}

impl From<CreateAnalyserGroupError> for PrintAnalysisError {
    fn from(value: CreateAnalyserGroupError) -> Self {
        PrintAnalysisError::CreateAnalyserGroupError(value)
    }
}

fn print_analysis(results: Vec<ZusiResult>) -> Result<(), PrintAnalysisError> {
    if results.len() == 1 {
        let mut analyser = ResultAnalyser::new(results.first().unwrap());
        let schedule = analyser.schedule()?;

        println!("Schedule:");
        println!();

        println!("{schedule}");
    }

    let mut analyser_group: ResultAnalyserGroup<_, _> = results.try_into()?;

    println!("Analysis results:");
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
