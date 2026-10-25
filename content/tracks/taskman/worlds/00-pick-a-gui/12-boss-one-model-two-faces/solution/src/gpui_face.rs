//! The list, drawn with GPUI.

use gpui::{Context, IntoElement, Render, SharedString, Window, div, prelude::*};

use crate::model::{SortBy, TaskList};

/// The list as a GPUI view.
pub struct TaskListView {
    pub list: TaskList,
}

/// A column header that sorts the list by `sort` when clicked.
fn header(label: &'static str, sort: SortBy, cx: &mut Context<TaskListView>) -> impl IntoElement {
    div()
        .id(SharedString::from(format!("header-{label}")))
        .debug_selector(move || format!("header-{label}"))
        .child(label)
        .on_click(cx.listener(move |view, _, _, cx| {
            view.list.sort = sort;
            cx.notify();
        }))
}

impl Render for TaskListView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let headers = div()
            .flex()
            .gap_4()
            .child(header("CPU", SortBy::Cpu, cx))
            .child(header("Memory", SortBy::Memory, cx))
            .child(header("Name", SortBy::Name, cx));
        let rows = self.list.visible().into_iter().enumerate().map(|(i, row)| {
            div()
                .debug_selector(move || format!("row-{i}"))
                .flex()
                .gap_4()
                .child(row.name)
                .child(row.cpu)
                .child(row.memory)
        });
        div().flex().flex_col().child(headers).children(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_processes;
    use gpui::{Entity, Modifiers, TestAppContext, VisualTestContext};

    fn open<'a>(cx: &'a mut TestAppContext, filter: &str) -> (Entity<TaskListView>, &'a mut VisualTestContext) {
        let mut list = TaskList::new(sample_processes());
        list.filter = filter.to_string();
        let (view, cx) = cx.add_window_view(|_, _| TaskListView { list });
        cx.run_until_parked();
        (view, cx)
    }

    #[gpui::test]
    fn gpui_memory_header_sorts_by_memory(cx: &mut TestAppContext) {
        let (view, cx) = open(cx, "");
        let at = cx.debug_bounds("header-Memory").expect("no box named header-Memory was drawn").center();
        cx.simulate_click(at, Modifiers::none());
        cx.run_until_parked();
        assert_eq!(view.read_with(cx, |view, _| view.list.sort), SortBy::Memory);
    }

    #[gpui::test]
    fn gpui_draws_one_row_per_visible_process(cx: &mut TestAppContext) {
        let (_view, cx) = open(cx, "chrome");
        assert!(cx.debug_bounds("row-0").is_some(), "no box named row-0: are the rows drawn from visible()?");
        assert!(cx.debug_bounds("row-1").is_some(), "two processes match \"chrome\", so row-1 should be drawn");
        assert!(cx.debug_bounds("row-2").is_none(), "only two processes match \"chrome\", but row-2 was drawn");
    }
}
