use chrono::offset::Local;
use chrono::DateTime;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::path::Path;
use tar;

fn archivate(path_to_dir: &String, path_to_archive: &String) -> Result<(), std::io::Error> {
    let date = Local::now();
    let arch_date = DateTime::to_rfc3339(&date)[..19].replace(":", "-");
    let arch_date = arch_date + ".tar";

    let arch_date_path = Path::new(&arch_date);

    let arch_path = Path::new(&path_to_archive).join(&arch_date_path);

    let tar_gz = File::create(&arch_path)?;
    let enc = GzEncoder::new(&tar_gz, Compression::default());
    let mut tar_obj = tar::Builder::new(enc);

    tar_obj.append_dir_all(".", &path_to_dir)?;

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() - 1 != 2 {
        println!(
            "Error, you entered {:?} Arguments: {:?}.",
            args.len() - 1,
            &args[1..]
        );

        println!("It is required to enter 2 arguments.");
        println!("1 Argument is a path to the directory to archive. (~/script/work/data)");
        println!("2 Argument is the path to save the backup archive. (~/backup)");
    } else {
        let path_to_dir = &args[1];
        let path_to_archive = &args[2];

        archivate(&path_to_dir, &path_to_archive)?;
    }

    Ok(())
}
