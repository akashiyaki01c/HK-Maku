use std::sync::Arc;
use wasm_bindgen::prelude::*;
use resvg::usvg::{fontdb, ImageRendering, ShapeRendering, TextRendering, TreeParsing, TreeTextToPath};

static FONT_KOSUGI_MARU: &[u8] = include_bytes!("./fonts/KosugiMaru-Regular.ttf");
static FONT_OPEN_SANS: &[u8] = include_bytes!("./fonts/OpenSans-Medium.ttf");

#[wasm_bindgen]
pub fn svg_to_png(svg: &str) -> Vec<u8> {
    let rtree = {
        let mut opt = resvg::usvg::Options::default();
        opt.image_rendering = ImageRendering::OptimizeQuality;
        opt.text_rendering = TextRendering::GeometricPrecision;
        opt.shape_rendering = ShapeRendering::GeometricPrecision;

        let mut fontdb = fontdb::Database::new();
        fontdb.load_system_fonts();
        fontdb.load_font_source(fontdb::Source::Binary(Arc::new(FONT_KOSUGI_MARU)));
        fontdb.load_font_source(fontdb::Source::Binary(Arc::new(FONT_OPEN_SANS)));
        let mut tree = resvg::usvg::Tree::from_data(svg.as_bytes(), &opt).expect("svg error");
        tree.convert_text(&fontdb);
        resvg::Tree::from_usvg(&tree)
    };
    let pixmap_size = rtree.size.to_int_size();
    let mut pixmap = resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).expect("pixmap init error");
    rtree.render(resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());
    pixmap.encode_png().expect("encode error")
}