# mr-pdf 📄✨

**mr-pdf** is a high-performance, lightweight PDF generation library for Rust. Designed with a focus on low memory footprint and premium aesthetics, it allows you to create stunning PDF documents with a fluent, developer-friendly API.

## 🚀 Key Features

- **Low Memory Footprint**: Uses a streaming architecture that writes document sections directly to the output. Memory usage scales with the complexity of a single page, not the entire document.
- **Premium Aesthetics**: Built-in support for advanced layouts, charts, and modern typography.
- **Flexible Layout Engine**: Flutter-inspired rows and columns (`Pdf::row`, `Pdf::column`) for complex designs.
- **Interactive Elements**: Support for clickable hyperlinks in text, tables, and images.
- **Advanced Tables**: Easy-to-use table builder with column spanning, striped rows, and auto-wrapping.
- **Rich Media**: Support for JPEG, PNG images (file or Base64) and SVG rendering.
- **Dynamic Charts**: Generate beautiful Bar, Line, and Pie charts directly in your PDF (Optional).

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mr-pdf = "0.1.0"
```

To enable charting capabilities, use the `chart` feature:

```toml
[dependencies]
mr-pdf = { version = "0.1.0", features = ["chart"] }
```

## 🛠️ Quick Start

```rust
use mr_pdf::{Pdf, Color, Align, PaperSize};

fn main() -> std::io::Result<()> {
    // Create a new PDF writer (to file or memory)
    let mut pdf = Pdf::memory()?;
    
    // Set Document Metadata
    pdf.set_title("My Awesome Report");
    pdf.set_author("Antigravity");

    // Add Text with Styling
    pdf.text("Hello, mr-pdf!")
        .size(24.0)
        .align_center()
        .color(Color::Rgb(30, 144, 255)) // Dodger Blue
        .margin_bottom(20.0);

    // Create a Responsive Row
    pdf.row(|r| {
        r.col(50.0.pct(), |pdf| {
            pdf.text("This is the left column.")
               .link("https://github.com");
            Ok(())
        });
        r.col(50.0.pct(), |pdf| {
            pdf.text("This is the right column with a red background.")
               .color(Color::Rgb(255, 0, 0));
            Ok(())
        });
    })?;

    // Finalize
    let _bytes = pdf.finish()?;
    Ok(())
}
```

## 📊 Visual Components

### Beautiful Charts (Opt-in)
Easily visualize data with built-in chart types by enabling the `chart` feature:
- **Bar Charts**: For comparison.
- **Line Charts**: For trends.
- **Pie Charts**: For proportions.

```rust
#[cfg(feature = "chart")]
{
    use mr_pdf::{Chart, ChartType};
    pdf.chart(Chart::new(vec![10.0, 20.0, 30.0])
        .chart_type(ChartType::Pie)
        .labels(vec!["A", "B", "C"])
        .show_values(true));
}
```

### Flexible Image & SVG Sizing
```rust
use mr_pdf::SizeExt;

// SVG with custom positioning and size (supports Points or Percent)
pdf.svg("logo.svg")
    .width(50.0.pct())
    .render()?;

// Images (PNG/JPG) with specific position and size
pdf.image("photo.jpg")
    .position(100.0, 500.0)
    .size(200.0, 150.0)
    .render()?;
```

### Tables with Spanning & Advanced Headers
```rust
use mr_pdf::{TableBorderStyle, Color, SizeExt, Align, VAlign};

pdf.table(|t| {
    t.widths(vec![33.0.pct(), 33.0.pct(), 33.0.pct()]);
    
    // Support for multiple header rows and rowspan via `header_row_builder`
    t.header_row_builder(|r| {
        r.cell("Multi-Row Header").span(2).rowspan(2).align(Align::Center).valign(VAlign::Center);
        r.cell("Col 3 Top");
    });
    t.header_row_builder(|r| {
        r.cell("Col 3 Bottom"); // Only provide remaining cells since span(2) rowspan(2) occupies earlier slots
    });
    
    // Customize Borders
    t.border(TableBorderStyle::Ghost); // Ghost style (no vertical lines)
    
    // Enable Zebra Striping
    t.zebra(Color::Rgb(240, 240, 240)); // Light Gray
    
    // Regular rows
    t.row_builder(|r| {
        r.cell("Spanned Cell").span(2); // Spans 2 columns horizontally
        r.cell("Single Cell");
    });
    t.row(vec!["Row 2", "Data", "Note"]);
})?;
```

## 🏎️ Performance Objectives

`mr-pdf` is tailored for environments where resource efficiency is paramount:
1. **Direct Writing**: Offsets and object metadata are managed centrally, but content streams are flushed directly to the disk/socket once a page is complete.
2. **Minimal Buffering**: Does not store the entire document DOM in memory.
3. **OptimizedTTF**: Only necessary font metrics are loaded into memory for layout calculation.

## 📄 License

MIT / Apache-2.0
