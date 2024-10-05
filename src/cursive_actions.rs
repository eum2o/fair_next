use crate::{cursive_actions, fsys, FILE_NAME};
use cursive::align::HAlign;
use cursive::event::Key::{Enter, Esc};
use cursive::event::{Event, EventResult};
use cursive::view::Nameable;
use cursive::views::{Dialog, NamedView, OnEventView, SelectView};
use cursive::Cursive;
use fsys::get_file_handle;
use std::io::Write;

const VIEW_NAME: &str = "names_view";

pub(crate) fn create_view(file_lines: Vec<&str>) -> NamedView<OnEventView<SelectView>> {
    let mut select_view: SelectView<String> = SelectView::new()
        .h_align(HAlign::Left)
        .autojump();

    select_view.add_all_str(file_lines);

    OnEventView::new(select_view)
        .on_event(Esc, |s| s.quit())
        .on_pre_event_inner(Event::CtrlChar('s'), cursive_actions::save)
        .on_pre_event_inner(Enter, |s, _| {
            let sel_id = s.selected_id().unwrap();
            let name = s.get_item(sel_id).unwrap().1.clone();
            s.insert_item(s.len(), &name, name.clone());
            s.remove_item(sel_id);
            let cb = s.set_selection(0);
            Some(EventResult::Consumed(Some(cb)))
        }).with_name(VIEW_NAME)
}

pub(crate) fn save_cursive(cursive: &mut Cursive) {
    if let Some(mut view_ref) = cursive.find_name::<OnEventView<SelectView>>(VIEW_NAME) {
        let select_view = view_ref.get_inner_mut();
        match save_sel_view(select_view) {
            // TODO give some feedback that saving worked
            Ok(_) => {}
            Err(e) => {}
        }
    } else {
        cursive.add_layer(Dialog::info("Error: Couldn't find the view to save."));
    }
}

pub(crate) fn save_sel_view(select_view: &mut SelectView) -> std::io::Result<()> {
    let names: Vec<&str> = select_view.iter().map(|(_, name)| name.as_str()).collect();
    let name_lines = names.join("\n");
    get_file_handle(FILE_NAME)?.write_all(name_lines.as_bytes())
}

pub(crate) fn save(select_view: &mut SelectView, _: &Event) -> Option<EventResult> {
    save_sel_view(select_view).expect("Error saving view.");
    Some(EventResult::Consumed(None))
}
