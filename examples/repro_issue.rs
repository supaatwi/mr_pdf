use mr_pdf::{Pdf, SizeExt, TableBorderStyle, TableBuilder, TableCell, Color};
use std::fs::File;

fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("preview").unwrap_or(());
    
    let file = File::create("preview/repro_issue.pdf")?;
    let mut pdf = Pdf::stream(file)?;
    pdf.set_orientation(mr_pdf::Orientation::Landscape);

    pdf.text("Reproduction of Table Cutting Issue")
        .size(18.0)
        .align_center()
        .margin_bottom(20.0);

    // Common builder mimicking the user's many-column table
    let mut builder = TableBuilder::new();
    builder
        .widths(vec![
            30.0.pt(), 60.0.pt(), 30.0.pt(), 70.0.pt(), 70.0.pt(), 
            90.0.pt(), 120.0.pt(), 40.0.pt(), 90.0.pt(), 120.0.pt(), 
            40.0.pt(), 80.0.pt(), 30.0.pt()
        ])
        .border(TableBorderStyle::Full)
        .zebra(Color::Rgb(245, 245, 245))
        .header(vec![
            "No.", "Status", "-", "Lat", "Lon", 
            "Time Start", "Address Start", "Val1", "Time End", "Address End", 
            "Val2", "Duration", "-"
        ]);

    // Test with repeat_header TRUE to verify headers appear on new pages
    builder.repeat_header(true);

    let mut streaming = builder.start(&mut pdf)?;

    for i in 1..=100 {
        streaming.add_row(vec![
            TableCell::from(format!("{}", i)),
            TableCell::from(if i % 10 == 0 { "MOVING" } else { "IDLING" }),
            TableCell::from("-"),
            TableCell::from("17.3712416"),
            TableCell::from("102.8165816"),
            TableCell::from("31/01/2026 14:07:41"),
            TableCell::from("ต. หนองบอนกว้าง อ. เมืองอุดรธานี จ. อุดรธานี"),
            TableCell::from("0.00"),
            TableCell::from("31/01/2026 14:10:21"),
            TableCell::from("ต. หนองบอนกว้าง อ. เมืองอุดรธานี จ. อุดรธานี"),
            TableCell::from("0.00"),
            TableCell::from("2 นาที 40 วินาที"),
            TableCell::from("-"),
        ])?;
    }

    pdf.finish()?;

    println!("Generated 'preview/repro_issue.pdf'");
    Ok(())
}
