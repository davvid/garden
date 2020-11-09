use dirs;
use xdg;

use super::errors;
use super::model;

/// YAML reader
pub mod reader;

/// YAML writer
pub mod writer;


// Search for configuration in the following locations:
//  .
//  ./garden
//  ./etc/garden
//  ~/.config/garden
//  ~/etc/garden
//  /etc/garden

fn search_path() -> Vec<std::path::PathBuf> {
    // Result: Vec<PathBufs> in priority order
    let mut paths: Vec<std::path::PathBuf> = Vec::new();

    let current_dir = std::env::current_dir().unwrap_or(std::path::PathBuf::from("."));
    let home_dir = dirs::home_dir().unwrap_or(std::path::PathBuf::from("/"));

    // . Current directory
    paths.push(current_dir.to_path_buf());

    // ./garden
    let mut current_garden_dir = current_dir.to_path_buf();
    current_garden_dir.push("garden");
    if current_garden_dir.exists() {
        paths.push(current_garden_dir);
    }

    // ./etc/garden
    let mut current_etc_garden_dir = current_dir.to_path_buf();
    current_etc_garden_dir.push("etc");
    current_etc_garden_dir.push("garden");
    if current_etc_garden_dir.exists() {
        paths.push(current_etc_garden_dir);
    }

    // $XDG_CONFIG_HOME/garden (typically ~/.config/garden)
    paths.push(xdg_dir());

    // ~/etc/garden
    let mut home_etc_dir = home_dir.to_path_buf();
    home_etc_dir.push("etc");
    home_etc_dir.push("garden");
    if home_etc_dir.exists() {
        paths.push(home_etc_dir);
    }

    // /etc/garden
    let etc_garden = std::path::PathBuf::from("/etc/garden");
    if etc_garden.exists() {
        paths.push(etc_garden);
    }

    paths
}


/// $XDG_CONFIG_HOME/garden (typically ~/.config/garden)
pub fn xdg_dir() -> std::path::PathBuf {
    let mut home_config_dir;

    if let Ok(xdg_dirs) = xdg::BaseDirectories::new() {
        home_config_dir = xdg_dirs.get_config_home();
    } else {
        home_config_dir = dirs::home_dir().unwrap_or(std::path::PathBuf::from("/"));
        home_config_dir.push(".config")
    }
    home_config_dir.push("garden");

    home_config_dir
}


pub fn new(
    config: &Option<std::path::PathBuf>,
    root: &str,
    verbose: bool,
) -> Result<model::Configuration, errors::GardenError> {

    let mut cfg = model::Configuration::new();
    cfg.verbose = verbose;

    // Override the configured garden root
    if !root.is_empty() {
        cfg.root.expr = root.into();
    }

    let mut basename: String = "garden.yaml".into();

    // Find garden.yaml in the search path
    let mut found = false;
    if let Some(config_path) = config {
        if config_path.is_file() || config_path.is_absolute() {
            // If an absolute path was specified, or if the file exists,
            // short-circuit the search; the config file might be missing but
            // we shouldn't silently use a different config file.
            cfg.set_path(config_path.to_path_buf());
            found = true;
        } else {
            // The specified path is a basename or relative path to be found
            // in the config search path.
            basename = config_path.to_string_lossy().into();
        }
    }

    if !found {
        for entry in search_path() {
            let mut candidate = entry.to_path_buf();
            candidate.push(basename.clone());
            if candidate.exists() {
                cfg.set_path(candidate);
                found = true;
                break;
            }
        }
    }
    if verbose {
        debug!(
            "config path is {:?}{} root is {:?}",
            cfg.path,
            cfg.root,
            match found {
                true => "",
                false => " (NOT FOUND)",
            }
        );
    }

    if found {
        // Read file contents.
        let config_path = cfg.get_path()?;
        let config_string = match std::fs::read_to_string(config_path) {
            Ok(content) => content,
            // Return a default Configuration If we are unable to read the file.
            Err(_) => return Ok(cfg),
        };

        parse(&config_string, verbose, &mut cfg)?;
    }

    // Default to the current directory when garden.root is unspecified
    if cfg.root.expr.is_empty() {
        cfg.root.expr = std::env::current_dir().unwrap_or(
            std::path::PathBuf::from(".")
        ).to_string_lossy().to_string();
    }

    // Grafts
    let mut exprs = Vec::new();
    for graft in &cfg.grafts {
        exprs.push(graft.config_expr.clone());
    }

    let mut paths = Vec::new();
    for expr in &exprs {
        let path_str = cfg.eval_tree_path(expr);
        paths.push(path_str);
    }

    let mut idx = 0;
    for graft in cfg.grafts.iter_mut() {
        let path_str = paths[idx].clone();
        let path = std::path::PathBuf::from(&path_str);
        if path.exists() {
            graft.config = Some(new(&Some(path), &graft.root, verbose)?);
        }
        idx += 1;
    }

    Ok(cfg)
}


/// Create a model::Configuration instance from model::CommandOptions

pub fn from_options(
    options: &model::CommandOptions,
) -> Result<model::Configuration, errors::GardenError> {
    let config_verbose = options.is_debug("config::new");
    let mut config = new(&options.filename, &options.root, config_verbose)?;

    if config.path.is_none() {
        error!("unable to find a configuration file -- use --config <path>");
    }
    if options.is_debug("config") {
        eprintln!("config: {:?}", config.get_path()?);
        debug!("{}", config);
    }

    for k_eq_v in &options.variables {
        let name: String;
        let expr: String;
        let values: Vec<&str> = k_eq_v.splitn(2, "=").collect();
        if values.len() == 1 {
            name = values[0].into();
            expr = "".into();
        } else if values.len() == 2 {
            name = values[0].into();
            expr = values[1].into();
        } else {
            error!("unable to split '{}'", k_eq_v);
        }
        config.variables.insert(
            0,
            model::NamedVariable {
                name: name,
                expr: expr,
                value: None,
            },
        );
    }

    Ok(config)
}

/// Parse and apply configuration from a YAML/JSON string
pub fn parse(
    config_string: &str,
    verbose: bool,
    cfg: &mut model::Configuration,
) -> Result<(), errors::GardenError> {

    reader::parse(&config_string, verbose, cfg)?;
    // Initialize the configuration now that the values have been read.
    cfg.initialize();

    Ok(())
}