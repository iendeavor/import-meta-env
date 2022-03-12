mod cli;

use anyhow::Result;
use clap::Parser;
use glob::glob;
use std::collections::HashMap;

fn main() -> Result<()> {
    let args = cli::Cli::parse();

    cli::assert_cli_arg(&args);

    let mut public_env_keys = Vec::new();
    #[allow(deprecated)]
    let example = dotenv::from_path_iter(std::path::Path::new(&args.example)).unwrap();
    for item in example {
        let (key, _) = item.unwrap();
        public_env_keys.push(key);
    }

    let mut public_env: HashMap<String, String> = HashMap::new();
    dotenv::from_path(std::path::Path::new(&args.env)).unwrap_or_default();
    for key in public_env_keys {
        match std::env::var(&key) {
            Ok(value) => {
                public_env.insert(key.clone(), value);
            }
            Err(_) => {}
        }
    }
    let serialized_public_env =
        serde_json::to_string(&public_env).expect("Failed to serialize public env");

    let placeholder_variants = [
        "\"__import_meta_env_placeholder__\"",
        "'__import_meta_env_placeholder__'",
    ];

    for placeholder in placeholder_variants {
        let arrow_function_re =
            regex::Regex::new(&format!("=>([\\s]*){}", &placeholder)).expect("Failed to new Regex");

        for output_glob in &args.output {
            for entry in glob(output_glob).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        if path
                            .clone()
                            .into_os_string()
                            .into_string()
                            .unwrap()
                            .ends_with(".bak")
                        {
                            continue;
                        }

                        let backup_path = format!(
                            "{}{}",
                            &path.clone().into_os_string().into_string().unwrap(),
                            ".bak"
                        );
                        // restore
                        if args.disposable == false {
                            let result = std::fs::read_to_string(&backup_path);
                            if result.is_ok() && result.unwrap().contains(placeholder) {
                                std::fs::copy(&backup_path, &path).unwrap_or_default();
                            }
                        }

                        // backup
                        let contents =
                            std::fs::read_to_string(&path).expect("Failed to read output file");
                        if args.disposable == false {
                            if contents.contains(placeholder) {
                                std::fs::write(&backup_path, &contents)
                                    .expect("Failed to backup file");
                            }
                        }

                        // populate
                        let contents = arrow_function_re
                            .replace_all(&contents, format!("=>$1({})", &serialized_public_env));
                        let contents = contents.replace(placeholder, &serialized_public_env);
                        std::fs::write(&path, &contents).expect("Failed to write output file");
                    }
                    Err(_) => {}
                }
            }
        }
    }

    Ok(())
}
