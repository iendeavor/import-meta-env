use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use insta::assert_snapshot;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn cli_help() -> Result<(), Box<dyn std::error::Error>> {
    // act
    let output = Command::cargo_bin("import-meta-env")?
        .arg("--help")
        .output()
        .unwrap();

    // assert
    assert!(output.status.success(), "--help returns 0");
    assert_eq!(output.stderr, Vec::<u8>::new(), "--help stderr is empty");
    assert_snapshot!("help", String::from_utf8(output.stdout).unwrap());

    Ok(())
}

#[test]
fn example_arg_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    // act
    let mut cmd = Command::cargo_bin("import-meta-env")?;

    // assert
    cmd.assert().failure();

    Ok(())
}

#[test]
fn example_file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    // act
    let mut cmd = Command::cargo_bin("import-meta-env")?;
    cmd.arg("--example").arg("file-does-not-exist");

    // assert
    cmd.assert().failure().stderr(predicate::str::contains(
        "Failed to load example file: file-does-not-exist",
    ));

    Ok(())
}

#[test]
fn output_contains_invalid_glob_pattern() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let example_file = assert_fs::NamedTempFile::new(".env.example")?;
    example_file.write_str("FOO=")?;

    // act
    let mut cmd = Command::cargo_bin("import-meta-env")?;
    cmd.arg("--example")
        .arg(example_file.path())
        .arg("--output")
        .arg("***");

    // assert
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read glob pattern"));

    Ok(())
}

#[test]
fn output_file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let example_file = assert_fs::NamedTempFile::new(".env.example")?;
    example_file.write_str("FOO=")?;

    // act
    let mut cmd = Command::cargo_bin("import-meta-env")?;
    cmd.arg("--example")
        .arg(example_file.path())
        .arg("--output")
        .arg("foo/**")
        .arg("bar/**");

    // assert
    cmd.assert().failure().stderr(predicate::str::contains(
        "Output file not found: foo/**, bar/**",
    ));

    Ok(())
}

#[test]
fn populate_environment_variables() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let example_file = assert_fs::NamedTempFile::new(".env.example")?;
    example_file.write_str("FOO=")?;
    let output_file = assert_fs::NamedTempFile::new("index.js")?;
    output_file.write_str("const foo = \"__import_meta_env_placeholder__\".FOO;")?;

    // act
    let mut cmd = Command::cargo_bin("import-meta-env")?;
    cmd.arg("--example")
        .arg(example_file.path())
        .arg("--output")
        .arg(&output_file.path());

    // assert
    cmd.assert().success();
    assert_eq!(
        std::fs::read_to_string(output_file.path())?,
        "const foo = {}.FOO;"
    );

    Ok(())
}

#[test]
fn populate_environment_variables_from_system() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    std::env::set_var("FOO", "bar");
    let example_file = assert_fs::NamedTempFile::new(".env.example")?;
    example_file.write_str("FOO=123")?;
    let output_file = assert_fs::NamedTempFile::new("index.js")?;
    output_file.write_str("const foo = \"__import_meta_env_placeholder__\".FOO;")?;

    // act
    let mut cmd = Command::cargo_bin("import-meta-env")?;
    cmd.arg("--example")
        .arg(example_file.path())
        .arg("--output")
        .arg(&output_file.path());

    // assert
    cmd.assert().success();
    assert_eq!(
        std::fs::read_to_string(output_file.path())?,
        "const foo = {\"FOO\":\"bar\"}.FOO;"
    );

    std::env::remove_var("FOO");
    Ok(())
}

#[test]
fn populate_environment_variables_from_env_file() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let env_file = assert_fs::NamedTempFile::new(".env.example")?;
    env_file.write_str("FOO=baz")?;
    let example_file = assert_fs::NamedTempFile::new(".env.example")?;
    example_file.write_str("FOO=123")?;
    let output_file = assert_fs::NamedTempFile::new("index.js")?;
    output_file.write_str("const foo = \"__import_meta_env_placeholder__\".FOO;")?;

    // act
    let mut cmd = Command::cargo_bin("import-meta-env")?;
    cmd.arg("--env")
        .arg(env_file.path())
        .arg("--example")
        .arg(example_file.path())
        .arg("--output")
        .arg(&output_file.path());

    // assert
    cmd.assert().success();
    assert_eq!(
        std::fs::read_to_string(output_file.path())?,
        "const foo = {\"FOO\":\"baz\"}.FOO;"
    );

    Ok(())
}

#[test]
fn populate_environment_variables_for_single_quotes() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let env_file = assert_fs::NamedTempFile::new(".env.example")?;
    env_file.write_str("FOO=baz")?;
    let example_file = assert_fs::NamedTempFile::new(".env.example")?;
    example_file.write_str("FOO=123")?;
    let output_file = assert_fs::NamedTempFile::new("index.js")?;
    output_file.write_str("const foo = \'__import_meta_env_placeholder__\'.FOO;")?;

    // act
    let mut cmd = Command::cargo_bin("import-meta-env")?;
    cmd.arg("--env")
        .arg(env_file.path())
        .arg("--example")
        .arg(example_file.path())
        .arg("--output")
        .arg(&output_file.path());

    // assert
    cmd.assert().success();
    assert_eq!(
        std::fs::read_to_string(output_file.path())?,
        "const foo = {\"FOO\":\"baz\"}.FOO;"
    );

    Ok(())
}

#[test]
fn populate_all_environment_variables() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let env_file = assert_fs::NamedTempFile::new(".env.example")?;
    env_file.write_str("FOO=foo")?;
    let example_file = assert_fs::NamedTempFile::new(".env.example")?;
    example_file.write_str("FOO=123")?;
    let output_file = assert_fs::NamedTempFile::new("index.js")?;
    output_file.write_str(
        "
            const foo1 = \'__import_meta_env_placeholder__\'.FOO;
            const foo2 = \'__import_meta_env_placeholder__\'.FOO;
        ",
    )?;

    // act
    let mut cmd = Command::cargo_bin("import-meta-env")?;
    cmd.arg("--env")
        .arg(env_file.path())
        .arg("--example")
        .arg(example_file.path())
        .arg("--output")
        .arg(&output_file.path());

    // assert
    cmd.assert().success();
    assert_eq!(
        std::fs::read_to_string(output_file.path())?,
        "
            const foo1 = {\"FOO\":\"foo\"}.FOO;
            const foo2 = {\"FOO\":\"foo\"}.FOO;
        "
    );

    Ok(())
}

#[test]
fn populate_environment_variables_for_arrow_functions() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let env_file = assert_fs::NamedTempFile::new(".env.example")?;
    env_file.write_str("FOO=foo")?;
    let example_file = assert_fs::NamedTempFile::new(".env.example")?;
    example_file.write_str("FOO=123")?;
    let output_file = assert_fs::NamedTempFile::new("index.js")?;
    output_file.write_str(
        "
            ()=>'__import_meta_env_placeholder__'.FOO;
            () =>

                '__import_meta_env_placeholder__'.FOO;
        ",
    )?;

    // act
    let mut cmd = Command::cargo_bin("import-meta-env")?;
    cmd.arg("--env")
        .arg(env_file.path())
        .arg("--example")
        .arg(example_file.path())
        .arg("--output")
        .arg(&output_file.path());

    // assert
    cmd.assert().success();
    assert_eq!(
        std::fs::read_to_string(output_file.path())?,
        "
            ()=>({\"FOO\":\"foo\"}).FOO;
            () =>

                ({\"FOO\":\"foo\"}).FOO;
        ",
    );

    Ok(())
}

#[test]
fn backup_if_disposable_flag_disabled() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let env_file = assert_fs::NamedTempFile::new(".env.example")?;
    env_file.write_str("FOO=baz")?;
    let example_file = assert_fs::NamedTempFile::new(".env.example")?;
    example_file.write_str("FOO=123")?;
    let output_file = assert_fs::NamedTempFile::new("index.js")?;
    output_file.write_str("const foo = \"__import_meta_env_placeholder__\".FOO;")?;
    let backup_output_file_path = {
        let backup_output_file_path = format!("{:?}{}", &output_file.path(), ".bak");
        backup_output_file_path.replace("\"", "")
    };

    // act
    let mut cmd = Command::cargo_bin("import-meta-env")?;
    cmd.arg("--env")
        .arg(env_file.path())
        .arg("--example")
        .arg(example_file.path())
        .arg("--output")
        .arg(&output_file.path());

    // assert
    cmd.assert().success();
    assert_eq!(
        std::fs::read_to_string(output_file.path())?,
        "const foo = {\"FOO\":\"baz\"}.FOO;"
    );
    assert_eq!(
        std::fs::read_to_string(backup_output_file_path)?,
        "const foo = \"__import_meta_env_placeholder__\".FOO;"
    );

    Ok(())
}

#[test]
fn restore_if_disposable_flag_disabled() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let env_file = assert_fs::NamedTempFile::new(".env.example")?;
    env_file.write_str("FOO=baz")?;
    let example_file = assert_fs::NamedTempFile::new(".env.example")?;
    example_file.write_str("FOO=123")?;
    let output_file = assert_fs::NamedTempFile::new("index.js")?;
    output_file.write_str("const foo = {\"FOO\":\"bar\"}.FOO;")?;
    let backup_output_file_path = {
        let backup_output_file_path = format!("{:?}{}", &output_file.path(), ".bak");
        backup_output_file_path.replace("\"", "")
    };
    std::fs::write(
        &backup_output_file_path,
        "const foo = \"__import_meta_env_placeholder__\".FOO;",
    )
    .unwrap();

    // act
    let mut cmd = Command::cargo_bin("import-meta-env")?;
    cmd.arg("--env")
        .arg(env_file.path())
        .arg("--example")
        .arg(example_file.path())
        .arg("--output")
        .arg(&output_file.path());

    // assert
    cmd.assert().success();
    assert_eq!(
        std::fs::read_to_string(output_file.path())?,
        "const foo = {\"FOO\":\"baz\"}.FOO;"
    );
    assert_eq!(
        std::fs::read_to_string(backup_output_file_path)?,
        "const foo = \"__import_meta_env_placeholder__\".FOO;"
    );

    Ok(())
}

#[test]
fn try_to_restore_if_disposable_flag_disabled() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let env_file = assert_fs::NamedTempFile::new(".env.example")?;
    env_file.write_str("FOO=baz")?;
    let example_file = assert_fs::NamedTempFile::new(".env.example")?;
    example_file.write_str("FOO=123")?;
    let output_file = assert_fs::NamedTempFile::new("index.js")?;
    output_file.write_str("const foo = \"__import_meta_env_placeholder__\".FOO;")?;
    let backup_output_file_path = {
        let backup_output_file_path = format!("{:?}{}", &output_file.path(), ".bak");
        backup_output_file_path.replace("\"", "")
    };

    // act
    let mut cmd = Command::cargo_bin("import-meta-env")?;
    cmd.arg("--env")
        .arg(env_file.path())
        .arg("--example")
        .arg(example_file.path())
        .arg("--output")
        .arg(&output_file.path());

    // assert
    cmd.assert().success();
    assert_eq!(
        std::fs::read_to_string(output_file.path())?,
        "const foo = {\"FOO\":\"baz\"}.FOO;"
    );
    assert_eq!(
        std::fs::read_to_string(backup_output_file_path)?,
        "const foo = \"__import_meta_env_placeholder__\".FOO;"
    );

    Ok(())
}

#[test]
fn do_not_try_to_restore_and_backup_if_disposable_flag_enabled(
) -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let env_file = assert_fs::NamedTempFile::new(".env.example")?;
    env_file.write_str("FOO=baz")?;
    let example_file = assert_fs::NamedTempFile::new(".env.example")?;
    example_file.write_str("FOO=123")?;
    let output_file = assert_fs::NamedTempFile::new("index.js")?;
    output_file.write_str("const foo = \"__import_meta_env_placeholder__\".FOO;")?;
    let backup_output_file_path = {
        let backup_output_file_path = format!("{:?}{}", &output_file.path(), ".bak");
        backup_output_file_path.replace("\"", "")
    };
    std::fs::write(
        &backup_output_file_path,
        "const foo = {\"FOO\":\"bar\"}.FOO;",
    )
    .unwrap();

    // act
    let mut cmd = Command::cargo_bin("import-meta-env")?;
    cmd.arg("--env")
        .arg(env_file.path())
        .arg("--example")
        .arg(example_file.path())
        .arg("--output")
        .arg(&output_file.path())
        .arg("--disposable");

    // assert
    cmd.assert().success();
    assert_eq!(
        std::fs::read_to_string(output_file.path())?,
        "const foo = {\"FOO\":\"baz\"}.FOO;"
    );
    assert_eq!(
        std::fs::read_to_string(backup_output_file_path)?,
        "const foo = {\"FOO\":\"bar\"}.FOO;"
    );

    Ok(())
}

#[test]
fn do_not_try_to_restore_if_backup_does_not_contains_placeholder(
) -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let env_file = assert_fs::NamedTempFile::new(".env.example")?;
    env_file.write_str("FOO=baz")?;
    let example_file = assert_fs::NamedTempFile::new(".env.example")?;
    example_file.write_str("FOO=123")?;
    let output_file = assert_fs::NamedTempFile::new("index.js")?;
    output_file.write_str("const foo = \"__import_meta_env_placeholder__\".FOO;")?;
    let backup_output_file_path = {
        let backup_output_file_path = format!("{:?}{}", &output_file.path(), ".bak");
        backup_output_file_path.replace("\"", "")
    };
    std::fs::write(
        &backup_output_file_path,
        "// should not restore from this file!",
    )
    .unwrap();

    // act
    let mut cmd = Command::cargo_bin("import-meta-env")?;
    cmd.arg("--env")
        .arg(env_file.path())
        .arg("--example")
        .arg(example_file.path())
        .arg("--output")
        .arg(&output_file.path());

    // assert
    cmd.assert().success();
    assert_eq!(
        std::fs::read_to_string(output_file.path())?,
        "const foo = {\"FOO\":\"baz\"}.FOO;"
    );
    assert_eq!(
        std::fs::read_to_string(backup_output_file_path)?,
        "const foo = \"__import_meta_env_placeholder__\".FOO;"
    );

    Ok(())
}

#[test]
fn do_not_backup_if_it_does_not_contains_placeholder() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let env_file = assert_fs::NamedTempFile::new(".env.example")?;
    env_file.write_str("FOO=baz")?;
    let example_file = assert_fs::NamedTempFile::new(".env.example")?;
    example_file.write_str("FOO=123")?;
    let output_file = assert_fs::NamedTempFile::new("index.js")?;
    output_file.write_str("// do not backup this file")?;
    let backup_output_file_path = {
        let backup_output_file_path = format!("{:?}{}", &output_file.path(), ".bak");
        backup_output_file_path.replace("\"", "")
    };

    // act
    let mut cmd = Command::cargo_bin("import-meta-env")?;
    cmd.arg("--env")
        .arg(env_file.path())
        .arg("--example")
        .arg(example_file.path())
        .arg("--output")
        .arg(&output_file.path());

    // assert
    cmd.assert().success();
    assert_eq!(
        std::fs::read_to_string(output_file.path())?,
        "// do not backup this file"
    );
    assert_eq!(
        std::path::Path::new(&backup_output_file_path).exists(),
        false
    );

    Ok(())
}

#[test]
fn do_not_backup_or_populate_if_it_is_a_backup_file() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let env_file = assert_fs::NamedTempFile::new(".env.example")?;
    env_file.write_str("FOO=baz")?;
    let example_file = assert_fs::NamedTempFile::new(".env.example")?;
    example_file.write_str("FOO=123")?;
    let output_file = assert_fs::NamedTempFile::new("index.js.bak")?;
    output_file.write_str("const foo = \"__import_meta_env_placeholder__\".FOO;")?;
    let backup_output_file_path = {
        let backup_output_file_path = format!("{:?}{}", &output_file.path(), ".bak");
        backup_output_file_path.replace("\"", "")
    };

    // act
    let mut cmd = Command::cargo_bin("import-meta-env")?;
    cmd.arg("--env")
        .arg(env_file.path())
        .arg("--example")
        .arg(example_file.path())
        .arg("--output")
        .arg(&output_file.path());

    // assert
    cmd.assert().success();
    assert_eq!(
        std::fs::read_to_string(output_file.path())?,
        "const foo = \"__import_meta_env_placeholder__\".FOO;"
    );
    assert_eq!(
        std::path::Path::new(&backup_output_file_path).exists(),
        false
    );

    Ok(())
}
