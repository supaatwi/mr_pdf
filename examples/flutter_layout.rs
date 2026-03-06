use mr_pdf::{Pdf, pct, pt};
use std::fs::File;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let file = File::create("preview/flutter_layout.pdf")?;
    let writer = BufWriter::new(file);

    let mut pdf = Pdf::new(writer)?;

    pdf.text("Flutter-like Layout (Row & Column)")
        .size(24.0)
        .align_center()
        .margin_bottom(20.0);

    // ── 1. Simple Column ──
    pdf.column(|c| {
        c.text("This is inside a Column").size(16.0);
        c.text("Columns just stack elements vertically.");
    })?;

    pdf.text("").margin_bottom(20.0); // spacer

    // ── 2. Row with Flex (Expanded) ──
    pdf.text("Row with Expanded (flex weights):")
        .size(14.0)
        .margin_bottom(10.0);
    pdf.row(|r| {
        r.expanded(2, |sub| {
            sub.text("Flex 2 (wider)")
                .size(12.0)
                .margin_top(5.0)
                .margin_bottom(5.0);
            Ok(())
        });
        r.expanded(1, |sub| {
            sub.text("Flex 1 (narrower)").size(12.0);
            Ok(())
        });
    })?;

    pdf.text("").margin_bottom(20.0); // spacer

    // ── 3. Row with Mixed Widths ──
    pdf.text("Row with Mixed Widths (Points, Percent, Flex):")
        .size(14.0)
        .margin_bottom(10.0);
    pdf.row(|r| {
        r.col(pt(100.0), |sub| {
            sub.text("Fixed 100pt");
            Ok(())
        });
        r.col(pct(20.0), |sub| {
            sub.text("20%");
            Ok(())
        });
        r.expanded(1, |sub| {
            sub.text("Expanded Remainder");
            Ok(())
        });
    })?;

    pdf.finish()?;

    println!("Successfully wrote flutter_layout.pdf to preview directory");
    Ok(())
}
