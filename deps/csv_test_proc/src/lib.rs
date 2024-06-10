use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};

/// This macro generates a test function that reads a CSV file, imports it into a `Shard`, exports
/// the `Shard` to a CSV file, and compares the exported CSV file with a reference CSV file.
///
/// A proc_macro is required here, because a simple macro doesn't work in this case, due to the fact
/// that we cannot use the `concat_idents!` macro to create a function name.
#[proc_macro]
pub fn csv_test(input: TokenStream) -> TokenStream {
    let literal = parse_macro_input!(input as LitStr);
    let value = literal.value();

    format!(
        "#[test]
fn test_{}() {{
    let mut shard = Shard::new();
    let csv_reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(\"data/{}.test.csv\")
        .unwrap();
    import_csv(&mut shard, csv_reader);

    let mut got = Vec::new();
    export_csv(&shard, &mut std::io::BufWriter::new(&mut got));
    let got = String::from_utf8(got).unwrap();

    let want = std::fs::read_to_string(\"data/{}.want.csv\")
        .unwrap();
    assert_eq!(got, want);
}}",
        value, value, value
    )
    .parse()
    .unwrap()
}
