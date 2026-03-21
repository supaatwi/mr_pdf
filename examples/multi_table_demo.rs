use mr_pdf::{Pdf, SizeExt, TableBorderStyle, TableBuilder, TableCell, Color, TitleStyle, Align};
use std::fs::File;

fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("preview").unwrap_or(());
    
    let file = File::create("preview/multi_table_demo.pdf")?;
    let mut pdf = Pdf::stream(file)?;

    pdf.text("Multiplexed Table Streaming Demo")
        .size(24.0)
        .align_center()
        .margin_bottom(20.0);

    pdf.text("This demo shows how to stream rows for multiple tables (e.g. A, B, C and Summary) \
              even if data arrives interleaved, using disk-buffering to save RAM.")
        .size(12.0)
        .margin_bottom(20.0);

    // Register a font if available (e.g. Thai Sarabun)
    let font_id = pdf.register_font("Sarabun", "font/Sarabun-Regular.ttf").ok();

    // 1. Common builder for the tables
    let mut builder = TableBuilder::new();
    builder
        .widths(vec![100.0.pt(), 150.0.pt(), 150.0.pt()])
        .border(TableBorderStyle::HeaderOnly)
        .zebra(Color::Rgb(245, 245, 245))
        .header(vec!["Key", "Timestamp", "Data"]);

    // Start multiplexed streaming
    let mut multi = pdf.multi_table_streaming(builder)?;

    // 2. ปรับแต่งโครงสร้างและสไตล์แยกเฉพาะโต๊ะ
    // แสดง Key เป็นชื่อหัวตารางเป็นค่าเริ่มต้น แต่เราจะปิดสำหรับ Summary
    multi.show_keys(true);
    multi.show_key("Summary", false);

    // ปรับแต่งสีและขนาดของหัวข้อ Key (เช่น A, B, C)
    let mut title_style = TitleStyle::default();
    title_style.size = 18.0;
    title_style.color = Some(Color::Rgb(0, 100, 200)); // สีน้ำเงินพรีเมียม
    if let Some(fid) = font_id { title_style.font = Some(fid); }
    
    multi.title_style(title_style);

    // สำหรับ "Summary"
    multi.builder("Summary")
        .widths(vec![150.0.pt(), 250.0.pt()])
        .border(TableBorderStyle::Full)
        .header(vec!["Summary Metric", "Details"]);

    // ทดสอบการใช้ Font เฉพาะเจาะจงในโต๊ะ "A"
    if let Some(fid) = font_id {
        multi.builder("A").row_style().font(fid);
        multi.builder("A").header_style().font(fid);
    }
    
    multi.widths("A", vec![80.0.pt(), 120.0.pt(), 200.0.pt()]);
    multi.header("A", vec!["Vehicle A ID", "Checked at", "Activity Log"]);

    // Simulate interleaved incoming data
    // เพิ่มจำนวนข้อมูลขึ้นเพื่อให้ล้นไปหน้า 2 (Page Break Test)
    for i in 1..=40 {
        multi.insert("A", vec![
            TableCell::from(format!("A-{}", i)), 
            TableCell::from(format!("10:0{:02}:00", i)), 
            TableCell::from(format!("Activity #{}", i)).link("https://github.com/supaatwi/mr-pdf")
        ])?;
        
        multi.insert("B", vec![
            TableCell::from(format!("B-{}", i)), 
            TableCell::from(format!("11:1{:02}:00", i)), 
            TableCell::from(format!("GPS update #{}", i))
        ])?;

        if i == 40 {
            multi.insert("Summary", vec![
                TableCell::from("TOTAL VEHICLES"),
                TableCell::from("3 (A, B, C)")
            ])?;
            multi.insert("Summary", vec![
                TableCell::from("STATUS"),
                TableCell::from("All systems operational")
            ])?;
        }

        multi.insert("C", vec![
            TableCell::from(format!("C-{}", i)), 
            TableCell::from(format!("12:2{:02}:00", i)), 
            TableCell::from(format!("Status OK #{}", i))
        ])?;
    }

    // Explicitly set order so Summary appears first in the PDF
    multi.order(vec!["Summary", "A", "B", "C"]);

    // Render everything
    multi.render(&mut pdf)?;

    pdf.finish()?;

    println!("Successfully generated 'preview/multi_table_demo.pdf'");
    Ok(())
}
