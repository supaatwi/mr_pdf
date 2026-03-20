use mr_pdf::{Color, Pdf};
use std::fs::File;

fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("preview").unwrap_or(());

    let file = File::create("preview/box_demo.pdf")?;
    let mut pdf = Pdf::stream(file)?;

    pdf.text("Box Component Demonstration")
        .size(24.0)
        .align_center()
        .margin_bottom(30.0);

    // Box with custom width and center alignment
    pdf.box_layout(|b| {
        b.width(300.0) // 300 points wide
         .align(mr_pdf::Align::Center)
         .padding(15.0)
         .bg_color(Color::Rgb(240, 255, 240))
         .border(1.0, Color::Rgb(0, 150, 0))
         .content(|pdf| {
            pdf.text("Centered Box with 300pt Width").align_center().bold();
            Ok(())
         })
    })?;

    pdf.advance_cursor(30.0);

    // Rounded Box
    pdf.box_layout(|b| {
        b.padding(15.0)
         .border_radius(10.0)
         .bg_color(Color::Rgb(255, 240, 240))
         .border(2.0, Color::Rgb(255, 100, 100))
         .content(|pdf| {
            pdf.text("Rounded Corners!")
               .size(18.0)
               .color(Color::Rgb(200, 0, 0));
            pdf.text("Multiple lines of text automatically wrap inside the box if you use paragraph or text with max_width.")
               .size(11.0);
            Ok(())
         })
    })?;

    pdf.advance_cursor(30.0);

    // Nested Boxes
    pdf.box_layout(|b| {
        b.padding(20.0)
         .bg_color(Color::Rgb(240, 240, 240))
         .content(|pdf| {
            pdf.text("Outer Box").size(12.0).bold();
            
            pdf.box_layout(|inner| {
                inner.padding(10.0)
                     .bg_color(Color::Rgb(255, 255, 255))
                     .border(1.0, Color::Rgb(200, 200, 200))
                     .content(|pdf| {
                         pdf.text("Inner Box").size(10.0);
                         Ok(())
                     })
            })?;
            
            Ok(())
         })
    })?;

    pdf.finish()?;

    println!("Successfully wrote box_demo.pdf to preview directory");
    Ok(())
}
