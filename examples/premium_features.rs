use mr_pdf::{Color, Pdf};

fn main() -> std::io::Result<()> {
    let bytes = Pdf::render(|pdf| {
        pdf.set_title("Premium Features Showcase");
        pdf.show_page_numbers = true;

        // 1. Showcase Rounded Rectangles and Shadows (Modern UI Style)
        pdf.text("Modern UI Primitives")
            .size(24.0)
            .align_center()
            .margin_bottom(30.0);

        let (_, y) = pdf.cursor_pos();
        let card_w = 400.0;
        let card_h = 100.0;
        let center_x = (pdf.page_width - card_w) / 2.0;

        // Draw shadow
        pdf.shadow_rect(center_x, y - card_h, card_w, card_h, 5.0)?;

        // Draw card background (Rounded)
        pdf.set_fill_color(Color::Rgb(255, 255, 255))?;
        pdf.fill_rounded_rect(center_x, y - card_h, card_w, card_h, 15.0)?;

        // Draw border
        pdf.set_stroke_color(Color::Rgb(0, 122, 255))?;
        pdf.rounded_rect(center_x, y - card_h, card_w, card_h, 15.0)?;

        // Content inside card
        pdf.set_cursor(center_x + 20.0, y - 30.0);
        pdf.set_fill_color(Color::Rgb(0, 0, 0))?;
        pdf.text("This is a Premium Card").size(18.0);

        pdf.set_cursor(center_x + 20.0, y - 60.0);
        pdf.text("It features rounded corners and a subtle shadow.")
            .size(12.0);

        pdf.set_cursor(center_x, y - card_h - 40.0);

        // 2. Showcase Multiple Pages and Auto-Numbering
        pdf.new_page()?;
        pdf.text("Second Page").size(20.0);
        pdf.text("Look at the bottom of the page! Continuous numbering is automatic.")
            .margin_top(20.0);

        pdf.new_page()?;
        pdf.text("Third Page").size(20.0);
        pdf.text("PDF Compression is also enabled by default now, making this file tiny.")
            .margin_top(20.0);

        Ok(())
    })?;

    std::fs::create_dir_all("preview")?;
    std::fs::write("preview/premium_features.pdf", bytes)?;
    println!("Successfully generated preview/premium_features.pdf with compression and modern UI!");

    Ok(())
}
