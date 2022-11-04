use clap::Parser;
use cli_args::CliArgs;

mod cli_args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    let out_file = args.out_file.unwrap_or_else(|| {
        let file_name: String = args
            .group
            .chars()
            .into_iter()
            .map(|c| {
                if c.is_ascii() {
                    c.to_ascii_lowercase()
                } else {
                    '_'
                }
            })
            .collect();
        format!("{}.sqlite", file_name)
    });

    if std::path::Path::new(&out_file).exists() {
        if !args.force {
            println!(
                "Out file '{}' already exists. Use --force to overwrite.",
                out_file
            );
            return Ok(());
        } else {
            std::fs::rename(&out_file, format!("{}~", out_file))?;
        }
    }

    gitlab_timereport::go(
        args.uri.strip_suffix('/').unwrap_or(&args.uri).to_string(),
        args.token,
        args.group,
        out_file.clone(),
    )?;

    println!("Wrote database to file {}", out_file);

    Ok(())
}
