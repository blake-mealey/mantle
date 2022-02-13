use yansi::Paint;

use rbx_mantle::{
    config::{load_project_config, StateConfig},
    logger,
    state::{get_state_from_source, save_state},
};

pub async fn run(project: Option<&str>, file: Option<&str>) -> i32 {
    logger::start_action("Upload state file:");
    let (project_path, config) = match load_project_config(project) {
        Ok(v) => v,
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };

    if matches!(config.state, StateConfig::Local) {
        logger::end_action(Paint::red("Project is not configured with remote state"));
        return 1;
    }

    let state_config = match file {
        Some(file) => StateConfig::LocalCustom(file.to_owned()),
        None => StateConfig::Local,
    };
    let state = match get_state_from_source(&project_path, state_config).await {
        Ok(v) => v,
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };

    match save_state(&project_path, &config.state, &state).await {
        Ok(_) => {}
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };
    logger::end_action("Succeeded");

    0
}
