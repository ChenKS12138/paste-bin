use serde::{Deserialize, Serialize};

macro_rules! generate_lang {
    ($( $lang:ident, $name:expr => $label:expr ),+) => {
        #[derive(Debug,Deserialize,Serialize,Clone)]
        pub enum Lang {
            $(
                #[serde(rename = $name)]
                $lang,
            )+
        }
        lazy_static! {
            pub static ref LANG_LIST:Vec<(Lang,&'static str)> = vec![
                $((Lang::$lang, $label),)+
            ];
        }
    };
}

generate_lang! {
    PlainText,"Plain Text" => "Plain Text",
    ActionScript,"ActionScript" => "ActionScript",
    AppleScript,"AppleScript" => "AppleScript",
    Asp,"ASP" => "ASP",
    BatchFile,"Batch File" => "Batch File",
    BibTeX,"BibTeX" => "BibTeX",
    BourneAgainShellbash,"Bourne Again Shell (bash)" => "Bourne Again Shell (bash)",
    C,"C" => "C",
    CSharp,"C#" => "C#",
    Camlp4,"camlp4" => "camlp4",
    CargoBuildResults,"Cargo Build Results" => "Cargo Build Results",
    Clojure,"Clojure" => "Clojure",
    Commandsbuiltinshellbash,"commands-builtin-shell-bash" => "commands-builtin-shell-bash",
    Cpp,"C++" => "C++",
    Css,"CSS" => "CSS",
    D,"D" => "D",
    Diff,"Diff" => "Diff",
    Erlang,"Erlang" => "Erlang",
    Go,"Go" => "Go",
    GraphvizDot,"Graphviz (DOT)" => "Graphviz (DOT)",
    Groovy,"Groovy" => "Groovy",
    HTMLRails,"HTML (Rails)" => "HTML (Rails)",
    HTMLTcl,"HTML (Tcl)" => "HTML (Tcl)",
    Haskell,"Haskell" => "Haskell",
    Html,"HTML" => "HTML",
    HtmlAsp,"HTML (ASP)" => "HTML (ASP)",
    HtmlErlang,"HTML (Erlang)" => "HTML (Erlang)",
    Java,"Java" => "Java",
    JavaDoc,"JavaDoc" => "JavaDoc",
    JavaProperties,"Java Properties" => "Java Properties",
    JavaScript,"JavaScript" => "JavaScript",
    JavaScriptRails,"JavaScript (Rails)" => "JavaScript (Rails)",
    JavaServerPageJSP,"Java Server Page (JSP)" => "Java Server Page (JSP)",
    Json,"JSON" => "JSON",
    LaTeX,"LaTeX" => "LaTeX",
    LaTeXLog,"LaTeX Log" => "LaTeX Log",
    Lisp,"Lisp" => "Lisp",
    LiterateHaskell,"Literate Haskell" => "Literate Haskell",
    Lua,"Lua" => "Lua",
    MakeOutput,"Make Output" => "Make Output",
    Makefile,"Makefile" => "Makefile",
    Markdown,"Markdown" => "Markdown",
    Matlab,"MATLAB" => "MATLAB",
    MultiMarkdown,"MultiMarkdown" => "MultiMarkdown",
    NAntBuildFile,"NAnt Build File" => "NAnt Build File",
    OCaml,"OCaml" => "OCaml",
    OCamllex,"OCamllex" => "OCamllex",
    OCamlyacc,"OCamlyacc" => "OCamlyacc",
    ObjectiveC,"Objective-C" => "Objective-C",
    ObjectiveCpp,"Objective-C++" => "Objective-C++",
    PHP,"PHP" => "PHP",
    PHPSource,"PHP Source" => "PHP Source",
    Pascal,"Pascal" => "Pascal",
    Perl,"Perl" => "Perl",
    Python,"Python" => "Python",
    R,"R" => "R",
    RConsole,"R Console" => "R Console",
    RdRDocumentation,"Rd (R Documentation)" => "Rd (R Documentation)",
    ReStructuredText,"reStructuredText" => "reStructuredText",
    RegularExpression,"Regular Expression" => "Regular Expression",
    RegularExpressionsJavascript,"Regular Expressions (Javascript)" => "Regular Expressions (Javascript)",
    RegularExpressionsPython,"Regular Expressions (Python)" => "Regular Expressions (Python)",
    Ruby,"Ruby" => "Ruby",
    RubyHaml,"Ruby Haml" => "Ruby Haml",
    RubyonRails,"Ruby on Rails" => "Ruby on Rails",
    Rust,"Rust" => "Rust",
    SQL,"SQL" => "SQL",
    SQLRails,"SQL (Rails)" => "SQL (Rails)",
    Scala,"Scala" => "Scala",
    ShellUnixGeneric,"Shell-Unix-Generic" => "Shell-Unix-Generic",
    Tcl,"Tcl" => "Tcl",
    TeX,"TeX" => "TeX",
    Textile,"Textile" => "Textile",
    XML,"XML" => "XML",
    YAML,"YAML" => "YAML"
}

#[cfg(test)]
mod test {
    use serde_variant::to_variant_name;
    use syntect::parsing::SyntaxSet;
    #[test]
    fn find_syntax_by_lang_name() {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        for (lang, _) in super::LANG_LIST.iter() {
            match lang {
                super::Lang::PlainText => {
                    // to nothing
                }
                _ => {
                    println!("{}", to_variant_name(lang).unwrap());
                    syntax_set
                        .find_syntax_by_name(to_variant_name(lang).unwrap())
                        .unwrap();
                }
            }
        }
    }
}
