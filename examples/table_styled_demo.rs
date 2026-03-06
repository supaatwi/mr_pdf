use mr_pdf::{Align, Color, Pdf, SizeExt, TableBorderStyle};
use std::fs::File;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let file = File::create("preview/table_styled_demo.pdf")?;
    let mut pdf = Pdf::new(BufWriter::new(file))?;
    pdf.set_title("Table Styling Demo");

    pdf.text("Table Styling Demo")
        .size(24.0)
        .align_center()
        .margin_bottom(30.0);

    // 1. Full Border (Default)
    pdf.text("1. Full Border (Default)")
        .size(14.0)
        .margin_bottom(10.0);
    pdf.table(|t| {
        t.widths(vec![30.0.pct(), 30.0.pct(), 40.0.pct()]);
        t.header(vec!["Item", "Qty", "Description"]);
        t.row(vec!["Apple", "10", "Fresh red apples"]);
        t.row(vec!["Banana", "5", "Ripened yellow bananas"]);
    })?;

    pdf.advance_cursor(30.0);

    // 2. Ghost Style (Horizontal lines only) + Zebra Striping
    pdf.text("2. Ghost Style + Zebra Striping")
        .size(14.0)
        .margin_bottom(10.0);
    pdf.table(|t| {
        t.widths(vec![30.0.pct(), 30.0.pct(), 40.0.pct()]);
        t.header(vec!["Service", "Frequency", "Cost"]);
        t.border(TableBorderStyle::Ghost);
        t.zebra(Color::Rgb(245, 245, 245));

        t.row(vec!["Hosting", "Monthly", "$10.00"]);
        t.row(vec!["Backup", "Daily", "$5.00"]);
        t.row(vec!["Support", "Annual", "$100.00"]);
        t.row(vec!["Security", "Monthly", "$15.00"]);
    })?;

    pdf.advance_cursor(30.0);

    // 3. Header Only Border
    pdf.text("3. Header Only Border")
        .size(14.0)
        .margin_bottom(10.0);
    pdf.table(|t| {
        t.widths(vec![50.0.pct(), 50.0.pct()]);
        t.header(vec!["Attribute", "Value"]);
        t.border(TableBorderStyle::HeaderOnly);

        t.row(vec!["Name", "Antigravity"]);
        t.row(vec!["Role", "AI Assistant"]);
        t.row(vec!["Status", "Active"]);
    })?;

    pdf.advance_cursor(30.0);

    // 4. No Border (Ghostly Clean)
    pdf.text("4. No Border (Clean)")
        .size(14.0)
        .margin_bottom(10.0);
    pdf.table(|t| {
        t.widths(vec![50.0.pct(), 50.0.pct()]);
        t.header(vec!["Key", "Description"]);
        t.border(TableBorderStyle::None);

        t.row_builder(|r| {
            r.cell("Minimalist").align(Align::Left);
            r.cell("Very clean layout").align(Align::Right);
        });
        t.row_builder(|r| {
            r.cell("Professional").align(Align::Left);
            r.cell("Maximum clarity").align(Align::Right);
        });
    })?;

    pdf.finish()?;
    println!("Successfully generated table_styled_demo.pdf");
    Ok(())
}
