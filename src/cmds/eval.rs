extern crate glob;

use ::eval;
use ::config;
use ::model;
use ::query;


pub fn main(options: &mut model::CommandOptions) {
    options.args.insert(0, "garden eval".to_string());

    let mut expr = String::new();
    let mut tree = String::new();
    let mut garden = String::new();
    let mut garden_opt: Option<String> = None;

    // Parse arguments
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("garden eval - evaluate expressions");

        ap.refer(&mut expr).required()
            .add_argument("garden-expr", argparse::Store,
                          "garden variable expression to evaluate");

        ap.refer(&mut tree)
            .add_argument("tree", argparse::Store, "tree to evaluate");

        ap.refer(&mut garden)
            .add_argument("garden", argparse::Store, "garden to evaluate");

        if let Err(err) = ap.parse(options.args.to_vec(),
                                   &mut std::io::stdout(),
                                   &mut std::io::stderr()) {
            std::process::exit(err);
        }
    }

    let verbose = options.is_debug("config::new");
    let mut cfg = config::new(&options.filename, verbose);
    if options.is_debug("config") {
        debug!("{}", cfg);
    }

    if tree.is_empty() {
        println!("{}", eval::value(&mut cfg, &expr));
        return;
    }

    if !garden.is_empty() {
        garden_opt = Some(garden);
    }

    // Evaluate and print the garden expression.
    match query::tree_context(&cfg, &tree, garden_opt) {
        Ok(ctx) => {
            let value = eval::tree_value(&mut cfg, &expr, ctx.tree, ctx.garden);
            println!("{}", value);
        }
        Err(err) => {
            error!("{}", err);
        }
    }
}