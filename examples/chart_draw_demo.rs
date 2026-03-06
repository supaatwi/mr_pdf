use mr_pdf::{Chart, Color, Pdf};
use std::fs::File;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let file = File::create("preview/chart_draw_demo.pdf")?;
    let writer = BufWriter::new(file);

    let mut pdf = Pdf::new(writer)?;

    pdf.text("Chart & Drawing Demonstration")
        .size(20.0)
        .align_center()
        .margin_bottom(20.0);

    // ── 1. Drawing Elements ──
    pdf.text("1. Basic Drawing Elements")
        .size(14.0)
        .margin_bottom(10.0);

    // Draw a house-like shape
    pdf.set_stroke_color(Color::Rgb(0, 0, 0))?;
    pdf.rect(100.0, 500.0, 100.0, 100.0)?; // House base
    pdf.line(100.0, 600.0, 150.0, 650.0)?; // Roof left
    pdf.line(150.0, 650.0, 200.0, 600.0)?; // Roof right

    // Colored shapes
    pdf.set_fill_color(Color::Rgb(255, 100, 100))?;
    pdf.fill_rect(250.0, 500.0, 80.0, 80.0)?;

    pdf.set_fill_color(Color::Rgb(100, 100, 255))?;
    pdf.fill_circle(400.0, 540.0, 40.0)?;

    pdf.set_stroke_color(Color::Rgb(0, 0, 0))?;
    pdf.circle(480.0, 540.0, 30.0)?;

    // ── 2. Chart ──
    pdf.set_cursor(50.0, 400.0);
    pdf.text("2. Chart Component (with Colors & Labels)")
        .size(14.0)
        .margin_bottom(10.0);

    let chart_data = vec![10.0, 50.0, 30.0, 80.0, 20.0, 100.0, 60.0];
    let chart = Chart::new(chart_data)
        .width(400.0)
        .height(150.0)
        .color(Color::Rgb(100, 200, 100))
        .labels(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]);

    pdf.chart(chart)?;

    pdf.finish()?;

    println!("Successfully wrote chart_draw_demo.pdf to preview directory");
    Ok(())
}
