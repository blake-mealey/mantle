use std::path::Path;

use yansi::Paint;

use rbx_mantle::{
    config::{load_project_config, StateConfig},
    logger,
    state::{get_state, save_state_locally},
};

pub async fn run(project: Option<&str>, output: Option<&str>) -> i32 {
    logger::start_action("Download state file:");
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

    let state = match get_state(&project_path, &config).await {
        Ok(v) => v,
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };

    match save_state_locally(
        &project_path,
        &state,
        output.map(|o| Path::new(o).to_owned()),
    )
    .await
    {
        Ok(_) => {}
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };
    logger::end_action("Succeeded");

    0
}
