use mr_pdf::{Align, Chart, ChartType, Color, Pdf, pct};
use std::fs::File;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let file = File::create("preview/chart_advanced.pdf")?;
    let writer = BufWriter::new(file);

    let mut pdf = Pdf::new(writer)?;

    pdf.text("Advanced Charting Features")
        .size(24.0)
        .align_center()
        .margin_bottom(30.0);

    let data = vec![10.0, 50.0, 30.0, 80.0, 20.0, 100.0, 60.0];
    let labels = vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

    // 1. Bar Chart - Center Aligned, Percent Width
    pdf.text("1. Bar Chart (Center, 80% width with values)")
        .size(16.0);
    let bar_chart = Chart::new(data.clone())
        .chart_type(ChartType::Bar)
        .width(pct(80.0))
        .height(150.0)
        .align(Align::Center)
        .color(Color::Rgb(54, 162, 235))
        .labels(labels.clone())
        .show_values(true);
    pdf.chart(bar_chart)?;

    pdf.advance_cursor(20.0);

    // 2. Line Chart - Right Aligned
    pdf.text("2. Line Chart (Right, 60% width)").size(16.0);
    let line_chart = Chart::new(data.clone())
        .chart_type(ChartType::Line)
        .width(pct(60.0))
        .height(120.0)
        .align(Align::Right)
        .color(Color::Rgb(255, 99, 132))
        .labels(labels.clone())
        .show_values(true);
    pdf.chart(line_chart)?;

    pdf.new_page()?;

    // 3. Pie Chart
    pdf.text("3. Pie Chart (Left)").size(16.0);
    let pie_chart = Chart::new(vec![30.0, 20.0, 50.0])
        .chart_type(ChartType::Pie)
        .width(200.0)
        .height(200.0)
        .labels(vec!["Rent", "Food", "Savings"])
        .show_values(true);
    pdf.chart(pie_chart)?;

    pdf.finish()?;

    println!("Successfully wrote chart_advanced.pdf to preview directory");
    Ok(())
}
