use clap::Parser;
use phi_tk_mini_lib::{CliArgs, RenderParams};

#[test]
fn obsolete_aspect_settings_are_ignored_and_not_saved() {
    let params: RenderParams = serde_json::from_value(serde_json::json!({
        "path": "legacy.pez",
        "info": {"name": "Legacy chart", "aspectRatio": 16.0 / 9.0},
        "config": {"resolution": [640, 180], "aspectRatio": 4.0 / 3.0, "fixAspectRatio": true}
    }))
    .unwrap();
    assert_eq!(params.config.output_resolution().unwrap(), (640, 180));
    let saved = serde_json::to_value(&params).unwrap();
    assert!(saved["info"].get("aspectRatio").is_none());
    assert!(saved["config"].get("aspectRatio").is_none());
    assert!(saved["config"].get("fixAspectRatio").is_none());
    let engine = serde_json::to_value(params.config.to_config()).unwrap();
    assert!(engine.get("aspectRatio").is_none());
    assert!(engine.get("fixAspectRatio").is_none());
}

#[test]
fn cli_version_matches_the_application_release() {
    let error = CliArgs::try_parse_from(["phi-tk-cli", "--version"]).unwrap_err();
    assert_eq!(error.kind(), clap::error::ErrorKind::DisplayVersion);
    assert_eq!(error.to_string().trim(), "phi-tk-cli 1.2.0");
}
