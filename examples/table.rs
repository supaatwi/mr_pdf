use mr_pdf::Pdf;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::create("preview/table.pdf")?;
    let mut pdf = Pdf::stream(file)?;

    // ── 1. Basic table ───────────────────────────────────────────────────────
    pdf.text("1. Basic Table").size(16.0);
    pdf.advance_cursor(8.0);

    pdf.table(|t| {
        t.widths(vec![180.0, 100.0, 215.0]);
        t.repeat_header(true);
        t.header(vec!["Product Name", "Price", "Country of Origin"]);

        let rows = vec![
            ("Mechanical Keyboard Cherry MX", "$149", "Germany"),
            ("4K Ultrawide Monitor 34\"", "$699", "South Korea"),
            ("Wireless Noise-Cancelling Headphones", "$299", "Japan"),
            ("USB-C Docking Station 12-in-1", "$89", "China"),
            ("Ergonomic Office Chair Pro", "$450", "Sweden"),
        ];
        for (name, price, country) in rows {
            t.row(vec![name, price, country]);
        }
    })?;

    pdf.advance_cursor(24.0);

    // ── 2. Column spanning ───────────────────────────────────────────────────
    pdf.text("2. Column Spanning (colspan)").size(16.0);
    pdf.advance_cursor(8.0);

    pdf.table(|t| {
        t.widths(vec![120.0, 120.0, 120.0, 120.0]);

        // Row spanning all 4 columns
        t.row_builder(|row| {
            row.cell("Full-Width Merged Header (colspan=4)").span(4);
        });

        // Row: 2 + 2
        t.row_builder(|row| {
            row.cell("Section A (colspan=2)")
                .span(2)
                .cell("Section B (colspan=2)")
                .span(2);
        });

        // Row: 1 + 2 + 1
        t.row_builder(|row| {
            row.cell("Normal")
                .cell("Merged Middle (colspan=2)")
                .span(2)
                .cell("Normal");
        });

        // Normal header row
        t.row(vec!["Col 1", "Col 2", "Col 3", "Col 4"]);

        // Regular rows
        for i in 1..=4 {
            t.row_builder(|row| {
                for j in 1..=4 {
                    row.cell(&format!("R{}C{}", i, j));
                }
            });
        }
    })?;

    pdf.advance_cursor(24.0);

    // ── 3. Invoice-style layout ──────────────────────────────────────────────
    pdf.text("3. Invoice-Style Table with Spanning").size(16.0);
    pdf.advance_cursor(8.0);

    pdf.table(|t| {
        t.widths(vec![240.0, 80.0, 80.0, 95.0]);

        // Invoice header spanning all columns
        t.row_builder(|row| {
            row.cell("INVOICE #INV-2025-001  |  Date: 2025-03-05  |  Due: 2025-04-05")
                .span(4);
        });

        // Column headers
        t.row(vec!["Description", "Qty", "Unit Price", "Total"]);

        // Line items
        let items: Vec<(&str, u32, f64)> = vec![
            ("Laptop M3 Pro 14-inch (Space Black)", 1, 1999.00),
            ("USB-C Docking Station 12-in-1", 2, 89.00),
            ("Thunderbolt 4 Cable 1.8m", 3, 39.00),
            ("Extended Warranty Plan (3 Years)", 1, 299.00),
            ("Express Shipping & Handling", 1, 49.00),
        ];

        for (desc, qty, unit) in &items {
            let total = *qty as f64 * unit;
            t.row_builder(|row| {
                row.cell(desc)
                    .cell(&qty.to_string())
                    .cell(&format!("${:.2}", unit))
                    .cell(&format!("${:.2}", total));
            });
        }

        // Subtotal row — label spans 3 cols
        let subtotal: f64 = items.iter().map(|(_, q, u)| *q as f64 * u).sum();
        t.row_builder(|row| {
            row.cell("Subtotal")
                .span(3)
                .cell(&format!("${:.2}", subtotal));
        });

        // Tax
        let tax = subtotal * 0.07;
        t.row_builder(|row| {
            row.cell("Tax (7%)").span(3).cell(&format!("${:.2}", tax));
        });

        // Grand total
        t.row_builder(|row| {
            row.cell("GRAND TOTAL")
                .span(3)
                .cell(&format!("${:.2}", subtotal + tax));
        });
    })?;

    pdf.advance_cursor(24.0);

    // ── 4. Auto-paginating with 100 rows ────────────────────────────────────
    pdf.text("4. 100 Rows — Auto Page Break + Repeat Header")
        .size(16.0);
    pdf.advance_cursor(8.0);

    pdf.table(|t| {
        t.widths(vec![50.0, 220.0, 75.0, 75.0, 75.0]);
        t.repeat_header(true);
        t.header(vec!["#", "Description", "Qty", "Unit Price", "Total"]);

        for i in 1..=100 {
            let qty = (i % 9) + 1u32;
            let unit = (i as f64 * 6.5 + 12.0) * 0.99;
            t.row_builder(|row| {
                row.cell(&i.to_string())
                    .cell(&format!("Item No. {:04} — some longer description text", i))
                    .cell(&qty.to_string())
                    .cell(&format!("${:.2}", unit))
                    .cell(&format!("${:.2}", qty as f64 * unit));
            });
        }
    })?;

    pdf.finish()?;
    println!("Successfully generated 'preview/table.pdf'");
    Ok(())
}
