use mr_pdf::{Color, Pdf, SizeExt, TableBuilder, TableBorderStyle, TableCell};
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

    // Common builder for the tables
    let mut builder = TableBuilder::new();
    builder
        .widths(vec![100.0.pt(), 150.0.pt(), 150.0.pt()])
        .border(TableBorderStyle::HeaderOnly)
        .zebra(Color::Rgb(245, 245, 245))
        .header(vec!["Key", "Timestamp", "Data"]);

    // Start multiplexed streaming
    let mut multi = pdf.multi_table_streaming(builder)?;

    // 2. ปรับแต่งโครงสร้างแยกเฉพาะแต่ละโต๊ะได้เลย
    // สำหรับ "Summary" เราปรับทั้งความกว้างคอลัมน์และลักษณะเส้นขอบผ่าน builder()
    multi.builder("Summary")
        .widths(vec![150.0.pt(), 250.0.pt()])
        .border(TableBorderStyle::Full)
        .header(vec!["Summary Metric", "Details"]);

    // สำหรับโต๊ะอื่นๆ เราอาจจะปรับแค่ Header หรือ Widths ก็ได้
    multi.widths("A", vec![80.0.pt(), 120.0.pt(), 200.0.pt()]);
    multi.header("A", vec!["Vehicle A ID", "Checked at", "Activity Log"]);
    
    multi.widths("B", vec![80.0.pt(), 120.0.pt(), 200.0.pt()]);
    multi.header("B", vec!["Vehicle B ID", "Captured at", "GPS Coordinate"]);
    
    multi.widths("C", vec![80.0.pt(), 120.0.pt(), 200.0.pt()]);
    multi.header("C", vec!["Vehicle C ID", "Updated at", "System Status"]);

    // Simulate interleaved incoming data
    // Summary, A, B, C arriving in random order
    for i in 1..=5 {
        multi.insert("A", vec![
            TableCell::from(format!("A-{}", i)), 
            TableCell::from(format!("10:0{:02}:00", i)), 
            TableCell::from(format!("Movement detected #{}", i))
        ])?;
        
        multi.insert("B", vec![
            TableCell::from(format!("B-{}", i)), 
            TableCell::from(format!("11:1{:02}:00", i)), 
            TableCell::from(format!("GPS update #{}", i))
        ])?;

        // Interleaving some summary data as it's calculated
        if i == 5 {
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
