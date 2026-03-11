use std::fs::File;
use mr_pdf::Pdf;

fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("preview").unwrap_or(());
    
    let file = File::create("preview/markdown_demo.pdf")?;
    let mut pdf = Pdf::stream(file)?;

    // Register a font for better look and to support extended chars if needed
    // You must make sure to curl/have font/Roboto-Regular.ttf
    // In this example, we'll try to use Roboto if it exists.
    let _ = pdf.register_font("Roboto", "font/Roboto-Regular.ttf");

    let markdown_content = "
# Welcome to MR-PDF Markdown!
    
This is a demonstration of the **Markdown** support within the PDF generator. 
Currently, the renderer supports translating block elements like Headings, Paragraphs, Lists, and Code Blocks into clean, native PDF layout elements.

## Features Supported
- **Headings**: Automatically scaled and margined based on level (H1 to H6).
- **Paragraphs**: Text is wrapped automatically based on the page width.
- **Lists**: Cleanly indented bullet points.
- **Code Blocks**: Displayed with a different font color and margin. Note: inline formatting like *italics* or **bold** is rendered as plain text within blocks for now since this is a lightweight engine.

### Let's see a list:
* First item in the list
* Second item in the list
* Third item with a bit more text to demonstrate the wrapping behavior when a list item might get too long for a single line on the PDF document page over to the right.

### Code Example
```rust
fn hello_world() {
    println!(\"Hello from Markdown!\");
}
```

That's it! You can now compose larger texts easily using Markdown formatting, and MR-PDF will handle laying it out into the document naturally alongside your tables and charts.
";

    // Use our new markdown block
    let mut md = pdf.markdown(markdown_content).size(12.0);
    
    // Check if Roboto was successfully set, otherwise fallback to builtin
    if std::path::Path::new("font/Roboto-Regular.ttf").exists() {
        md = md.font("Roboto");
    }
    
    md.render()?;

    pdf.finish()?;
    println!("preview/markdown_demo.pdf created successfully!");

    Ok(())
}
