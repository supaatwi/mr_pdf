use mr_pdf::{Align, Color, Orientation, PaperSize, Pdf, VAlign};
use std::fs::File;

fn main() -> std::io::Result<()> {
    // Ensure preview dir exists
    std::fs::create_dir_all("preview").unwrap_or(());
    
    let file = File::create("preview/logbook_table.pdf")?;
    let mut pdf = Pdf::stream(file)?;

    // Configure A4 Landscape size
    pdf.set_paper_size(PaperSize::A4);
    pdf.set_orientation(Orientation::Landscape);
    pdf.set_margin(30.0);

    pdf.text("Logbook Table Example (Fully featuring Rowspan and Colspan)").size(16.0);
    pdf.advance_cursor(10.0);

    pdf.table(|t| {
        // We have 11 functional columns with fixed widths
        t.widths(vec![
            90.0,  // (8) Name
            70.0,  // (9) License
            60.0,  // (10) Date
            60.0,  // (11) Time Out
            60.0,  // (12) Time In
            70.0,  // (13) From
            70.0,  // (14) To
            60.0,  // (15) Dist Out
            60.0,  // (16) Dist In
            80.0,  // (17) Total Dist
            80.0,  // (18) Total Hours
        ]);

        t.header_style()
            .font_size(9.0)
            .bg_color(Color::Rgb(240, 240, 240));

        t.row_style().font_size(9.0);

        // --- Header Row 1 ---
        t.header_row_builder(|row| {
            // Rowspanned cells
            row.cell("(8) Driver's\nName & Surname").align(Align::Center).valign(VAlign::Center).rowspan(2);
            row.cell("(9) Driving\nLicense No.").align(Align::Center).valign(VAlign::Center).rowspan(2);
            row.cell("(10) Date\n(DD/MM/YYYY)").align(Align::Center).valign(VAlign::Center).rowspan(2);
            
            // Colspanned cells for sub-headers
            row.cell("Transport Time").span(2).align(Align::Center);
            row.cell("Workplace Details").span(2).align(Align::Center);
            row.cell("Odometer Number").span(2).align(Align::Center);
            
            // Rowspanned cells on the right
            row.cell("(17) Total\nDistance (km)").align(Align::Center).valign(VAlign::Center).rowspan(2);
            row.cell("(18) Total\nWorking Hours").align(Align::Center).valign(VAlign::Center).rowspan(2);
        });

        // --- Header Row 2 ---
        // Exclude cells that are already covered by the rowspan=2 from the first row
        t.header_row_builder(|row| {
            row.cell("(11) Time\nOut").align(Align::Center);
            row.cell("(12) Time\nIn").align(Align::Center);
            
            row.cell("(13) From").align(Align::Center);
            row.cell("(14) To").align(Align::Center);
            
            row.cell("(15) Out").align(Align::Center);
            row.cell("(16) In").align(Align::Center);
        });

        // --- Sample Data (10 Rows) ---
        for i in 1..=10 {
            t.row_builder(|row| {
                row.cell(&format!("Driver #{}", i));
                row.cell(&format!("DL-{:04}", i));
                row.cell("11-03-2026");
                row.cell("08:00");
                row.cell("12:00");
                row.cell("City A");
                row.cell("City B");
                row.cell("1000");
                row.cell("1080");
                row.cell("80");
                row.cell("4");
            });
        }
    })?;

    pdf.finish()?;

    println!("logbook_table.pdf created successfully in preview folder!");

    Ok(())
}
