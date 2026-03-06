use mr_pdf::{PageNumberPosition, Pdf};

fn main() -> std::io::Result<()> {
    // Example: Top Right Page Numbering
    let bytes = Pdf::render(|pdf| {
        pdf.set_title("Page Numbering Example");

        // 1. Enable page numbers
        pdf.show_page_numbers = true;

        // 2. Set position (Try: TopLeft, TopCenter, TopRight, BottomLeft, BottomCenter, BottomRight)
        pdf.page_number_position = PageNumberPosition::TopRight;

        pdf.text("Page Numbering Demo").size(24.0).align_center();

        pdf.text("Look at the Top-Right corner of each page!")
            .margin_top(20.0);

        pdf.new_page()?;
        pdf.text("Second Page Content").size(18.0);

        pdf.new_page()?;
        pdf.text("Third Page Content").size(18.0);

        Ok(())
    })?;

    std::fs::create_dir_all("preview")?;
    std::fs::write("preview/page_numbers_custom.pdf", bytes)?;
    println!("Successfully generated preview/page_numbers_custom.pdf with Top-Right numbering!");

    Ok(())
}
