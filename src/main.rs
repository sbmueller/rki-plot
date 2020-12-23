#[macro_use]
extern crate prettytable;
extern crate rasciigraph;
extern crate reqwest;

use prettytable::format;
use prettytable::Table;
use serde::Deserialize;
use std::env;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct RKIDataSet {
    #[serde(rename = "Datum")]
    date: String,
    #[serde(rename = "Schätzer_Neuerkrankungen")]
    new_cases: u32,
    #[serde(rename = "UG_PI_Neuerkrankungen")]
    new_cases_lower: u32,
    #[serde(rename = "OG_PI_Neuerkrankungen")]
    new_cases_upper: u32,
    #[serde(rename = "Schätzer_Neuerkrankungen_ma4")]
    new_cases_average: u32,
    #[serde(rename = "UG_PI_Neuerkrankungen_ma4")]
    new_cases_average_lower: u32,
    #[serde(rename = "OG_PI_Neuerkrankungen_ma4")]
    new_cases_average_upper: u32,
    #[serde(rename = "Schätzer_Reproduktionszahl_R")]
    r_value: f32,
    #[serde(rename = "UG_PI_Reproduktionszahl_R")]
    r_value_lower: f32,
    #[serde(rename = "OG_PI_Reproduktionszahl_R")]
    r_value_upper: f32,
    #[serde(rename = "Schätzer_7_Tage_R_Wert")]
    r_value_average: f32,
    #[serde(rename = "UG_PI_7_Tage_R_Wert")]
    r_value_average_lower: f32,
    #[serde(rename = "OG_PI_7_Tage_R_Wert")]
    r_value_average_upper: f32,
}

/// Return a PathBuf to a csv file in the temp directory determined by the OS
fn get_file_path() -> std::path::PathBuf {
    let mut temp_path = env::temp_dir();
    temp_path.push("rki_data.csv");
    temp_path
}

/// Perform a GET request to `url` and saves the response to file at `path`
///
/// # Arguments
///
/// * `url` - A string slice that represents an URL
/// * `path` - A string slice that represents a path to a file
fn download_file(url: &str, path: &std::path::PathBuf) {
    let mut res = reqwest::blocking::get(url).expect("Could not download file");
    let mut file = File::create(path).expect("Could not create file");
    ::std::io::copy(&mut res, &mut file).expect("Could not write to file");
}

/// Return a String representing an ASCII plot of `data` with `caption`
///
/// # Arguments
///
/// * `data`: A Vec<f64> with data points y(x)
/// * `caption: A String used for the plot caption
fn ascii_plot(data: Vec<f64>, caption: String) -> String {
    rasciigraph::plot(
        data,
        rasciigraph::Config::default()
            .with_offset(10)
            .with_height(10)
            .with_caption(caption),
    )
}

/// Open a RKI CSV file located at `path` and return its content as Vector
///
/// # Arguments
///
/// * `path`: Location of the RKI CSV file
fn load_csv_data(path: &std::path::PathBuf) -> Vec<RKIDataSet> {
    let data = std::fs::read_to_string(path)
        .expect("Could not load CSV file")
        .replace(",", ".");
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(data.as_bytes());
    let (dataset, _): (Vec<_>, Vec<_>) =
        reader.deserialize::<RKIDataSet>().partition(Result::is_ok);
    let dataset: Vec<RKIDataSet> = dataset.into_iter().map(Result::unwrap).collect();
    dataset
}

/// Format certain data fields in `data` and return a table for printing
///
/// # Arguments
///
/// * `data`: Vector of structs with data
/// * `limit`: Limits the table entries to the recent ones
fn get_statistics(data: &Vec<RKIDataSet>, limit: usize) -> Table {
    let mut table = Table::new();
    table.add_row(row!["Date", "New cases", "R-Value"]);
    for infection in data.iter().rev().take(limit) {
        table.add_row(row![
            infection.date,
            infection.new_cases.to_string(),
            infection.r_value.to_string()
        ]);
    }
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table
}

/// Print the main output of the application, consisting of plots and tables
///
/// # Arguments
///
/// * `data`: Vector of data
/// * `plot_limit`: Number of most recent data points to plot
/// * `table_limit`: Number of most recent table entries to output
fn print_output(data: &Vec<RKIDataSet>, plot_limit: usize, table_limit: usize) {
    let fig1 = ascii_plot(
        data.iter()
            .rev()
            .take(plot_limit) // plot last 14 days
            .rev()
            .map(|i| i.new_cases as f64)
            .collect::<Vec<_>>(),
        format!("New infections in the last {} days", plot_limit),
    );
    let fig2 = ascii_plot(
        data.iter()
            .rev()
            .take(plot_limit)
            .rev()
            .map(|i| i.r_value as f64)
            .collect::<Vec<_>>(),
        format!("R-value in the last {} days", plot_limit),
    );
    let statistics = get_statistics(&data, table_limit);
    let mut output = Table::new();
    output.add_row(row![fig1, fig2]);
    output.add_row(row![statistics, ""]);
    output.set_format(*format::consts::FORMAT_CLEAN);
    output.printstd();
}

fn main() {
    let url = "https://www.rki.de/DE/Content/InfAZ/N/Neuartiges_Coronavirus/Projekte_RKI/Nowcasting_Zahlen_csv.csv?__blob=publicationFile";
    let plot_limit = 30; // how many recent days should be plotted
    let table_limit: usize = 7; // how many recent days should be printed as table

    // start the program
    let path = get_file_path();
    download_file(url, &path);
    let infections = load_csv_data(&path);
    print_output(&infections, plot_limit, table_limit);
}
