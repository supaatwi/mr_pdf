use mr_pdf::{Pdf, PdfPermissions};
use std::fs::File;

fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("preview").unwrap_or(());

    // NOT YET IMPLEMENTED IN LIBRARY
    // JUST A DESIGN PREVIEW AS REQUESTED
    
    let file = File::create("preview/secured.pdf")?;
    let mut pdf = Pdf::stream(file)?;

    pdf.set_encryption("owner_pass", Some("user_pass"), PdfPermissions {
        can_print: false,
        can_copy: false,
        ..Default::default()
    });

    pdf.text("This is Top Secret!")
        .size(30.0)
        .align_center();

    pdf.text("When the encryption feature is implemented, this file will ask for a password when you open it in Acrobat Reader.")
        .size(14.0)
        .margin_bottom(20.0);

    // This is an example of the desired API. 
    // Generating password protection using standard PDF 1.4 RC4 128-bit encryption
    // is a massive change that touches the cross-reference tables and random ID generation.
    // We would insert the Encrypt dictionary in the Trailer, and AES/RC4 encrypt every String and Stream inside writer.rs

    pdf.finish()?;

    println!("Successfully generated 'preview/secured.pdf'");
    Ok(())
}
