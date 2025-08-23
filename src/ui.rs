use ratatui::{
    layout::{ Constraint, Layout, Rect },
    style::{ Color, Style, Stylize },
    symbols::border,
    text::{ Line, Span, Text },
    widgets::{ Block, List, Paragraph },
    Frame,
};

use crate::app::App;

pub fn draw(app: &mut App, frame: &mut Frame) {
    let app_layout = Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Percentage(2),
        Constraint::Percentage(85),
    ]);
    let [header_area, _, main_area] = app_layout.areas(frame.area());

    draw_header(frame, header_area);
    draw_main_area(app, frame, main_area);
}

fn draw_header(frame: &mut Frame, header_area: Rect) {
    let header_area_layout = Layout::horizontal([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ]);
    let [header_left, header_right] = header_area_layout.areas(header_area);

    frame.render_widget(Paragraph::new(Text::from(mavis_title())), header_left);
    frame.render_widget(Paragraph::new("Iteration: 0 | Speed: 0 cells/sec"), Rect {
        x: header_right.right() - ("Iteration: 0 | Speed: 0 cells/sec".len() as u16) - 1,
        y: header_right.bottom() - 1,
        width: "Iteration: 0 | Speed: 0 cells/sec".len() as u16,
        height: 1,
    });
}

fn draw_main_area(app: &mut App, frame: &mut Frame, main_area: Rect) {
    let main_area_layout = Layout::horizontal([
        Constraint::Percentage(70),
        Constraint::Percentage(3),
        Constraint::Percentage(27),
    ]);
    let [grid, _, sidebar_area] = main_area_layout.areas(main_area);

    draw_sidebar(app, frame, sidebar_area);
    draw_grid(frame, grid);
}

fn draw_grid(frame: &mut Frame, grid: Rect) {
    //let (map_width, map_height) = (grid.width - 2, grid.height - 2);

    let border = Block::bordered().title(" Main Grid ").border_set(border::THICK);

    frame.render_widget(border, Rect {
        x: grid.left(),
        y: grid.top(),
        width: grid.width,
        height: grid.height,
    });
}

fn draw_sidebar(app: &mut App, frame: &mut Frame, sidebar_area: Rect) {
    let sidebar_area_container = Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Percentage(70),
        Constraint::Percentage(20),
    ]);
    let [_, sidebar, sidebar_description] = sidebar_area_container.areas(sidebar_area);

    let sidebar_description_text = Paragraph::new(
        Text::from(
            vec![
                Line::from(Span::styled("[O] Increase Speed", Style::default().fg(Color::White))),
                Line::from(Span::styled("[P] Decrease Speed", Style::default().fg(Color::White))),
                Line::from(Span::styled("[Space] Resume/Pause", Style::default().fg(Color::White))),
                Line::from(
                    Span::styled("[R] Reset/Stop Algorithm", Style::default().fg(Color::White))
                ),
                Line::from(Span::styled("[Q] Quit Application", Style::default().fg(Color::White)))
            ]
        )
    );
    frame.render_widget(sidebar_description_text, sidebar_description);

    let options = List::new(
        app.sidebar.page
            .options()
            .iter()
            .map(|o| o.title)
    )
        .block(Block::bordered().title(" What would you like to do? "))
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(
        options,
        Rect {
            x: sidebar.left(),
            y: sidebar.top(),
            width: sidebar.width,
            height: sidebar.height,
        },
        &mut app.sidebar.state
    );
}

fn mavis_title() -> Vec<Line<'static>> {
    vec![
        Line::from(Span::styled("                  __ ", Style::default().fg(Color::White))),
        Line::from(Span::styled("|\\/|  /\\  \\  / | /__`", Style::default().fg(Color::White))),
        Line::from(Span::styled("|  | /~~\\  \\/  | .__/", Style::default().fg(Color::White)))
    ]
}
