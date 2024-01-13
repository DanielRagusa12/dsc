use std::env;
use std::fs;
use std::path::Path;
use clap::{Parser, Subcommand};
use rust_search::SearchBuilder;
use anyhow::Result;
use std::time::Duration;
use log::LevelFilter;
use nanoid::nanoid;


#[derive(Debug, Parser)]
struct DriveScraperArgs {
    #[clap(subcommand)]
    sub_command_type: SubCommandType,

}

#[derive(Debug, Subcommand)]
enum SubCommandType {
    /// List files found
    List {
        // required extension argument
        /// File extension to search for
        #[clap(short, long)]
        extension : String,
        /// Limit the number of files displayed
        #[clap(short, long)]
        limit: Option<usize>,
        
    },
    /// Copy files found to dscraped dir
    Copy {
        /// Directory Reconstriuction
        #[clap(short, long)]
        reconstruct: bool,
        /// File extension to search for
        #[clap(short, long)]
        extension : String,
    },
    /// Clear descraped/searches dir
    Clear,

}



fn copy_files(files: &Vec<String>, ext: &String, dscraped_path: &Path, reconstruct: &bool) -> Result<()> {
    let pb = indicatif::ProgressBar::new(files.len() as u64);
    let copy_log_file = dscraped_path.join("logs").join("copy.log");
    simple_logging::log_to_file(&copy_log_file, LevelFilter::Info)?;

    let ext_path = if *reconstruct {
        dscraped_path.join("reconstructions").join(&ext)
    } else {
        dscraped_path.join("searches").join(&ext)
    };

    fs::create_dir_all(&ext_path)?;

    // clear the dir if it already exists
    if ext_path.exists() {
        fs::remove_dir_all(&ext_path)?;
        fs::create_dir_all(&ext_path)?;
    }

    let mut error_count = 0;
    for file in files {
        let file_path = Path::new(&file);
        let file_name = file_path.file_name().unwrap();
        let mut dscraped_file_path = ext_path.join(file_name);

        if *reconstruct {
            let root_dir = env::current_dir()?;
            let original_dir = file_path.parent().unwrap().strip_prefix(root_dir).unwrap();
            let new_dir = ext_path.join(original_dir);
            fs::create_dir_all(&new_dir)?;
            dscraped_file_path = new_dir.join(file_name);
        } else if dscraped_file_path.exists() {
            let file_name = file_name.to_str().unwrap().to_string();
            let ext_index = file_name.find(".").unwrap();
            let mut new_file_name = file_name.clone();
            new_file_name.insert_str(ext_index, &format!("-{}", nanoid!(5)));
            dscraped_file_path = ext_path.join(new_file_name);
        }

        if let Err(err) = fs::copy(file_path, &dscraped_file_path) {
            log::error!("Error copying file: {:?}: {}", file_path ,err);
            error_count += 1;
        }
        pb.inc(1);
    }
    pb.finish_and_clear();
    println!("{} out of {} files copied", files.len() - error_count, files.len());

    Ok(())
}


fn search_files (ext: &String) -> Result<Vec<String>, anyhow::Error> {
    let spinner = indicatif::ProgressBar::new_spinner();
    let current_dir = env::current_dir()?;
    let tick_rate = Duration::from_millis(100);
    // spinner.set_message("Searching...");
    spinner.enable_steady_tick(tick_rate);
    let files: Vec<String> = SearchBuilder::default()
    .location(current_dir)
    .ext(ext)
    .build()
    .collect();
    
    spinner.finish_and_clear();

    Ok(files)

}

fn get_time (elapsed: Duration) -> String {
    if elapsed.as_secs() < 1 {
        // return formatted string
        return format!("{}ms", elapsed.as_millis());
    } else {
        // print seconds up to two decimal places
        let secs = elapsed.as_secs_f32();
        return format!("{:.2}s", secs);
    }
} 

fn main() -> Result<()> {
    


    let args = DriveScraperArgs::parse();
    // let current_dir = env::current_dir()?;

    
    // *****************************************************
    let local_app_data = env::var("LOCALAPPDATA")?;  
    let dscraped_path = Path::new(&local_app_data).join("dscraped");
    if !dscraped_path.exists() {
        fs::create_dir_all(&dscraped_path)?;
        
    }
    
    // create logs dir
    let logs_path = dscraped_path.join("logs");
    if !logs_path.exists() {
        fs::create_dir_all(&logs_path)?;
    }

    // *****************************************************

    


    // *****************************************************
        
    match args.sub_command_type {
        SubCommandType::List { limit, extension } => {
            // time the search
            let now = std::time::Instant::now();
            let files: Vec<String> = search_files(&extension)?;
            let elapsed = now.elapsed();
            println!("{} files found in {}", files.len(), get_time(elapsed));
            let limit = limit.unwrap_or(0);
            if limit > 0 {
                for file in files.iter().take(limit) {
                    println!("{}", file);
                }
                if files.len() > limit {
                    println!("+ {} more", files.len() - limit);
                }
                
            } 
        },
        SubCommandType::Copy { reconstruct, extension } => {
            // time the search
            let now = std::time::Instant::now();
            let files: Vec<String> = search_files(&extension)?;
            let elapsed = now.elapsed();
            println!("{} files found in {}", files.len(), get_time(elapsed));
            copy_files(&files, &extension, &dscraped_path, &reconstruct)?;
        }
        SubCommandType::Clear => {
            let searches_path = dscraped_path.join("searches");
            let reconstructions_path = dscraped_path.join("reconstructions");
            if searches_path.exists() {
                fs::remove_dir_all(&searches_path)?;
                println!("Searches cleared");
            }
            if reconstructions_path.exists() {
                fs::remove_dir_all(&reconstructions_path)?;
                println!("Reconstructions cleared");
            }
            else {
                println!("No searches to clear");
            }
        }
    }

    // *****************************************************


    Ok(())

 
}






