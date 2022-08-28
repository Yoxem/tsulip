extern crate pango;
extern crate cairo;
use pangocairo;

/// convert font_style string to Style class
/// - font_style : Nommal, Oblique, Italic, or omitted as string.
fn convert_string_to_font_style(font_style_str : String)->pango::Style{
    if font_style_str == "" || font_style_str == "Normal"{
        return pango::Style::Normal;
    }else if font_style_str == "Oblique"{
        return pango::Style::Oblique;
    }else if font_style_str == "Italic"{
        return pango::Style::Italic;
    }else {
        println!("the font-style is illegal: {}, set to Normal", font_style_str);
        return pango::Style::Normal;
    }

}

/// convert font_weight string to Weight class
/// - font_style : Thin,     Ultralight,     Light,     Semilight,     Book, 
///     Normal,     Medium,     Semibold,      Bold,     Ultrabold,     Heavy, 
///    Ultraheavy, or omitted as string.
fn convert_string_to_font_weight(font_weight_str : String)->pango::Weight{
    match font_weight_str.as_str(){
        "" => pango::Weight::Normal,
        "Normal" => pango::Weight::Normal,
        "Thin" => pango::Weight::Thin,
        "Ultralight" => pango::Weight::Ultralight,
        "Light" => pango::Weight::Light,
        "Semilight" => pango::Weight::Semilight,
        "Book" => pango::Weight::Book,
        "Medium" => pango::Weight::Medium,
        "Semibold" => pango::Weight::Semibold,
        "Bold" => pango::Weight::Bold,
        "Ultrabold" => pango::Weight::Ultrabold,
        "Heavy" => pango::Weight::Heavy,
        "Ultraheavy" => pango::Weight::Ultraheavy,
        _ => {println!("the font-weight is illegal: {}, set to Normal", font_weight_str); pango::Weight::Normal}

    }

}


/// put chars to  the pdf
/// - x = position from left in px
/// - y = position from bottom in px
pub(crate) fn put_chars(text : String, x : f64, y : f64 , font_family : String,
    font_style_str : String, font_weight_str : String, color : String, context: &cairo::Context){
    let mut font = pango::FontDescription::new();
    font.set_family(font_family.as_str());

    let font_style = convert_string_to_font_style(font_style_str);
    font.set_style(font_style);

    let weight = convert_string_to_font_weight(font_weight_str);
    font.set_weight(weight);

    println!("{:?}", font);

    let fontmap = pangocairo::FontMap::default().unwrap();
    let pango_context = pango::Context::new();
    let pango_layout = pango::Layout::new(&pango_context);
    pango_context.set_font_map(&fontmap);
    pango_layout.set_font_description(Some(&font));
    pango_layout.set_text(text.as_str());


    context.move_to(x, y);
    pangocairo::show_layout(&context, &pango_layout);  
    // context.save();

}