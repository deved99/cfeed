// STD
use std::rc::Rc;
// Local
use crate::common::term;
use crate::config::LoadedConfig;
use crate::element::Element;
// External
use gtk4::prelude::*;
use gtk4::{
    self,
    Align, Application, ApplicationWindow, Label, ListBox, ListBoxRow,
    Notebook, PositionType, ScrolledWindow
};

pub fn run() {
    let application = Application::builder()
        .application_id("org.deved99.cfeed")
        .build();

    application.connect_activate(move |a| {
        let window = ApplicationWindow::builder()
            .application(a)
            .title("cFeed")
            .default_width(1920)
            .default_height(1080)
            .build();
        let d = get_conf();
        build_ui(&d, &window)
    });

    application.run();
}

fn get_conf() -> LoadedConfig {
    match LoadedConfig::load() {
        Err(why) => term!("{:#?}", why),
        Ok(c) => c
    }
}

fn build_ui(d: &LoadedConfig, window: &ApplicationWindow) {
    // All elements
    let n = Notebook::builder().tab_pos(PositionType::Left).build();
    let all = Label::new(Some("All"));
    let all_c = add_content(d.elements.iter());
    n.append_page(&all_c, Some(&all));
    // Tabs
    add_categories(&d, &n);
    window.set_child(Some(&n));
    window.show();
}

fn add_categories(d: &LoadedConfig, n: &Notebook) {
    for (k, v) in &d.categories {
        let l = Label::new(Some(&k));
        let iter = d.elements.iter().filter(|e| v.contains(e.from()));
        let c = add_content(iter);
        n.append_page(&c, Some(&l));
    }
}

fn add_content<'a>(d: impl IntoIterator<Item = &'a Rc<Element>>) -> ScrolledWindow {
    let list = ListBox::builder().halign(Align::Start).build();
    let scroll = ScrolledWindow::builder().child(&list).build();
    for e in d {
        let l = Label::builder()
            .label(&e.pretty())
            .halign(Align::Start)
            .use_markup(true)
            .build();
        let r = ListBoxRow::builder().child(&l).build();
        let ec = e.clone();
        r.connect_activate(move |_| println!("Executing: {:#?}", ec.open()));
        list.append(&r)
    }
    scroll
}
