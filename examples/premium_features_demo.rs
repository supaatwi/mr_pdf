use mr_pdf::{Align, Color, Pdf, TableBorderStyle};
use std::fs::File;

fn main() -> std::io::Result<()> {
    // Ensure preview dir exists
    std::fs::create_dir_all("preview").unwrap_or(());

    let file = File::create("preview/premium_features_demo.pdf")?;
    let mut pdf = Pdf::stream(file)?;

    // 1. Setup Fonts (Note: For real bold, register "Sarabun-Bold" font)
    pdf.register_font("Sarabun", "font/Sarabun-Regular.ttf")?;
    
    // 2. Feature 6: Watermark
    // Set a light gray "DRAFT" watermark at 45 degrees
    pdf.set_watermark("PREMIUM REPORT", 60.0, Color::Rgb(220, 220, 220), 0.2, 45.0);

    pdf.text("Premium Features Demonstration").size(24.0).align(Align::Center);
    pdf.advance_cursor(20.0);

    pdf.text("This document showcases the latest features added to mr-pdf library: Rich Text, QR Codes, Individual Cell Styling, and Watermarks.")
        .size(12.0);
    pdf.advance_cursor(30.0);

    // 3. Table with multiple new features
    pdf.table(|t| {
        t.widths(vec![150.0, 200.0, 150.0]);
        t.border(TableBorderStyle::Full);
        
        t.header_row_builder(|row| {
            row.cell("Feature").align(Align::Center).bg_color(Color::Rgb(240, 240, 240));
            row.cell("Demonstration").align(Align::Center).bg_color(Color::Rgb(240, 240, 240));
            row.cell("Details").align(Align::Center).bg_color(Color::Rgb(240, 240, 240));
        });

        // Row 1: Feature 1 - Rich Text
        t.row_builder(|row| {
            row.cell("1. Rich Text");
            row.cell("This is **Bold Text** and this is [#FF5555]Red Text[]. You can even combine **[#0000FF]Bold Blue[]**.");
            row.cell("Uses simple Markdown syntax and hex color tags.");
        });

        // Row 2: Feature 4 - Individual Styling
        t.row_builder(|row| {
            row.cell("2. Cell Styling");
            row.cell("I have a yellow background")
                .bg_color(Color::Rgb(255, 255, 200)) // Yellow BG
                .text_color(Color::Rgb(200, 100, 0)); // Brown Text
            row.cell("I am very BIG")
                .font_size(18.0)
                .text_color(Color::Rgb(0, 150, 0)); // Green Text
        });

        // Row 3: Feature 2 - QR Code
        #[cfg(feature = "qrcode")]
        t.row_builder(|row| {
            row.cell("3. QR Codes");
            row.cell_qr("https://github.com/supaatwi/mr_pdf");
            row.cell("Vector-based QR code generated instantly. Lightweight and sharp.");
        });
        
        #[cfg(not(feature = "qrcode"))]
        t.row_builder(|row| {
            row.cell("3. QR Codes");
            row.cell("Enable 'qrcode' feature to see this.");
            row.cell("Optional dependency.");
        });
    })?;

    pdf.advance_cursor(40.0);
    pdf.text("You can also render QR codes independently:").size(12.0);
    pdf.advance_cursor(10.0);
    
    // Independent QR Code
    #[cfg(feature = "qrcode")]
    {
        let (x, y) = pdf.cursor_pos();
        pdf.render_qr("https://github.com/supaatwi/mr_pdf", x, y - 80.0, 80.0)?;
        pdf.advance_cursor(90.0);
    }

    pdf.finish()?;

    println!("premium_features_demo.pdf created successfully in preview folder!");
    println!("To see QR Codes, run with: cargo run --example premium_features_demo --features qrcode");

    Ok(())
}
