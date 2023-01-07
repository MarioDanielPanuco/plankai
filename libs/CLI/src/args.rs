pub use clap:: {
    Args,
    Subcommand,
    Parser,
};

#[command(author="Mario Daniel Panuco",
        version="1.0", about="PlankAI CLI Tool")]
#[derive(Debug, Parser)]
pub struct PlankArgs {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    // #[arg(short='t', long="train", value_parser)]
    /// Train the model for n iterations
    Train {
        iterations: Option<usize>
    },

    // #[arg(short='p', long="pause", value_parser)]
    Pause { },

    // #[arg(short, long, value_parser)]
    Reset {

    },
}

#[cfg(test)]
mod tests {
    #[test]
    fn parsing() {
        todo!()
    }
}