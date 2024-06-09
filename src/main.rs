use csv;
use std::rc::Rc;
use txn::*;

fn main() {
    let mut shard = Shard::new();

    {
        // Parse the input and populate the shard with data.
        let mut csv_reader = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_reader(std::io::stdin());
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

    {
        // Generate the output and write it to stdout.
        let output = shard.generate_output();
        let mut csv_writer = csv::Writer::from_writer(std::io::stdout());
        for record in output {
            csv_writer.serialize(record).unwrap();
        }
        csv_writer.flush().unwrap();
    }

    {
        // Print any errors that occurred during processing.
        shard.errors().iter().for_each(|err| {
            eprintln!("{}", err);
        });
    }
}
