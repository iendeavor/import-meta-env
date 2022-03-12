use clap::Parser;
use glob::glob;

/// Populates your environment variables from the system or `.env` file.
#[derive(Parser)]
#[clap(version)]
pub struct Cli {
    /// The .env file path to load
    #[clap(short, long, default_value = ".env")]
    pub env: String,

    /// The .env example file path to load
    #[clap(short('x'), long)]
    pub example: String,

    /// The output globs to populate (in-place)
    #[clap(
        short,
        long,
        multiple_values = true,
        default_values = &["dist/**/*", ".next/**/*", ".nuxt/**/*", ".output/**/*", "build/**/*"]
    )]
    pub output: Vec<String>,

    /// Do not create backup files and restore from backup files. In local development, disable this option to avoid rebuilding the project when environment variable changes, In production, enable this option to avoid generating unnecessary backup files.
    #[clap(long)]
    pub disposable: bool,
}

pub fn assert_cli_arg(args: &Cli) {
    assert_example_arg(args);
    assert_output_arg(args);
}

fn assert_example_arg(args: &Cli) {
    std::fs::read_to_string(&args.example)
        .expect(&format!("Failed to load example file: {}", &args.example));
}

fn assert_output_arg(args: &Cli) {
    let mut matched_count = 0;
    for output_glob in &args.output {
        matched_count += glob(output_glob)
            .expect("Failed to read glob pattern")
            .count();
    }

    assert_ne!(
        matched_count,
        0,
        "Output file not found: {}",
        &args.output.join(", ")
    )
}
