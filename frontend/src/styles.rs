use stylist::{style, Style};
use web_sys;

// Define color variables for our atomic-age/neubrutalist theme
pub mod colors {
    pub const PRIMARY: &str = "#FF3D00";       // Vibrant orange-red
    pub const SECONDARY: &str = "#2B2B2B";     // Dark gray
    pub const ACCENT: &str = "#FFCC00";        // Yellow
    pub const BACKGROUND: &str = "#F7F7F7";    // Light gray
    pub const TEXT: &str = "#202020";          // Near black
    pub const TEXT_LIGHT: &str = "#FFFFFF";    // White
    pub const SUCCESS: &str = "#4CAF50";       // Green
    pub const ERROR: &str = "#F44336";         // Red
    pub const WARNING: &str = "#FF9800";       // Orange
    pub const INFO: &str = "#2196F3";          // Blue
}

// Typography
pub mod typography {
    pub const HEADING_FONT: &str = "'Space Grotesk', sans-serif";
    pub const BODY_FONT: &str = "'Inter', sans-serif";
}

// Border styles
pub mod borders {
    pub const BORDER_RADIUS: &str = "0";      // Sharp corners for neubrutalist style
    pub const BORDER_WIDTH: &str = "3px";     // Thick borders
    pub const BOX_SHADOW: &str = "4px 4px 0 #000000"; // Offset shadow for depth
}

// Add Google Fonts
fn load_google_fonts() {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(head) = document.head() {
                // Check if the font is already loaded
                if head.query_selector("link[href*='fonts.googleapis.com']").ok().flatten().is_none() {
                    if let Ok(link) = document.create_element("link") {
                        let _ = link.set_attribute("rel", "stylesheet");
                        let _ = link.set_attribute("href", 
                            "https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;700&family=Inter:wght@400;500;700&display=swap");
                        let _ = head.append_child(&link);
                    }
                }
            }
        }
    }
}

// Define our global styles
pub fn global_style() -> Style {
    // Load Google Fonts
    load_google_fonts();
    
    // Create CSS without @import
    style!(
        r#"
        /* Base styles */
        *, *::before, *::after {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }
        
        html {
            font-size: 16px;
        }
        
        body {
            font-family: ${body_font};
            color: ${text};
            background-color: ${background};
            line-height: 1.5;
            min-height: 100vh;
        }
        
        h1, h2, h3, h4, h5, h6 {
            font-family: ${heading_font};
            font-weight: 700;
            margin-bottom: 1rem;
        }
        
        h1 {
            font-size: 2.5rem;
        }
        
        h2 {
            font-size: 2rem;
        }
        
        h3 {
            font-size: 1.5rem;
        }
        
        a {
            color: ${primary};
            text-decoration: none;
            font-weight: 500;
        }
        
        a:hover, a:focus {
            text-decoration: underline;
        }
        
        button {
            font-family: ${heading_font};
            font-weight: 700;
            background-color: ${primary};
            color: ${text_light};
            border: ${border_width} solid #000;
            border-radius: ${border_radius};
            padding: 0.75rem 1.5rem;
            cursor: pointer;
            font-size: 1rem;
            box-shadow: ${box_shadow};
            transition: transform 0.2s, box-shadow 0.2s;
        }
        
        button:hover {
            transform: translate(2px, 2px);
            box-shadow: 2px 2px 0 #000000;
        }
        
        button:active {
            transform: translate(4px, 4px);
            box-shadow: none;
        }
        
        input, select, textarea {
            font-family: ${body_font};
            font-size: 1rem;
            padding: 0.75rem;
            border: ${border_width} solid #000;
            border-radius: ${border_radius};
            background-color: white;
            width: 100%;
        }
        
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 0 1rem;
        }
        
        .card {
            background-color: white;
            border: ${border_width} solid #000;
            border-radius: ${border_radius};
            padding: 1.5rem;
            margin-bottom: 1.5rem;
            box-shadow: ${box_shadow};
        }
        
        /* Mobile responsiveness */
        @media (max-width: 768px) {
            html {
                font-size: 14px;
            }
            
            .container {
                padding: 0 0.75rem;
            }
        }
        
        .text-decoration-none {
            text-decoration: none;
            color: inherit;
        }
        "#,
        body_font = typography::BODY_FONT,
        heading_font = typography::HEADING_FONT,
        primary = colors::PRIMARY,
        text = colors::TEXT,
        text_light = colors::TEXT_LIGHT,
        background = colors::BACKGROUND,
        border_radius = borders::BORDER_RADIUS,
        border_width = borders::BORDER_WIDTH,
        box_shadow = borders::BOX_SHADOW,
    ).expect("Failed to create global style")
}

// Button styles
pub fn primary_button() -> Style {
    style!(
        r#"
        background-color: ${primary};
        color: ${text_light};
        "#,
        primary = colors::PRIMARY,
        text_light = colors::TEXT_LIGHT,
    ).expect("Failed to create primary button style")
}

pub fn secondary_button() -> Style {
    style!(
        r#"
        background-color: ${secondary};
        color: ${text_light};
        "#,
        secondary = colors::SECONDARY,
        text_light = colors::TEXT_LIGHT,
    ).expect("Failed to create secondary button style")
}

pub fn accent_button() -> Style {
    style!(
        r#"
        background-color: ${accent};
        color: ${text};
        "#,
        accent = colors::ACCENT,
        text = colors::TEXT,
    ).expect("Failed to create accent button style")
}

// Card styles
pub fn card() -> Style {
    style!(
        r#"
        background-color: white;
        border: ${border_width} solid #000;
        border-radius: ${border_radius};
        padding: 1.5rem;
        margin-bottom: 1.5rem;
        box-shadow: ${box_shadow};
        "#,
        border_radius = borders::BORDER_RADIUS,
        border_width = borders::BORDER_WIDTH,
        box_shadow = borders::BOX_SHADOW,
    ).expect("Failed to create card style")
}
