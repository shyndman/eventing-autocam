use std::fmt::Display;
use std::time::Instant;

use aa_foundation::path::to_canonicalized_path_string;
use anyhow::{anyhow, Result};
use chrono::Local;
use gst::prelude::*;

use super::{names, CONFIGURE_CAT};
use crate::config::Config;
use crate::logging::*;
use crate::system::HardwareSystems;

pub fn configure_pipeline(
    config: &Config,
    hardware: HardwareSystems,
    (main_loop, pipeline): (glib::MainLoop, gst::Pipeline),
) -> Result<(glib::MainLoop, gst::Pipeline, HardwareSystems)> {
    info!(CONFIGURE_CAT, "Configuring pipeline");
    let now = Instant::now();

    if let Err(e) = configure_video_storage(config, &pipeline) {
        warning!(
            CONFIGURE_CAT,
            "Problem encountered while configuring video storage, {}",
            e
        );
    }
    if let Err(err) = configure_detection(config, &pipeline) {
        warning!(
            CONFIGURE_CAT,
            "Problem encountered while configuring inference, {}",
            err
        );
    }

    info!(
        CONFIGURE_CAT,
        "Finished configuring in {}ns",
        now.elapsed().as_nanos()
    );

    Ok((main_loop, pipeline, hardware))
}

fn configure_video_storage(
    config: &Config,
    pipeline: &gst::Pipeline,
) -> Result<(), anyhow::Error> {
    let storage_config = &config.video_storage;
    let persistence_sink = pipeline
        .by_name(names::PERSISTENCE_SINK)
        .ok_or(anyhow!("Persistence sink not found"))?;

    set_object_property(
        &persistence_sink,
        "max-size-time",
        storage_config.video_chunk_duration_nanos(),
    );
    set_object_property(
        &persistence_sink,
        "location",
        storage_config.video_path_pattern_for_datetime(Local::now())?,
    );

    Ok(())
}

fn configure_detection(
    config: &Config,
    pipeline: &gst::Pipeline,
) -> Result<(), anyhow::Error> {
    let detection_config = &config.detection;
    let detection_sink = pipeline
        .by_name(names::DETECTION_SINK)
        .ok_or(anyhow!("Detection sink not found"))?;

    if config.detection.is_ml() {
        set_object_property(
            &detection_sink,
            "model-location",
            to_canonicalized_path_string(&detection_config.model_path.relative())?.as_str(),
        );
        set_object_property(
            &detection_sink,
            "score-threshold",
            detection_config.score_threshold,
        );
    } else {
        // Color detection
        set_object_property(
            &detection_sink,
            "detection-pixel-threshold",
            detection_config.color_detection_pixel_threshold,
        );
    }

    set_object_property(&detection_sink, "max-results", detection_config.max_results);
    Ok(())
}

fn set_object_property<V: ToValue + Display>(
    element: &gst::Element,
    prop_name: &str,
    value: V,
) {
    debug!(
        CONFIGURE_CAT,
        obj: element,
        "Setting {} to {}",
        prop_name,
        value
    );
    element.set_property(prop_name, value);
}
