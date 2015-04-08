extern crate conrod;
extern crate piston;
extern crate opengl_graphics;
extern crate glutin_window;

use self::conrod::{
    Background, 
    Button, 
    Callable, 
    Color, 
    Colorable, 
    Drawable, 
    Frameable, 
    Label, 
    Labelable, 
    Positionable, 
    Shapeable, 
    TextBox, 
    Theme, 
    Ui,
    WidgetMatrix,
};

use self::opengl_graphics::{GlGraphics, OpenGL};
use self::opengl_graphics::glyph_cache::GlyphCache;
use self::glutin_window::GlutinWindow;
use self::piston::window::{WindowSettings, Size};
use self::piston::event::{Event, Events};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use std::rc::Rc;
use std::cell::RefCell;

use super::schedule::{Schedule, Source, Program, Instruction};
use super::tags::Tags;

pub struct XBTVEd {
    schedules: Vec<Schedule>,
    bg_color: Color,
    current_schedule: usize
}

impl<'a> XBTVEd {
    pub fn new() -> XBTVEd {
        let loc = Source::Pathname("foo".to_string());
        let tags = Tags::new();
        let instrs = Instruction::Play(0, 0);
        let progs = vec!(Program::new(loc, tags, vec!(instrs)));
        XBTVEd {
            schedules: vec!(Schedule::new("Example", progs)),
            bg_color: Color::new(0.2, 0.2, 0.2, 1.0),
            current_schedule: 0
        }
    }

    pub fn get_schedule(&'a self) -> &'a Schedule {
        self.schedules.get(self.current_schedule).unwrap()
    }

    pub fn get_schedule_at(&'a self, idx: usize) -> Option<&'a Schedule> {
        self.schedules.get(idx)
    }

    pub fn push_schedule(&mut self, other: Schedule) {
        self.schedules.push(other)
    }
        
    pub fn prev_schedule(&mut self) {
        if self.current_schedule > 0 {
            self.current_schedule -= 1
        }
    }

    pub fn next_schedule(&mut self) {
        if self.current_schedule + 1 < self.schedules.len() {
            self.current_schedule += 1;
        }
    }

    pub fn change_current_schedule(&mut self, idx: usize) -> Result<(), String> {
        if idx >= self.schedules.len() {
            Err("Out of bounds".to_string())
        } else {
            self.current_schedule = idx;
            Ok(())
        }
    }

    pub fn modify_current_schedule(&mut self) -> &mut Schedule {
        self.schedules.get_mut(self.current_schedule).unwrap()
    }

    pub fn remove_schedule(&mut self, idx: usize) -> Result<(), String> {
        if idx >= self.schedules.len() {
            Err("Out of bounds".to_string())
        } else if self.schedules.len() == 1 {
            self.schedules.remove(idx);

            let loc = Source::Pathname("foo".to_string());
            let tags = Tags::new();
            let instrs = Instruction::Play(0, 0);
            let progs = vec!(Program::new(loc, tags, vec!(instrs)));

            self.schedules.push(Schedule::new("Example", progs));
            Ok(())
        } else {
            if idx + 1 == self.schedules.len() {
                self.current_schedule -= 1;
            }
            self.schedules.remove(idx);
            Ok(())
        }
    }
}

pub fn make_window(title: &str, width: u32, height: u32) -> (GlutinWindow, GlGraphics, Ui<GlyphCache>) {
    let opengl = OpenGL::_3_2;
    let window = GlutinWindow::new(
        opengl,
        WindowSettings::new(
            title.to_string(), 
            Size { width: width, height: height }
            ).exit_on_esc(true).samples(4));
    let gl = GlGraphics::new(opengl);

    let font_path = Path::new("./assets/NotoSans-Regular.ttf");
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();
    let ui = Ui::<GlyphCache>::new(glyph_cache, theme);

    (window, gl, ui)
}

pub fn pop_up_msg(msg: &str) {
    let (tx, rx) = channel();
    let mesg = msg.to_string();
    tx.send(mesg).unwrap();;
    thread::spawn(move || {
        let mesg = rx.recv().unwrap();
        let (window, mut gl, mut ui) = make_window("Error", 300, 200);
        let window_ref = Rc::new(RefCell::new(window));

        let light_bg = Color::new(0.8, 0.8, 0.8, 1.0);
        let mut clicked = false;

        for event in Events::new(window_ref).ups(180).max_fps(60) {
            ui.handle_event(&event);
            if let Event::Render(args) = event {
                gl.draw([0, 0, args.width as i32, args.height as i32], |_, gl| {
                    Background::new().color(light_bg.clone()).draw(&mut ui, gl);

                    Label::new(&mesg)
                        .position(5.0, 10.0)
                        .size(16)
                        .color(light_bg.plain_contrast())
                        .draw(&mut ui, gl);
                
                    Button::new(0)
                        .dimensions(100.0, 60.0)
                        .position(5.0, 100.0)
                        .rgba(0.25, 0.25, 0.25, 1.0)
                        .frame(1.0)
                        .label("Ok")
                        .callback(|| {
                            clicked = true;
                        }).draw(&mut ui, gl);

                });
            }

            if clicked {
                break
            }
        }
    });
}

pub fn confirm(msg: &str) -> bool {
    let (msg_tx, msg_rx) = channel();
    let (res_tx, res_rx) = channel();

    let mesg = msg.to_string();
    msg_tx.send(mesg).unwrap();;

    thread::spawn(move || {
        let mesg = msg_rx.recv().unwrap();
        let (window, mut gl, mut ui) = make_window("Confirm", 500, 200);
        let window_ref = Rc::new(RefCell::new(window));

        let light_bg = Color::new(0.8, 0.8, 0.8, 1.0);
        let mut clicked_cancel = false;
        let mut clicked_ok = false;

        for event in Events::new(window_ref).ups(180).max_fps(60) {
            ui.handle_event(&event);
            if let Event::Render(args) = event {
                gl.draw([0, 0, args.width as i32, args.height as i32], |_, gl| {
                    Background::new().color(light_bg.clone()).draw(&mut ui, gl);

                    Label::new(&mesg)
                        .position(5.0, 10.0)
                        .size(16)
                        .color(light_bg.plain_contrast())
                        .draw(&mut ui, gl);
                
                    Button::new(0)
                        .dimensions(100.0, 60.0)
                        .position(5.0, 100.0)
                        .rgba(0.25, 0.25, 0.25, 1.0)
                        .frame(1.0)
                        .label("Confirm")
                        .callback(|| {
                            clicked_ok = true;
                        }).draw(&mut ui, gl);

                    Button::new(1)
                        .dimensions(100.0, 60.0)
                        .position(110.0, 100.0)
                        .rgba(0.25, 0.25, 0.25, 1.0)
                        .frame(1.0)
                        .label("Cancel")
                        .callback(|| {
                            clicked_cancel = true;
                        }).draw(&mut ui, gl);

                });
            }

            if clicked_cancel || clicked_ok {
                break
            }
        }
        if clicked_ok {
            res_tx.send(true).unwrap();
        } else {
            res_tx.send(false).unwrap();
        }
    });
    if let Ok(true) = res_rx.recv() {
        true
    } else {
        false
    }
}    

pub fn get_input() -> Option<String> {
    let (window, mut gl, mut ui) = make_window("Add Schedule", 600, 200);
    let window_ref = Rc::new(RefCell::new(window));

    let light_bg = Color::new(0.8, 0.8, 0.8, 1.0);
    let (mut clicked, mut val) = (false, None);
    let ref mut sched_name = "".to_string();

    for event in Events::new(window_ref).ups(180).max_fps(60) {
        ui.handle_event(&event);
        if let Event::Render(args) = event {
            gl.draw([0, 0, args.width as i32, args.height as i32], |_, gl| {
                Background::new().color(light_bg.clone()).draw(&mut ui, gl);

                Label::new("Schedule Name").position(5.0, 10.0)
                    .size(18).color(light_bg.plain_contrast()).draw(&mut ui, gl);

                TextBox::new(0, sched_name)
                    .font_size(14)
                    .dimensions(100.0, 20.0)
                    .position(5.0, 35.0)
                    .frame(1.0)
                    .frame_color(light_bg.invert().plain_contrast())
                    .color(light_bg.clone())
                    .callback(|_string: &mut String| {})
                    .draw(&mut ui, gl);

                Button::new(1)
                    .dimensions(100.0, 60.0)
                    .position(5.0, 100.0)
                    .rgba(0.25, 0.25, 0.25, 1.0)
                    .frame(1.0)
                    .label("Ok")
                    .callback(|| {
                        if sched_name.len() == 0 {
                            pop_up_msg("Please enter a string.")
                        } else { 
                            clicked = true;
                            val = Some(sched_name.clone());
                        }
                    }).draw(&mut ui, gl);

                Button::new(2)
                    .dimensions(100.0, 60.0)
                    .position(110.0, 100.0)
                    .rgba(0.25, 0.25, 0.25, 1.0)
                    .frame(1.0)
                    .label("Cancel")
                    .callback(|| { 
                        clicked = true;
                        val = None;
                    }).draw(&mut ui, gl);
            });
        }
        
        if clicked {
            return val
        }
    }

    None
}

pub fn draw_ui(gl: &mut GlGraphics, ui: &mut Ui<GlyphCache>, xbtved: &mut XBTVEd) {
    let add_sched_uiid = 1;
    let prev_sched_uiid = 2;
    let next_sched_uiid = 3;
    let buff_display_uiid = 4;
    let del_sched_uiid = 16543;
//    let ref mut buffer = xbtved.get_schedule(xbtved.current_schedule).unwrap().to_string();

    Background::new().color(xbtved.bg_color).draw(ui, gl);
    
    Button::new(add_sched_uiid)
        .dimensions(200.0, 40.0)
        .position(10.0, 50.0)
        .color(xbtved.bg_color.plain_contrast())
        .frame(1.0)
        .label("Add New Schedule")
        .callback(|| {
            let (tx, rx) = channel();
            thread::spawn(move || {
                          tx.send(get_input()).unwrap();
                          });
            if let Ok(Some(sched_name)) = rx.recv() {
                let tags = Tags::new();
                let instrs = vec!(Instruction::Play(0, 0));
                let program = Program::new(Source::Pathname("foo".to_string()), tags, instrs);

                xbtved.push_schedule(Schedule::new(&sched_name, vec!(program)))
            };
        }).draw(ui, gl);

    Button::new(prev_sched_uiid)
        .dimensions(200.0, 40.0)
        .position(220.0, 50.0)
        .color(xbtved.bg_color.plain_contrast())
        .frame(1.0)
        .label("Previous Schedule")
        .callback(|| {
            xbtved.prev_schedule();
        }).draw(ui, gl);

    Button::new(next_sched_uiid)
        .dimensions(200.0, 40.0)
        .position(430.0, 50.0)
        .color(xbtved.bg_color.plain_contrast())
        .frame(1.0)
        .label("Next Schedule")
        .callback(|| {
            xbtved.next_schedule();
        }).draw(ui, gl);

    Button::new(del_sched_uiid)
        .dimensions(200.0, 40.0)
        .position(640.0, 50.0)
        .frame(1.0)
        .color(xbtved.bg_color.plain_contrast())
        .label("Delete Schedule")
        .callback(|| {
            let msg = format!("Do you really want to delete schedule {}?",
                              xbtved.get_schedule().name());
            if confirm(&msg) {
                let idx = xbtved.current_schedule;
                match xbtved.remove_schedule(idx) {
                    Ok(_) => { },
                    Err(f) => pop_up_msg(&f)
                }
            }
        }).draw(ui, gl);

    WidgetMatrix::new(xbtved.schedules.len(), 1)
        .dimensions(820.0, 40.0)
        .position(10.0, 100.0)
        .each_widget(|num, _col, _row, pos, dim| {
            Button::new(5 + num as u64)
                .dim(dim)
                .point(pos)
                .color(xbtved.bg_color.plain_contrast())
                .frame(1.0)
                .label(&xbtved.get_schedule_at(num).unwrap().name())
                .callback(|| {
                    match xbtved.change_current_schedule(num) {
                        Ok(_) => {},
                        Err(f) => pop_up_msg(&f)
                    };
                }).draw(ui, gl);
        });

//    let sched = xbtved.get_schedule();
    let mut sched_name = xbtved.get_schedule().name();

    TextBox::new(buff_display_uiid, &mut sched_name)
        .font_size(16)
        .dimensions(150.0, 30.0)
        .position(10.0, 145.0)
        .frame(1.0)
        .frame_color(xbtved.bg_color.invert().plain_contrast())
        .rgba(0.4, 0.1, 0.1, 1.0)
        .callback(|_string: &mut String| {})
        .draw(ui, gl); 

    WidgetMatrix::new(xbtved.get_schedule().programs_len(), 1)
        .dimensions(670.0, 30.0)
        .position(160.0, 145.0)
        .each_widget(|num, _col, _row, pos, dim| {
            let mut text = match xbtved.get_schedule().program_ref(num) {
                Some(prog) => prog.get_location().to_string(),
                None => "Whoops".to_string()
            };
            TextBox::new(1000 + num as u64, &mut text)
                .dim(dim)
                .point(pos)
                .rgba(0.1, 0.1, 0.4, 1.0)
                .frame(1.0)
                .callback(|_string: &mut String| {})
                .draw(ui, gl);
        });


    Button::new(8111)
        .dimensions(150.0, 40.0)
        .position(10.0, 180.0)
        .frame(1.0)
        .color(xbtved.bg_color.plain_contrast())
        .label("Change Name")
        .callback(|| {
            let (tx, rx) = channel();
            thread::spawn(move || {
                          tx.send(get_input()).unwrap();
                          });
            if let Ok(Some(new_name)) = rx.recv() {
                xbtved.modify_current_schedule().change_name(new_name);
            }
        }).draw(ui, gl);

    Button::new(8381)
        .dimensions(150.0, 40.0)
        .position(10.0, 180.0)
        .frame(1.0)
        .color(xbtved.bg_color.plain_contrast())
        .label("Add Program")
        .callback(|| {
            let (tx, rx) = channel();
            thread::spawn(move || {
                          tx.send(get_source()).unwrap();
                          });
            if let Ok(Some(prog_loc)) = rx.recv() {
                let source: Source  = match prog_loc. 
                xbtved.modify_current_schedule().(new_name);
            }
        }).draw(ui, gl);

/*    TextBox::new(buff_display_uiid, buffer)
        .font_size(16)
        .dimensions(780.0, 100.0)
        .position(10.0, 150.0)
        .frame(1.0)
        .frame_color(xbtved.bg_color.invert().plain_contrast())
        .color(xbtved.bg_color.invert())
        .callback(|_string: &mut String| {})
        .draw(ui, gl); */
}
