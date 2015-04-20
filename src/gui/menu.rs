use std::num::ToPrimitive;
use std::cmp;

extern crate conrod;
extern crate opengl_graphics;

use self::conrod::{
    Color,
    Colorable,
    Drawable,
    DropDownList,
    Frameable,
    Labelable,
    Positionable,
    Shapeable,
    Ui
};
use self::conrod::color::rgb;
use self::opengl_graphics::GlGraphics;
use self::opengl_graphics::glyph_cache::GlyphCache;

static MENU_R: f32 = 0.40;
static MENU_G: f32 = 0.44;
static MENU_B: f32 = 0.48;

static LABEL_R: f32 = 0.85;
static LABEL_G: f32 = 0.89;
static LABEL_B: f32 = 0.93;

pub struct Menu {
    entries: Vec<String>,
    width: f64,
    height: f64,
    ofsx: f64,
    color: Color,
    file_selected_idx: Option<usize>
}

impl<'a> Menu {
    pub fn new(entries: Vec<String>, size: f64, ofsx: f64) -> Menu {
        let longest_entry = entries.iter().fold(0, |x, word| cmp::max(x, word.len()))
            .to_f64().unwrap() * size;

        Menu {
            entries: entries,
            width: longest_entry,
            height: size * 1.8,
            ofsx: ofsx,
            color: rgb(MENU_R, MENU_G, MENU_B),
            file_selected_idx: None
        }
    }

    pub fn entries_mut(&'a mut self) -> &'a mut Vec<String> {
        &mut self.entries
    }

    pub fn color(&self) -> Color {
        self.color.clone()
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn ofsx(&self) -> f64 {
        self.ofsx
    }

    pub fn sel_idx_mut(&'a mut self) -> &'a mut Option<usize> {
        &mut self.file_selected_idx
    }

    pub fn draw(&mut self, ui: &mut Ui<GlyphCache<'a>>, gl: &mut GlGraphics) {
        let width = self.width;
        let height = self.height;
        let mut entries = self.entries_mut().clone();
        let color = self.color();
        let ofsx = self.ofsx;
        let label = self.entries.get(0).unwrap().clone();
        DropDownList::new(0, &mut entries, &mut self.file_selected_idx)
            .dimensions(width, height)
            .position(ofsx, 0.0)
            .color(color)
            .frame(1.0)
            .label(&label)
            .label_color(rgb(LABEL_R, LABEL_G, LABEL_B))
            .callback(|selected_idx: &mut Option<usize>, new_idx, _string| {
                *selected_idx = Some(new_idx);
            })
            .draw(ui, gl);
    }

    pub fn idx(&self) -> Option<usize> {
        self.file_selected_idx
    }

    pub fn set_idx(&mut self, novo: Option<usize>) {
        self.file_selected_idx = novo
    }
}

pub struct MenuBar {
    menus: Vec<Menu>,
    font_size: f64,
    width: f64,
    height: f64
}

impl<'a> MenuBar {
    pub fn new(font_size: f64, entries: Vec<Vec<String>>, width: f64) -> MenuBar {
        let mut ofsx = 0.0;
//        let file_menu = Menu::new(file_entries, font_size, ofsx);

        let menus = entries.into_iter().map(|entry| {
            let menu = Menu::new(entry, font_size, ofsx);
            ofsx += menu.width() + 1.0;
            menu
        }).collect::<Vec<Menu>>();

        MenuBar {
            menus: menus,
            font_size: font_size,
            width: width,
            height: font_size * 1.8,
        }
    }

    pub fn font_size(&self) -> f64 {
        self.font_size
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }
    
    pub fn menu(&'a self, idx: usize) -> Option<&'a Menu> {
        self.menus.get(idx)
    }

    pub fn menu_mut(&'a mut self, idx: usize) -> Option<&'a mut Menu> {
        self.menus.get_mut(idx)
    }

    pub fn label_color(&self) -> Color {
        rgb(LABEL_R, LABEL_G, LABEL_B)
    }
/*
    pub fn draw_menus(&mut self, gl: &mut GlGraphics, ui: &mut Ui<GlyphCache<'a>>) {
        for menu in self.menus.iter_mut() {
            menu.draw(gl, ui);
        }
    }
*/
}
