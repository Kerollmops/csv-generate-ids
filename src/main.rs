use std::io;
use std::iter::once;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "csv-generate-ids",
    about = "A simple tool to generate unique sequential identifiers."
)]
struct Opt {
    /// Define the name of the field which will contain the record id.
    #[structopt(long, default_value = "id")]
    id_field_name: String,

    /// Define the amount of ids that seperate each generated identifier.
    #[structopt(long, default_value = "1")]
    id_step_by: usize,

    /// Define the first identifier that must be generated.
    #[structopt(long, default_value = "0")]
    id_start_at: usize,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut record = csv::StringRecord::new();

    let headers = rdr.headers()?;
    wtr.write_record(once(opt.id_field_name.as_str()).chain(headers.iter()))?;

    let mut record_id = opt.id_start_at;
    while rdr.read_record(&mut record)? {
        let id = record_id.to_string();
        let values = once(id.as_str()).chain(record.iter());
        wtr.write_record(values)?;

        record_id += opt.id_step_by;
    }

    Ok(())
}
