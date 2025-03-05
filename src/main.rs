use std::{rc::Rc, sync::Arc};

use oxc::{
    codegen::{Codegen, CodegenOptions},
    mangler::{MangleOptions, Mangler},
    minifier::{CompressOptions, Compressor, MinifierOptions},
    semantic::SemanticBuilder,
    span::SourceType,
};

fn main() {
    let input_1 = include_str!("../fixtures/input1.js").to_string();
    //let input_2: String = include_str!("../fixtures/input2.js").to_string();
    oxc_minify(&input_1);
    swc_minify(input_1);
    
}
fn oxc_minify(code: &str)  {
    use oxc::allocator::Allocator;
    use oxc::parser::Parser;
    let allocator = Allocator::new();
    let program = &mut Parser::new(&allocator, &code, SourceType::default())
        .parse()
        .program;
    let semantic = SemanticBuilder::new().build(program).semantic;
    let stats = semantic.stats();
    let (symbols, scopes) = semantic.into_symbol_table_and_scope_tree();
    Compressor::new(&allocator, CompressOptions::default())
        .build_with_symbols_and_scopes(symbols, scopes, program);

    let symbol_table = {
        let semantic = SemanticBuilder::new()
            .with_stats(stats)
            .with_scope_tree_child_ids(true)
            .build(&program)
            .semantic;
        Mangler::default()
            .with_options(MangleOptions::default())
            .build_with_semantic(semantic, program)
    };
    let ret = Codegen::new()
        .with_options(CodegenOptions {
            ..Default::default()
        })
        .with_symbol_table(Some(symbol_table))
        .build(&program);
    std::fs::write("output/oxc.js", ret.code).unwrap();
}
fn swc_minify(code: String) {
    use swc_core::common::Mark;
    use swc_core::common::comments::SingleThreadedComments;
    use swc_core::common::input::{self, SourceFileInput};
    use swc_core::common::{FileName, GLOBALS, Globals, SourceMap, sync::Lrc};
    use swc_core::ecma::codegen;
    use swc_core::ecma::minifier::optimize;
    use swc_core::ecma::minifier::option::{ExtraOptions, MinifyOptions};
    use swc_core::ecma::parser::lexer::Lexer;
    use swc_core::ecma::parser::{Parser, Syntax};
    GLOBALS.set(&Default::default(), || {
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(Lrc::new(FileName::Anon), code);
        let lexer = Lexer::new(
            Default::default(),
            Default::default(),
            SourceFileInput::from(&*fm),
            None,
        );
        let mut parser = Parser::new_from(lexer);

        let comments = SingleThreadedComments::default();
        let unresolved_mark = Mark::new();
        let top_level_mark = Mark::new();
        let program = parser.parse_program().unwrap();
        let program = optimize(
            program,
            cm.clone(),
            Some(&comments),
            None,
            &MinifyOptions {
                ..Default::default()
            },
            &ExtraOptions {
                unresolved_mark,
                top_level_mark,
                mangle_name_cache: None,
            },
        );

        // Generate the minified code
        let mut buf = vec![];
        {
            let wr = Box::new(codegen::text_writer::JsWriter::new(
                cm.clone(),
                "\n",
                &mut buf,
                None,
            ));
            let mut emitter = codegen::Emitter {
                cfg: codegen::Config::default(),
                comments: Some(&comments),
                cm: cm.clone(),
                wr,
            };
            emitter.emit_program(&program).unwrap();
        }
        let code =  unsafe { String::from_utf8_unchecked(buf) };
        std::fs::write("output/swc.js", code).unwrap();
    })
    
}
