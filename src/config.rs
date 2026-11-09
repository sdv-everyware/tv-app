//! tv-app: the display and the command line.

use clap::Parser as _;
use clap_derive::Parser;
use har_platform_api::looper::DisplayId;
use har_types::color_format::ColorFormat;
use std::collections::HashMap;

/// The screen's root frame in the design (tv-patch.js), 1920 x 1080: the RB3's HDMI mode.
pub const ROOT_NODE: &str = "#tv-root";

#[derive(Parser, Debug)]
pub struct CommandLineArgs {
    /// Directory with the document (.dcf).
    #[arg(short, long, default_value = "/data/local/tmp/tv")]
    pub data_path: String,
    /// The design's file in data_path (`<name>.dcf`; a name without the extension is completed).
    #[arg(short = 'i', long, default_value = "tv.dcf")]
    pub document_id: String,
    /// Display size "WxH" to map touch positions to (RB3 HDMI touch reports 0..32767).
    #[arg(long)]
    pub touch_size: Option<String>,
}

#[derive(Debug)]
pub struct Display {
    pub id: DisplayId,
    pub width: u32,
    pub height: u32,
    pub dpi: f32,
    pub root_node_id: String,
    pub display_params: HashMap<String, String>,
    pub input_devices: Option<Vec<String>>,
}

pub fn displays() -> Vec<Display> {
    let mut display_params = HashMap::new();
    display_params.insert("interface_id".to_string(), 1.to_string());
    display_params.insert("use_exact_size".to_string(), false.to_string());
    display_params.insert("use_preferred_mode".to_string(), false.to_string());
    display_params.insert("color_format".to_string(), ColorFormat::ABGR8888.to_string());
    vec![Display {
        id: DisplayId::new(1),
        width: 1920,
        height: 1080,
        dpi: 160f32,
        root_node_id: ROOT_NODE.to_string(),
        display_params,
        input_devices: None,
    }]
}

pub fn parse_size(s: &str) -> Option<(u32, u32)> {
    let (w, h) = s.split_once('x')?;
    Some((w.trim().parse().ok()?, h.trim().parse().ok()?))
}

pub fn args() -> CommandLineArgs {
    let mut args = CommandLineArgs::parse();
    // the file name harry opens: <name>.dcf, also when the name came without the extension
    args.document_id = everyware_ui::design::document_file(&args.document_id);
    args
}
