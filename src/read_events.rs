use serde::Deserialize;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

#[derive(Deserialize, Debug)]
struct Invoice {
    amount: f64,
    #[serde(rename = "feedId")]
    feed_id: Option<String>,
    #[serde(rename = "invoiceId")]
    invoice_id: Option<String>,
    #[serde(rename = "eventType")]
    event_type: String,
    timestamp: String,
}

pub struct EventReader {}

impl EventReader {
    pub fn read_events() -> std::io::Result<()> {
        let choosen_year = "2022";
        let months_of_the_year = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];

        let input_file = File::open("./src/events.txt")?;
        let reader = BufReader::new(input_file);
        let output_file = File::create("./src/yearlyreport.txt")?;
        let mut writer = BufWriter::new(output_file);

        let mut month: usize = 1;
        let mut current_exposure: f64 = 0.0;
        let mut months_max_exposure: f64 = 0.0;
        let mut corrupt_lines = 0;

        for line in reader.lines() {
            let event = line.unwrap();

            let invoice: Invoice = match serde_json::from_str(&event) {
                Ok(value) => value,
                Err(error) => {
                    // println!("ERRROR {:?}", error);
                    corrupt_lines = corrupt_lines + 1;
                    continue;
                }
            };

            let time_stamp = invoice.timestamp.split('-').collect::<Vec<&str>>();

            //TODO: Validate date
            if (time_stamp[0] == choosen_year) {
                if (&time_stamp[1].parse::<usize>().unwrap() > &month) {
                    let month_output = format!(
                        "{}: {} \n",
                        months_of_the_year[&month - 1],
                        months_max_exposure
                    );
                    writer.write_all(month_output.as_bytes());
                    months_max_exposure = current_exposure;
                    month = month + 1;
                }

                match invoice.event_type.as_str() {
                    "InvoiceRegistered" => current_exposure += invoice.amount,
                    "LateFeeRegistered" => current_exposure += invoice.amount,
                    "PaymentRegistered" => current_exposure -= invoice.amount,
                    _ => corrupt_lines = corrupt_lines + 1,
                };

                if (current_exposure > months_max_exposure) {
                    months_max_exposure = current_exposure;
                }
            } else {
                corrupt_lines = corrupt_lines + 1;
            }
        }

        let last_month_output = format!(
            "{}: {} \n",
            months_of_the_year[&month - 1],
            months_max_exposure
        );
        writer.write_all(last_month_output.as_bytes());

        let corrupt_lines_output = format!("\nNumber of corrupted lined: {} \n", corrupt_lines);
        writer.write_all(corrupt_lines_output.as_bytes());
        writer.flush();

        Ok(())
    }
}
