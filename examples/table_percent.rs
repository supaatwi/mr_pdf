use mr_pdf::{pct, Align, Color, Pdf};
use std::fs::File;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let file = File::create("preview/table_percent.pdf")?;
    let writer = BufWriter::new(file);

    let mut pdf = Pdf::new(writer)?;

    pdf.text("Table with Percentage Widths").size(24.0).align_center();

    pdf.table(|t| {
        // ── Specify Widths as Percent or Points ──
        // 10% for ID, 60% for Description, 15% for Status, 15% for Price
        // Using pct(n) helper for percentage or raw f64 for points
        t.widths(vec![pct(10.0), pct(60.0), pct(15.0), pct(15.0)]);
        
        t.column_align(0, Align::Center);
        t.column_align(2, Align::Center);
        t.column_align(3, Align::Right);

        t.header_style()
            .bg_color(Color::Rgb(52, 73, 94))
            .text_color(Color::Rgb(255, 255, 255));
            
        t.header(vec!["ID", "Description", "Status", "Price"]);

        t.row(vec!["1", "This table uses percentage-based widths.", "OK", "$ 10.00"]);
        t.row(vec![
            "2", 
            "The percentage is calculated based on the available page width (Page Width - Margins). This makes it very easy to create responsive-like layouts that fit the page perfectly.", 
            "Great", 
            "$ 50.00"
        ]);
    })?;

    pdf.finish()?;

    println!("Successfully wrote table_percent.pdf to preview directory");
    Ok(())
}
