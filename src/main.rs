use swc_core::common::{FileName, Globals, GLOBALS, Mark, SourceMap};
use swc_core::common::input::StringInput;
use swc_core::common::sync::Lrc;
use swc_core::ecma::codegen::Emitter;
use swc_core::ecma::codegen::text_writer::JsWriter;
use swc_core::ecma::parser::{EsConfig, Parser, Syntax};
use swc_core::ecma::transforms::optimization::simplify::expr_simplifier;
use swc_core::ecma::visit::VisitMutWith;
use swc_core::common::pass::Repeated;

fn main() {
    // Input JavaScript
    let input = "3 * (2)";
    
    let globals = Globals::new();
    GLOBALS.set(&globals, || {
        // Setup SWC
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(FileName::Anon, input.to_string());

        // Parse JavaScript code
        let mut parser = Parser::new(
            Syntax::Es(EsConfig::default()),
            StringInput::from(&*fm),
            None,
        );
        let mut program = parser.parse_program().expect("parse_program failed");

        // Apply expr_simplifier
        let mut simplifier = expr_simplifier(Mark::new(), Default::default());
        loop {
            program.visit_mut_with(&mut simplifier);
            
            if !simplifier.changed() {
                break;
            }
            simplifier.reset();
        }

        // Generate new code from the modified AST
        let mut buf = Vec::new();
        let writer = Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None));
        let mut emitter = Emitter {
            cfg: Default::default(),
            cm: cm.clone(),
            comments: None,
            wr: writer,
        };
        emitter.emit_program(&program).expect("emit_script failed");
        let output_code = String::from_utf8(buf).expect("String::from_utf8 failed");
        
        println!("{}", output_code);
    });
}
