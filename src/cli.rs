use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about)]
pub(crate) struct Cli {
    /// Nix flake to use
    #[arg(short, long)]
    pub flake: String,
    /// Flake input that will be used to provide the secret values
    #[arg(short, long, default_value = "kent")]
    pub input: String,
    /// Filter which secret are propagated.
    /// For example '-s metadata.name=foo -s metadata.name=bar,metadata.labels.bazz=true'.
    /// Multiple expressions are combined as following '-s a -s b,c' <==> 'a || (b && c)'
    #[arg(long, short, number_of_values = 1, default_value = "")]
    pub selector: Vec<String>,
    /// Namespace to extract secret values from. If not provided the currently active one will
    /// be used
    #[arg(long, short)]
    pub namespace: Option<String>,
    /// Nix command to run
    #[arg(long, short, default_value = "develop")]
    pub command: String,
}
