mod args;

use args::PlankArgs;
use clap::parser;

#[cfg(test)]
mod tests {
    mod args {
        use super::*;

        #[test]
        fn args() {

            // let args = PlankArgs.parse();
            //
            // for _ in 0..args.count {
            //     println!("Hello {}!", args.name)
            // }
        }
    }
}