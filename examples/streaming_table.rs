use mr_pdf::{Color, Pdf, SizeExt, TableBorderStyle};
use std::fs::File;

fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("preview").unwrap_or(());
    
    let file = File::create("preview/streaming_table.pdf")?;
    let mut pdf = Pdf::stream(file)?;

    pdf.text("Streaming Table Example")
        .size(20.0)
        .align_center()
        .margin_bottom(20.0);

    pdf.text("This table uses `StreamingTable` which draws rows immediately directly to the PDF file stream. \
              This keeps memory usage exceptionally low, allowing you to stream millions of rows (like from a database or a channel) \
              without running out of RAM.")
        .size(12.0)
        .margin_bottom(20.0);

    // Using the builder to configure the structure
    let mut table_builder = mr_pdf::TableBuilder::new();
    table_builder
        .widths(vec![50.0.pt(), 100.0.pt(), 150.0.pt(), 200.0.pt()])
        .repeat_header(true)
        .border(TableBorderStyle::HeaderOnly)
        .zebra(Color::Rgb(240, 240, 240))
        .header(vec!["Ref ID", "Type", "Timestamp", "Description"]);

    // Start streaming: this consumes the builder and draws the header.
    let mut streaming_table = table_builder.start(&mut pdf)?;

    let total_rows = 500; // Imagine this comes continuously from rx.recv().await

    for i in 1..=total_rows {
        // Stream data row by row. Once `.row` finishes, the bytes are pushed to the PDF stream!
        streaming_table.row(|r| {
            r.cell(&format!("{}", i));
            r.cell(if i % 2 == 0 { "Vehicle" } else { "Personnel" });
            r.cell(&format!("2026-03-11 12:{:02}:00", i % 60));
            r.cell(&format!("Record details for reference id #{}", i));
        })?;
    }

    // No need to call `.build()` or `.render()`, as the table is already drawn.
    // Memory used during the loop scales only with one single row, not 500 rows!

    // Wait until the borrow over `pdf` drops, then finish
    drop(streaming_table);

    pdf.finish()?;

    println!("Successfully generated 'preview/streaming_table.pdf'");
    Ok(())
}
