use mr_pdf::{Align, Color, Pdf, VAlign};
use std::fs::File;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let file = File::create("preview/table_styled.pdf")?;
    let writer = BufWriter::new(file);

    let mut pdf = Pdf::new(writer)?;

    pdf.text("Styled Table Example").size(24.0).align_center();

    pdf.table(|t| {
        // Set column widths
        t.widths(vec![50.0, 200.0, 80.0, 80.0]);
        
        // ── 1. Set Global Column Alignment ──
        // (No need to set per record as requested)
        t.column_align(0, Align::Center);  // ID: Center
        t.column_align(2, Align::Center);  // Status: Center
        t.column_align(3, Align::Right);   // Price: Right
        
        // Set vertical alignment for the description column to Center
        t.column_valign(1, VAlign::Center); 

        // ── 2. Set Header Style ──
        t.header_style()
            .bg_color(Color::Rgb(41, 128, 185)) // Professional Blue
            .text_color(Color::Rgb(255, 255, 255))
            .font_size(12.0);
            
        t.header(vec!["ID", "Description", "Status", "Price"]);

        // ── 3. Set Row Style ──
        t.row_style()
            .font_size(10.0);

        // Add some records
        t.row(vec!["1", "Simple row with short text", "Active", "$ 0.00"]);
        
        t.row(vec![
            "2", 
            "This description is very long and will wrap into multiple lines. Because we set the column vertical alignment to Center, this text will stay vertically centered even as the row height grows to accommodate the wrapped lines.", 
            "Pending", 
            "$ 120.00"
        ]);

        t.row(vec!["3", "Another record", "Done", "$ 99.90"]);
        
        // Individual cells can still override alignment if needed
        // but for column-wide settings, the above suffices.
    })?;

    pdf.finish()?;

    println!("Successfully wrote table_styled.pdf to preview directory");
    Ok(())
}
