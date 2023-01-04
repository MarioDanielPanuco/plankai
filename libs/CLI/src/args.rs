use clap::Parser;
use std::fmt::Formatter;

#[derive(Parser)]
#[clap(author = "Mario Daniel Panuco")]
#[clap(version = "0.2.1")]
#[clap(about = "PlankAI")]
#[clap(long_about = None)]
#[derive(Debug)]
pub struct Args {
    action: Action,
}

#[derive(Parser, Debug, Clone)]
pub enum Action {
    #[clap(short='t', long="train", value_parser)]
    Train(isize),

    #[clap(short='p', long="pause", value_parser)]
    Pause(String),

    #[clap(short='r', long="reset", value_parser)]
    Reset(isize),
}