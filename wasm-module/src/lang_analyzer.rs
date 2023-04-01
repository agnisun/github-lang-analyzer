use std::collections::HashMap;
use std::{cmp};
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlCanvasElement, CanvasRenderingContext2d};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn calculate_sum(values: &HashMap<String, f64>) -> f64 {
    let mut sum = 0.0;

    for val in values.values() {
        sum += val;
    }

    sum
}

fn convert_to_percent(values: &mut HashMap<String, f64>, sum: f64) {
    for (_, val) in values.iter_mut() {
        let t = val.clone() / sum;
        let t = f64::from(t) * 100.0;
        
        *val = (t * 10.0).round() / 10.0;
    }
}

fn get_canvas_and_context(document:&Document, id: &str) -> Result<(HtmlCanvasElement, CanvasRenderingContext2d), String> {
    let canvas = match document.get_element_by_id(id) {
        Some(canvas) => canvas,
        _ => {
            return Err("no canvas element".to_string());
        }
    };

    let canvas: web_sys::HtmlCanvasElement = match canvas
        .dyn_into::<web_sys::HtmlCanvasElement>() {
        Ok(element) => element,
        Err(_) => {
            return Err("type conversion cannot be performed".to_string());
        }
    };

    let context = match canvas.get_context("2d") {
        Ok(context) => match context {
            Some(context) => context,
            _ => {
                return Err("context identifier is not supported, or the canvas has already been set to a different context mode".to_string());
            }
        },
        Err(_) => {
            return Err("context identifier is not supported, or the canvas has already been set to a different context mode".to_string());
        }
    };
    let context = match context.dyn_into::<CanvasRenderingContext2d>() {
        Ok(element) => element,
        Err(_) => {
            return Err("type conversion cannot be performed".to_string());
        }
    };

    Ok((canvas, context))
}

fn get_max(values: &HashMap<String, f64>) -> Result<f64, String> {
    let mut max_value = match values.values().next() {
        Some(value) => value,
        _ => {
            return Err("hash map is empty".to_string());
        }
    };

    for (_, value) in values.iter().skip(1) {
        if value > max_value {
            max_value = value;
        }
    }

    Ok(max_value.clone())
}

pub fn get_values(values: JsValue) -> Result<HashMap<String, f64>, String> {
    let mut values: HashMap<String, f64> = match serde_wasm_bindgen::from_value(values) {
        Ok(value) => value,
        Err(_) => {
            return Err("incorrect type".to_string())
        }
    };

    let sum = calculate_sum(&values);
    convert_to_percent(&mut values, sum);
    Ok(values)
}

fn get_color(key: &str) -> &str {
    let colors:HashMap<&str, &str> = HashMap::from([("ABAP", "#E8274B"),("AGS Script", "#B9D9FF"),("AMPL", "#E6EFBB"),("ANTLR", "#9DC3FF"),("API Blueprint", "#2ACCA8"),("APL", "#5A8164"),("ASP", "#6a40fd"),("ATS", "#1ac620"),("ActionScript", "#882B0F"),("Ada", "#02f88c"),("Agda", "#315665"),("Alloy", "#64C800"),("Arc", "#aa2afe"),("Arduino", "#bd79d1"),("AspectJ", "#a957b0"),("Assembly", "#6E4C13"),("AutoHotkey", "#6594b9"),("AutoIt", "#1C3552"),("BlitzMax", "#cd6400"),("Boo", "#d4bec1"),("Brainfuck", "#2F2530"),("C", "#555555"),("C Sharp", "#178600"),("CSS", "#563d7c"),("Chapel", "#8dc63f"),("Cirru", "#ccccff"),("Clarion", "#db901e"),("Clean", "#3F85AF"),("Click", "#E4E6F3"),("Clojure", "#db5855"),("CoffeeScript", "#244776"),("ColdFusion", "#ed2cd6"),("ColdFusion CFC", "#ed2cd6"),("Common Lisp", "#3fb68b"),("Component Pascal", "#b0ce4e"),("Crystal", "#776791"),("D", "#ba595e"),("DM", "#447265"),("Dart", "#00B4AB"),("Diff", "#88dddd"),("Dogescript", "#cca760"),("Dylan", "#6c616e"),("E", "#ccce35"),("ECL", "#8a1267"),("Eagle", "#814C05"),("Eiffel", "#946d57"),("Elixir", "#6e4a7e"),("Elm", "#60B5CC"),("Emacs Lisp", "#c065db"),("EmberScript", "#FFF4F3"),("Erlang", "#B83998"),("F#", "#b845fc"),("FLUX", "#88ccff"),("FORTRAN", "#4d41b1"),("Factor", "#636746"),("Fancy", "#7b9db4"),("Fantom", "#dbded5"),("Forth", "#341708"),("FreeMarker", "#0050b2"),("Frege", "#00cafe"),("Game Maker Language", "#8fb200"),("Glyph", "#e4cc98"),("Gnuplot", "#f0a9f0"),("Go", "#375eab"),("Golo", "#88562A"),("Gosu", "#82937f"),("Grammatical Framework", "#79aa7a"),("Groovy", "#e69f56"),("HTML", "#e44b23"),("Handlebars", "#01a9d6"),("Harbour", "#0e60e3"),("Haskell", "#29b544"),("Haxe", "#df7900"),("Hy", "#7790B2"),("IDL", "#a3522f"),("Io", "#a9188d"),("Ioke", "#078193"),("Isabelle", "#FEFE00"),("J", "#9EEDFF"),("JFlex", "#DBCA00"),("JSONiq", "#40d47e"),("Java", "#b07219"),("JavaScript", "#f1e05a"),("Julia", "#a270ba"),("Jupyter Notebook", "#DA5B0B"),("KRL", "#28431f"),("Kotlin", "#F18E33"),("LFE", "#004200"),("LOLCODE", "#cc9900"),("LSL", "#3d9970"),("Lasso", "#999999"),("Latte", "#A8FF97"),("Lex", "#DBCA00"),("LiveScript", "#499886"),("LookML", "#652B81"),("Lua", "#000080"),("MAXScript", "#00a6a6"),("MTML", "#b7e1f4"),("Makefile", "#427819"),("Mask", "#f97732"),("Matlab", "#bb92ac"),("Max", "#c4a79c"),("Mercury", "#ff2b2b"),("Metal", "#8f14e9"),("Mirah", "#c7a938"),("NCL", "#28431f"),("Nemerle", "#3d3c6e"),("NetLinx", "#0aa0ff"),("NetLinx+ERB", "#747faa"),("NetLogo", "#ff6375"),("NewLisp", "#87AED7"),("Nimrod", "#37775b"),("Nit", "#009917"),("Nix", "#7e7eff"),("Nu", "#c9df40"),("OCaml", "#3be133"),("Objective-C", "#438eff"),("Objective-C++", "#6866fb"),("Objective-J", "#ff0c5a"),("Omgrofl", "#cabbff"),("Opal", "#f7ede0"),("Oxygene", "#cdd0e3"),("Oz", "#fab738"),("PAWN", "#dbb284"),("PHP", "#4F5D95"),("PLSQL", "#dad8d8"),("Pan", "#cc0000"),("Papyrus", "#6600cc"),("Parrot", "#f3ca0a"),("Pascal", "#b0ce4e"),("Perl", "#0298c3"),("Perl6", "#0000fb"),("PigLatin", "#fcd7de"),("Pike", "#005390"),("PogoScript", "#d80074"),("Processing", "#0096D8"),("Prolog", "#74283c"),("Propeller Spin", "#7fa2a7"),("Puppet", "#302B6D"),("Pure Data", "#91de79"),("PureBasic", "#5a6986"),("PureScript", "#1D222D"),("Python", "#3572A5"),("QML", "#44a51c"),("R", "#198ce7"),("RAML", "#77d9fb"),("Racket", "#22228f"),("Ragel in Ruby Host", "#9d5200"),("Rebol", "#358a5b"),("Red", "#ee0000"),("Ren'Py", "#ff7f7f"),("Rouge", "#cc0088"),("Ruby", "#701516"),("Rust", "#dea584"),("SAS", "#B34936"),("SQF", "#3F3F3F"),("SaltStack", "#646464"),("Scala", "#DC322F"),("Scheme", "#1e4aec"),("Self", "#0579aa"),("Shell", "#89e051"),("Shen", "#120F14"),("Slash", "#007eff"),("Slim", "#ff8f77"),("Smalltalk", "#596706"),("SourcePawn", "#5c7611"),("Squirrel", "#800000"),("Stan", "#b2011d"),("Standard ML", "#dc566d"),("SuperCollider", "#46390b"),("Swift", "#ffac45"),("SystemVerilog", "#DAE1C2"),("Tcl", "#e4cc98"),("TeX", "#3D6117"),("Turing", "#45f715"),("TypeScript", "#2b7489"),("Unified Parallel C", "#4e3617"),("Unity3D Asset", "#ab69a1"),("UnrealScript", "#a54c4d"),("VHDL", "#adb2cb"),("Vala", "#fbe5cd"),("Verilog", "#b2b7f8"),("VimL", "#199f4b"),("Visual Basic", "#945db7"),("Volt", "#1F1F1F"),("Vue", "#2c3e50"),("Web Ontology Language", "#9cc9dd"),("X10", "#4B6BEF"),("XC", "#99DA07"),("XQuery", "#5232e7"),("Zephir", "#118f9e"),("cpp", "#f34b7d"),("eC", "#913960"),("edn", "#db5855"),("nesC", "#94B0C7"),("ooc", "#b0b77e"),("wisp", "#7582D1"),("xBase", "#403a40")]);
    match colors.get(key) {
        Some(color) => color,
        _ => "#EDEDED"
    }
}

pub fn draw_pie_diagram(document: &Document, values: &HashMap<String, f64>) -> Result<(), String> {
    let (canvas, context) = match get_canvas_and_context(&document, "pie-diagram") {
        Ok(result) => result,
        Err(err) => {
            return Err(err.to_string());
        }
    };
    let canvas_width = canvas.width();
    let canvas_height = canvas.height();

    context.clear_rect(0.0, 0.0, f64::from(canvas_width), f64::from(canvas_height));

    let x = canvas_width / 2;
    let y = canvas_height / 2;
    let radius = f64::from(cmp::min(x, y));
    let x = f64::from(x);
    let y = f64::from(y);
    let mut current_angle = 0.0;
    let end_angle = f64::from(2.0 * PI);

    for (key, value) in values.iter() {
        let angle = value.clone() / 100.0 * end_angle;
        context.begin_path();
        context.move_to(x, y);
        if let Err(_) = context.arc(x, y, radius, current_angle, current_angle + angle) {
            return Err("cannot create circular arc".to_string());
        }
        context.close_path();
        let color = get_color(key);
        context.set_fill_style(&JsValue::from_str(color));
        context.fill();

        current_angle = current_angle + angle;
    }

    Ok(())
}

pub fn draw_bar_diagram(document: &Document, values: &HashMap<String, f64>) -> Result<(), String> {
    let (canvas, context) = match get_canvas_and_context(&document, "bar-diagram") {
        Ok(result) => result,
        Err(err) => {
            return Err(err.to_string());
        }
    };


    let canvas_width = f64::from(canvas.width());
    let canvas_height = f64::from(canvas.height());

    context.clear_rect(0.0, 0.0, canvas_width, canvas_height);

    context.begin_path();
    context.move_to(0.0, 0.0);
    context.line_to(0.0, canvas_height);
    context.move_to(0.0, canvas_height);
    context.line_to(canvas_width, canvas_height);
    context.stroke();

    let max_value = match get_max(&values) {
        Ok(value) => value,
        Err(err) => {
            return Err(err)
        }
    };
    let spacing = 10.0;
    let scale = canvas_height / max_value;
    let bar_width = canvas_width / values.len() as f64;
    let mut index = 0.0;

    for (key, value) in values.iter() {
        let bar_height = scale * value;
        let x = index * bar_width + spacing;
        let y = canvas_height - (bar_height + 1.0);
        let color = get_color(key);

        context.set_fill_style(&JsValue::from_str(color));
        context.fill_rect(x, y, bar_width - (2.0 * spacing), bar_height);

        index += 1.0;
    }

    Ok(())
}

pub fn get_colors_map(values: HashMap<String, f64>) -> HashMap<String, String> {
    let mut colors_map:HashMap<String, String> = HashMap::new();
    
    for (key, _) in values.iter() {
        let color = get_color(key);

        colors_map.insert(key.to_string(), color.to_string());
    }

    colors_map
}