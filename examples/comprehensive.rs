use mr_pdf::{pct, pt, Align, Color, Orientation, PaperSize, Pdf, VAlign};
use std::fs::File;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let file = File::create("preview/comprehensive.pdf")?;
    let writer = BufWriter::new(file);

    let mut pdf = Pdf::new(writer)?;

    // ── 1. Set Paper Size (A4 Landscape) ──
    pdf.set_paper_size(PaperSize::A4);
    pdf.set_orientation(Orientation::Landscape);

    pdf.text("Comprehensive PDF Report").size(24.0).align_center();
    pdf.text("This report demonstrates styling, percent widths, and paper orientation.")
        .size(12.0)
        .align_center();

    pdf.table(|t| {
        // ── 2. Mixed Widths (Percent and Points) ──
        // Column 1: 10%
        // Column 2: 60%
        // Column 3: 100 points (absolute)
        // Column 4: remainder (let's say 15%)
        t.widths(vec![pct(10.0), pct(60.0), pt(100.0), pct(15.0)]);
        
        // ── 3. Global Alignment (Column-wide) ──
        t.column_align(0, Align::Center);
        t.column_align(2, Align::Center);
        t.column_align(3, Align::Right);
        
        // Set all columns to vertically center (Prevents text from being too close to the bottom)
        t.column_valign(0, VAlign::Center);
        t.column_valign(1, VAlign::Center);
        t.column_valign(2, VAlign::Center);
        t.column_valign(3, VAlign::Center);

        // ── 4. Styling ──
        t.header_style()
            .bg_color(Color::Rgb(44, 62, 80))
            .text_color(Color::Rgb(255, 255, 255))
            .font_size(12.0);
            
        t.row_style().font_size(10.0);

        t.header(vec!["ID", "Description", "Status", "Total"]);

        t.row(vec!["001", "Short description", "Finished", "$ 1,200.00"]);
        
        t.row(vec![
            "002", 
            "This description is very long. Because we have vertical centering (VAlign::Center) and the new baseline adjustment, even with multiple lines, the text will look perfectly balanced inside the cell. It will not be too close to the top or the bottom line, providing a clean professional look.", 
            "In Progress", 
            "$ 3,500.25"
        ]);
        
        t.row(vec!["003", "Quick task", "Completed", "$ 450.00"]);
    })?;

    pdf.finish()?;

    println!("Successfully wrote comprehensive.pdf to preview directory");
    Ok(())
}
