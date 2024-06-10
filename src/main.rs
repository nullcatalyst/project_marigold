use csv;
use std::{fs::File, rc::Rc};
use txn::*;

fn main() {
    let mut shard = Shard::new();

    {
        // Parse the input and populate the shard with data.

        let mut cli_args = std::env::args();

        // Remember that the first argument is always the name of the program.
        match cli_args.len() {
            1 => {
                // No arguments, read from stdin.
                let reader: Box<dyn std::io::Read> = Box::new(std::io::stdin());
                let csv_reader = csv::ReaderBuilder::new()
                    .trim(csv::Trim::All)
                    .from_reader(reader);
                import_csv(&mut shard, csv_reader);
            }

            2 => {
                // One argument, read from the given file.
                let reader: Box<dyn std::io::Read> =
                    Box::new(File::open(cli_args.nth(1).unwrap()).unwrap());
                let csv_reader = csv::ReaderBuilder::new()
                    .trim(csv::Trim::All)
                    .from_reader(reader);
                import_csv(&mut shard, csv_reader);
            }

            _ => {
                // Too many arguments.
                panic!("Usage: {} [input.csv]", cli_args.nth(0).unwrap());
            }
        }
    }

    {
        // Generate the output and write it to stdout.
        export_csv(&shard, &mut std::io::stdout());
    }

    {
        // Print any errors that occurred during processing.
        shard.errors().iter().for_each(|err| {
            eprintln!("{}", err);
        });
    }
}

fn import_csv<R: std::io::Read>(shard: &mut Shard, mut csv_reader: csv::Reader<R>) {
    for result in csv_reader.deserialize() {
        let record: Result<Event, _> = result;
        match record {
            Ok(event) => {
                shard.push_event(event);
            }
            Err(e) => {
                shard.push_error(ShardError::CsvParseError(Rc::new(e)));
            }
        }
    }
}

fn export_csv<W: std::io::Write>(shard: &Shard, writer: &mut W) {
    let output = shard.generate_output_sorted();
    let mut csv_writer = csv::Writer::from_writer(writer);
    for record in output {
        csv_writer.serialize(record).unwrap();
    }
    csv_writer.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use csv_test_proc::csv_test;

    csv_test!("1_simple");
    csv_test!("2_reorder_headings");
    csv_test!("3_invalid_rows");
    csv_test!("4_overflow");
    csv_test!("5_hold");
    csv_test!("6_chargeback");
}
