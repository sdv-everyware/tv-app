//! tv-app: fonts, for the renderer and for the layout helper that measures text (main.rs).

use har_platform_api::error::Error;
use har_platform_api::graphics::{FontInfo, Renderer};
use har_platform_api::looper::RendererHelper;

const INTER_REGULAR: &[u8] = include_bytes!("../data/inter_regular.otf");
const INTER_BOLD: &[u8] = include_bytes!("../data/inter_bold.otf");

pub const FONTS: &[(&str, u16, &[u8])] =
    &[("Inter", 400, INTER_REGULAR), ("Inter", 500, INTER_REGULAR), ("Inter", 600, INTER_BOLD)];

#[derive(Default)]
pub struct ResourceManager;

impl RendererHelper for ResourceManager {
    fn initialize_renderer(&self, renderer: &mut dyn Renderer) -> Result<(), Error> {
        // pictures are registered by key (everyware_ui::images)
        renderer.replace_raw_image_storage(everyware_ui::images::storage());
        for (family, weight, data) in FONTS {
            // SAFETY: OpenType fonts are supported.
            unsafe { renderer.load_font(vec![FontInfo::new(family, *weight)], data)? };
        }
        Ok(())
    }
}
