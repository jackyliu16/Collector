use ratatui::{
    buffer::Buffer,
    layout::{Layout, Rect},
    style::{palette::tailwind, Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Text},
    widgets::{
        Block, Borders, Cell, List, ListItem, Paragraph, Row, StatefulWidget, Table, Tabs, Widget,
    },
    Frame,
};
use strum::IntoEnumIterator;

use crate::app::{App, Mode, SelectedTab};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    use ratatui::layout::Constraint::{Length, Min};

    let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
    let [header_area, inner_data, footer_data] = vertical.areas(frame.area());

    let horizontal = Layout::horizontal([Min(0), Length(38)]);
    let [tabs_area, title_area] = horizontal.areas(header_area);

    render_title(title_area, frame.buffer_mut());
    app.render_tabs_name(tabs_area, frame.buffer_mut());
    if app.mode == Mode::Editing {
        app.render_input_box(inner_data, frame.buffer_mut());
    } else {
        app.render_convert_box(inner_data, frame.buffer_mut());
    }

    app.render_footer(footer_data, frame.buffer_mut());
}

// Right of header_area
fn render_title(area: Rect, buf: &mut Buffer) {
    let title = Paragraph::new("Simple decompression password manager")
        .style(Style::default().add_modifier(Modifier::BOLD));
    title.render(area, buf);
}

impl App {
    // Left of header_area
    pub fn render_tabs_name(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), tailwind::BLUE.c700);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }

    pub fn render_input_box(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.message.as_str())
            .block(Block::bordered().title("Input"))
            .render(area, buf);
    }

    pub fn render_convert_box(&mut self, area: Rect, buf: &mut Buffer) {
        use ratatui::layout::Constraint::{Length, Min};
        let vertical = Layout::vertical([Min(0), Length(3)]);
        let [history_area, input_area] = vertical.areas(area);

        Block::new()
            .title("Cmd History")
            .borders(Borders::ALL)
            .render(history_area, buf);

        // Cmd History
        let list_items: Vec<ListItem> = self
            .cmd
            .history
            .iter()
            .map(|item| ListItem::new(format!("$ {}", item.join("\n"))))
            .collect();
        let list = List::new(list_items)
            .block(Block::default().title("Cmd History").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol("> ");
        StatefulWidget::render(list, history_area, buf, &mut self.cmd.state);

        // Input Box
        Paragraph::new(format!(" $ {}", self.cmd.input.clone()))
            .block(
                Block::default()
                    .title("Input")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            // .bg(tailwind::YELLOW.c900)
            .render(input_area, buf)
    }

    pub fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.help_message.clone())
            .style(Style::default().add_modifier(Modifier::BOLD))
            .render(area, buf);
    }
}
