use mr_pdf::{Align, Color, PaperSize, Pdf, SizeExt, TableBorderStyle, VAlign};
use std::fs::File;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    // 1. Initialize PDF
    let file = File::create("preview/invoice_premium.pdf")?;
    let mut pdf = Pdf::new(BufWriter::new(file))?;
    
    pdf.set_paper_size(PaperSize::A4);
    pdf.set_title("Premium Invoice - #INV-2026-001");
    pdf.set_author("Modern Solutions Group");

    // --- HEADER SECTION ---
    pdf.row(|r| {
        // Logo / Company Name
        r.col(60.0.pct(), |pdf| {

            // Company Name
            pdf.text("MODERN SOLUTIONS")
                .size(28.0)
                .color(Color::Rgb(44, 62, 80)) // Deep Navy
                .margin_bottom(5.0);
            
            pdf.text("Professional Business Infrastructure Services")
                .size(10.0)
                .color(Color::Rgb(120, 120, 120));
            Ok(())
        });

        // Invoice Label & Details
        r.col(40.0.pct(), |pdf| {
            pdf.text("INVOICE")
                .size(32.0)
                .align_right()
                .margin_bottom(10.0);
            
            pdf.table(|t| {
                t.border(TableBorderStyle::None);
                t.widths(vec![50.0.pct(), 50.0.pct()]);
                t.column_align(1, Align::Right);
                
                t.row(vec!["Invoice #:", "INV-2026-001"]);
                t.row(vec!["Date:", "March 06, 2026"]);
                t.row(vec!["Due Date:", "April 06, 2026"]);
            })?;
            Ok(())
        });
    })?;

    pdf.advance_cursor(40.0);

    // --- BILLING INFO SECTION ---
    pdf.row(|r| {
        // From
        r.col(50.0.pct(), |pdf| {
            pdf.text("FROM:")
                .size(10.0)
                .color(Color::Rgb(150, 150, 150))
                .margin_bottom(5.0);
            
            pdf.text("Modern Solutions Group LLC")
                .size(12.0)
                .margin_bottom(3.0);
            
            pdf.text("123 Orbital Station Dr.\nLow Earth Orbit\nSpace-001")
                .size(10.0)
                .color(Color::Rgb(80, 80, 80));
            Ok(())
        });

        // To
        r.col(50.0.pct(), |pdf| {
            pdf.text("BILL TO:")
                .size(10.0)
                .color(Color::Rgb(150, 150, 150))
                .margin_bottom(5.0);
            
            pdf.text("Global Tech Innovators")
                .size(12.0)
                .margin_bottom(3.0);
            
            pdf.text("789 Silicon Valley Way\nPalo Alto, CA 94301\nUnited States")
                .size(10.0)
                .color(Color::Rgb(80, 80, 80));
            Ok(())
        });
    })?;

    pdf.advance_cursor(40.0);

    // --- LINE ITEMS TABLE ---
    pdf.table(|t| {
        // Setup widths (Qty, Description, Unit Price, Total)
        t.widths(vec![10.0.pct(), 60.0.pct(), 15.0.pct(), 15.0.pct()]);
        
        // Style the Header
        t.header_style()
            .bg_color(Color::Rgb(30, 144, 255))
            .text_color(Color::Rgb(255, 255, 255))
            .font_size(10.0);
        
        t.header(vec!["Qty", "Description", "Unit Price", "Total"]);
        
        // Alignment
        t.column_align(0, Align::Center);
        t.column_align(2, Align::Right);
        t.column_align(3, Align::Right);
        
        // Styling: Ghost borders + Zebra
        t.border(TableBorderStyle::Ghost);
        t.zebra(Color::Rgb(245, 250, 255));
        
        // Data Rows
        t.row(vec!["1", "AI Code Review System - Enterprise Tier", "$ 2,500.00", "$ 2,500.00"]);
        t.row(vec!["10", "Agentic API Seats (Monthly)", "$ 45.00", "$ 450.00"]);
        t.row(vec!["1", "Custom LLM Fine-tuning (GPU Hours included)", "$ 1,200.00", "$ 1,200.00"]);
        t.row(vec!["5", "Consultation Hours", "$ 250.00", "$ 1,250.00"]);
        t.row(vec!["1", "Priority Support License (Yearly)", "$ 600.00", "$ 600.00"]);
    })?;

    pdf.advance_cursor(30.0);

    // --- SUMMARY SECTION ---
    pdf.row(|r| {
        r.col(60.0.pct(), |pdf| {
            pdf.text("NOTES:")
                .size(9.0)
                .color(Color::Rgb(150, 150, 150))
                .margin_bottom(5.0);
            
            pdf.text("Thank you for your business. Please include the invoice number in your payment reference.")
                .size(9.0)
                .color(Color::Rgb(100, 100, 100));
            Ok(())
        });

        r.col(40.0.pct(), |pdf| {
            pdf.table(|t| {
                t.border(TableBorderStyle::None);
                t.widths(vec![60.0.pct(), 40.0.pct()]);
                t.column_align(1, Align::Right);
                
                t.row(vec!["Subtotal:", "$ 6,000.00"]);
                t.row(vec!["Tax (7%):", "$ 420.00"]);
                
                // Use row_builder for the 'Total' to make it bold-like
                t.row_builder(|r| {
                    r.cell("TOTAL DUE:").valign(VAlign::Center);
                    r.cell("$ 6,420.00").valign(VAlign::Center);
                });
            })?;

            // Draw a thick line under total
            let (cx, _cy) = pdf.cursor_pos();
            pdf.set_stroke_color(Color::Rgb(30, 144, 255))?;
            pdf.line(cx, _cy + 5.0, cx + 200.0, _cy + 5.0)?;
            
            Ok(())
        });
    })?;

    // --- FOOTER ---
    pdf.set_cursor(50.0, 50.0); // Move to bottom
    pdf.text("Payment via Bank Transfer: MODERN-SOLUTIONS-CORP-BANK-001")
        .size(8.0)
        .align_center()
        .color(Color::Rgb(180, 180, 180));

    // Finish
    pdf.finish()?;
    println!("Successfully generated invoice_premium.pdf");
    Ok(())
}
