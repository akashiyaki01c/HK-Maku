use std::sync::Arc;
use wasm_bindgen::prelude::*;
use resvg::usvg::{fontdb, ImageRendering, ShapeRendering, TextRendering, TreeParsing, TreeTextToPath};

static FONT_KOSUGI_MARU: &[u8] = include_bytes!("./fonts/KosugiMaru-Regular.ttf");
static FONT_OPEN_SANS: &[u8] = include_bytes!("./fonts/OpenSans-Medium.ttf");

#[wasm_bindgen]
pub fn svg_to_png(svg: &str) -> Vec<u8> {
    let rtree = {
        let mut opt = resvg::usvg::Options::default();
        opt.image_rendering = ImageRendering::OptimizeSpeed;
        opt.text_rendering = TextRendering::OptimizeSpeed;
        opt.shape_rendering = ShapeRendering::OptimizeSpeed;

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

#[test]
fn test() {
	svg_to_png(r###"
	<svg xmlns="http://www.w3.org/2000/svg" version="1.0" width="5600" height="1600" viewBox="0 0 5600 1600">
	<rect 
        x="0" y="0" 
        width="5600" height="1600" 
        fill="#222"></rect>
	<text 
			font-family="Kosugi Maru" 
			text-anchor="middle" font-size="1060" 
			x="2800" y="1050" 
			fill="#fff" 
			letter-spacing="100"
			transform="translate(-140.0000000000001 0) scale(1.05 1.0)"
			stroke-width="30" stroke="#fff">大阪梅田</text>
	<text font-family="Open Sans" text-anchor="middle" x="2800" y="1420" font-size="300" fill="#fff">Osaka-umeda</text>
	</svg>
	"###);
}