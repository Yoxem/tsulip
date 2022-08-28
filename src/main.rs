extern crate pango;
extern crate cairo;
use std::env;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use clap::{Arg, App};

mod pdf_interface;

/*
ls = ["(",  "3", ")", "1", ")"] 


def consume(ls):
    return ls[1:]

def parse(): 
     global ls
     if ls[0] == "(": 
         ls = consume(ls) 
         a = [] 
         print(ls) 
         while ls[0] != ")": 
             b = parse() 
             a.append(b)
         ls = consume(ls)
         return a 
     else: 
          
         a =  ls[0] 
         ls = consume(ls) 
         return a
g = parse()
if ls != []:
    print("Unexpected right parenthesis ).")
*/

/*
mod author;
use author::* */

/// DataToken store the tokenized data.
/// - line_no : num of line
/// - col_no : num of col
/// - text : text as string
#[derive(Debug)]
struct DataToken{
    text: String,
    line_no: i32,
    col_no: i32,
}

/// DatatokenAST - Storing Data token AST.
/// Node -> for a node
/// Tree -> for a subtree
enum DataTokenAST {
    Node(DataToken),
    Tree(Vec<DataTokenAST>),

}

fn tokenizer(data: String) -> Vec<DataToken>{
    // pattern list. int, float, parenthesis left and right, spaces, identifier, and string
    let pattern_list = vec!["\\d+", "\\d+[.]\\d+", "[(]", "[)]", "[ \t\n]", "[^() \t]+", "\"([ \t]|[^\\\\]|\\\\)*\""];
    let joined_string = pattern_list.join("|"); // join the string.
    let regex_string = format!(r"({})", joined_string);
    let re_pattern  = Regex::new(&regex_string).unwrap();

    let mut token_list : Vec<DataToken> = vec![];

    let mut line_no = 1;
    let mut col_no: i32 = 1;
    for cap in re_pattern.captures_iter(&data) {
        let text = &cap[0];
        let mut text_length: i32 = (text.chars().count()) as i32;
        println!("Month: {}", &cap[0]);
        let data_token = DataToken{text : (text.to_string()), line_no: line_no, col_no:col_no};
        token_list.push(data_token);

        if text == "\n"{
            col_no = 1;
            line_no += 1;
        }else{
            col_no += text_length;
        }

    }

    return token_list;
}

/// parsing the token to let it be a AST.
/// - data_token_list : the list of tokens.
/// return DataTokenAST.
fn token_parser(data_token_list : Vec<DataToken>){
    // remove the spaces
    let data_token_list_remove_space = data_token_list
        .iter().filter(| x|{
            if (vec!["\n", "\t", " "].contains(&(x.text.as_str())))
                {return false;}
                {return true;}});
    println!("{:?}", data_token_list_remove_space);
}

///
/// main function.
fn main()  {

    // version number of the typesetter.
    let version_num = "0.0.1";

    // input command line
    let args_matches = App::new("tsulip")
    .version(version_num)
    .author("Tan, Kian-ting <yoxem.tem98@nctu.edu.tw>")
    .about("A litter typesetter.")
    .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .takes_value(true)
            .help("input filepath"))
    .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .help("destination pdf output."))
    .get_matches();




    let mut input_file_path_raw = args_matches.values_of_lossy("input");

    let mut input_file_path = "".to_string();

    match input_file_path_raw {
        Some(x) => input_file_path = x[0].clone(),
        None => {println!("missing input path."); std::process::exit(1) ;}
    }

    let input_file_path_pattern = Regex::new(r"^(.+)[.]tsu$").unwrap();

    // if file name unmatched, shoew the error:
    if !(input_file_path_pattern.is_match(&input_file_path)){
        println!("the file_name {} doesn't match the pattern. It should be \".tsu\".", input_file_path);
        std::process::exit(1) ;
    }

    let data = fs::read_to_string(input_file_path.clone()).expect((format!("Error: unable to read the file in {}", input_file_path)).as_str());

    let tokenized = tokenizer(data);








    // destination of the pdf.
    let default_output_pdf_path = format!("{}{}", &input_file_path[0..input_file_path.chars().count()-3], "pdf");

    let mut output_pdf_path_raw = args_matches.values_of_lossy("output")
        .unwrap_or(vec![default_output_pdf_path.to_string()]);
    let output_pdf_path = &output_pdf_path_raw[0];

    let pdf_size = "a4";

    let mut size_table = HashMap::new();

    //
    // pepersize -> px (w, h) comparison table
    size_table.insert("a4", [595.0, 842.0]);

    // setting pdf_size.
    let mut pdf_width = 595.0;
    let mut pdf_height = 842.0;
    if !size_table.contains_key(pdf_size){
        println!("the size {} is unfound. using a4 size.", pdf_size);}
    else{
        pdf_width = size_table[pdf_size][0];
        pdf_height = size_table[pdf_size][1];
    }

    println!("{}", pdf_width);

    // create page.
    let surface = cairo::PdfSurface::new(110.0, 110.0, "/tmp/a.pdf").expect("Couldn't create surface!");
    let ctx = cairo::Context::new(&surface).expect("running error");
    ctx.scale(110.0, 110.0);  // Normalizing the canvas

    ctx.set_source_rgba(0.8, 1.0, 1.0, 0.5); // 設定顏色
        ctx.paint().ok();// 設定背景顏色
    ctx.save();

    /*
    let string = "愛".to_string();
    let x = 100.0;
    let y = 100.0;
    let font_family = "Noto Sans CJK TC".to_string();
    let font_style = "".to_string();
    let font_weight  = "".to_string();
    let color = "#ffffff".to_string();
    */
    // pdf_interface::put_chars(string, x, y , font_family, font_style, font_weight, color, &cxt);



    std::process::exit(0) ;
}


