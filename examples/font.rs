use mr_pdf::{Pdf, Color};
use std::fs::File;

fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("preview").unwrap_or(());
    
    let file = File::create("preview/font_switching.pdf")?;
    let mut pdf = Pdf::stream(file)?;

    // ── 1. Register Multiple Fonts ──
    // Note: Make sure the .ttf files actually exist at the specified paths.
    // For this example, we'll demonstrate switching by registering the SAME font file twice 
    // under different names ("PrimaryFont" and "SecondaryFont").
    // In a real project, you would register visually distinct fonts (e.g., Sarabun-Regular.ttf and Sarabun-Bold.ttf).
    pdf.register_font("PrimaryFont", "font/Sarabun-Regular.ttf")?;
    pdf.register_font("SecondaryFont", "font/Sarabun-Regular.ttf")?;

    // As soon as the first font is registered, it becomes the default font for the document.
    
    // Title using default font (PrimaryFont in this case)
    pdf.text("Example: Managing and Switching Multiple Fonts")
       .size(20.0)
       .margin_bottom(20.0);

    // ── 2. Explicitly switch to SecondaryFont ──
    pdf.text("This paragraph explicitly uses the 'SecondaryFont'.")
       .font("SecondaryFont")
       .size(16.0)
       .color(Color::Rgb(50, 50, 200)) // Blue
       .margin_bottom(15.0);

    // ── 3. Switch back to PrimaryFont (which supports Thai) ──
    pdf.text("ส่วนข้อความนี้สลับกลับมาใช้ฟอนต์ 'PrimaryFont' แล้วครับ รองรับภาษาไทยได้เต็มรูปแบบ!")
       .font("PrimaryFont")
       .size(16.0)
       .color(Color::Rgb(200, 50, 50)) // Red
       .margin_bottom(15.0);

    // ── 4. Mixed inline usage within a Row ──
    pdf.text("You can also mix them quickly using the Layout builder:")
       .font("SecondaryFont")
       .size(14.0)
       .margin_bottom(10.0);

    pdf.row(|row| {
        row.col(200.0, |p| {
            p.text("Column 1: สวัสดี").font("PrimaryFont").size(12.0);
            Ok(())
        });
        row.col(200.0, |p| {
            p.text("Column 2: Hello!").font("SecondaryFont").size(12.0);
            Ok(())
        });
    })?;

    pdf.finish()?;

    println!("Successfully generated 'preview/font_switching.pdf'");
    Ok(())
}
