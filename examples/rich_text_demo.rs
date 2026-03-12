use mr_pdf::{Align, Color, Pdf};
use std::fs::File;

fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("preview").unwrap_or(());

    let file = File::create("preview/rich_text_demo.pdf")?;
    let mut pdf = Pdf::stream(file)?;

    // You can register your custom font here
    // For true bold, register the font with name "MyFont" and "MyFont-Bold" 
    // Example: 
    pdf.register_font("Sarabun", "font/Sarabun-Regular.ttf")?;
    // pdf.register_font("Sarabun-Bold", "font/Sarabun-Bold.ttf")?;

    pdf.text("Rich Text API Demonstration")
        .size(24.0)
        .align_center()
        .margin_bottom(20.0);

    // Using Option 3: Closure Builder for inline styles
    pdf.rich_text(|rt| {
        rt.span("นี่คือข้อความปกติที่มีคำว่า ");
        rt.span("หนา").bold();
        rt.span(" และคำที่มี ");
        rt.span("สีแดง").color(Color::Rgb(255, 0, 0));
        rt.span(" ผสมอยู่ และยังสามารถจัดเป็น ");
        rt.span("สีน้ำเงินหนา").bold().color(Color::Rgb(0, 0, 255));
        rt.span(" ได้อีกด้วย");
    })
    .size(16.0)
    .align(Align::Center)
    .margin_bottom(30.0);
    
    // Another paragraph demonstrating wrapping with mixed inline styles
    pdf.line_height(1.0, |p| {
        p.rich_text(|rt| {
            rt.span("The mr-pdf library is ").color(Color::Rgb(50, 50, 50));
            rt.span("extremely fast").bold().color(Color::Rgb(0, 150, 0));
            rt.span(" and memory efficient. With the new ");
            rt.span("Rich Text API").bold().color(Color::Rgb(200, 100, 0));
            rt.span(", you can precisely control styles for individual words or phrases directly inline without creating complex layouts or nested rows. ");
            rt.span("It supports word wrapping automatically ").color(Color::Rgb(100, 100, 100));
            rt.span("even when styles change mid-sentence.");
        })
        .size(12.0)
        .align(Align::Left)
        .margin_left(40.0)
        .margin_right(40.0)
        .margin_bottom(10.0);
    });

    pdf.finish()?;

    println!("Successfully wrote rich_text_demo.pdf to preview directory");
    Ok(())
}
