extern crate reqwest;

use std::fs::File;

fn main() {
    let url = "https://www.rki.de/DE/Content/InfAZ/N/Neuartiges_Coronavirus/Projekte_RKI/Nowcasting_Zahlen_csv.csv?__blob=publicationFile";
    let path = "rki_data.csv";
    println!("Downloading latest RKI data ...");
    download_file(url, path);
}

/// Performs a GET request to `url` and saves the response to file at `path`
///
/// # Arguments
///
/// * `url` - A string slice that represents an URL
/// * `path` - A string slice that represents a path to a file
fn download_file(url: &str, path: &str) {
    let mut res = reqwest::blocking::get(url).expect("Could not download file");
    let mut file = File::create(path).expect("Could not create file");
    ::std::io::copy(&mut res, &mut file).expect("Could not write to file");
}
