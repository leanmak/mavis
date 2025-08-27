use ratatui::{
    layout::{ Constraint, Layout, Rect },
    style::{ Color, Style, Stylize },
    symbols::border,
    text::{ Line, Span, Text },
    widgets::{ Block, List, Paragraph },
    Frame,
};

use crate::{app::App, grid::{GridState, Node, NodeType}};

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
    draw_grid(app, frame, grid);
}

fn draw_grid(app: &mut App, frame: &mut Frame, grid: Rect) {
    let (map_width, map_height) = (grid.width - 2, grid.height - 2);

    // generate new grid on resize
    if app.grid.width() != map_width as usize || app.grid.height() != map_height as usize {
        app.grid.content = (0..map_height).map(|_| {
            (0..map_width).map(|_| Node { node_type: NodeType::Empty }).collect()
        }).collect();

        app.grid.grid_start = Some((grid.left() as i32 + 1, grid.top() as i32 + 1));
        app.grid.grid_end = Some((grid.right() as i32 - 1, grid.bottom() as i32 - 1));
    }

    let border_title = if let GridState::PlacingMarkers(_) = app.grid.state {
        if app.grid.markers.start == None {
            String::from(" Click anywhere on the grid to place the START marker... ")
        } else {
            String::from(" Click anywhere on the grid to place the END marker... ")
        }
    } else {
        format!(" Main Grid ({} x {})", map_width, map_height)
    };

    let border = Block::bordered().title(border_title).border_set(border::THICK);

    frame.render_widget(border, Rect {
        x: grid.left(),
        y: grid.top(),
        width: grid.width,
        height: grid.height,
    });

    // draw nodes on screen
    let content: Vec<Line> = app.grid.content.iter().map(|grid_row| {
        let nodes: Vec<Span> = grid_row.iter().map(|n| n.node_type.to_span()).collect();
        Line::from(nodes)
    }).collect();

    frame.render_widget(Paragraph::new(Text::from(content)), Rect {
        x: grid.left() + 1,
        y: grid.top() + 1,
        width: map_width,
        height: map_height,
    });

    if let Some(position) = app.grid.markers.start {
        frame.render_widget(Text::from("S"), Rect {
            x: position.0 as u16,
            y: position.1 as u16,
            width: 1,
            height: 1
        });
    }

    if let Some(position) = app.grid.markers.end {
        frame.render_widget(Text::from("E"), Rect {
            x: position.0 as u16,
            y: position.1 as u16,
            width: 1,
            height: 1
        });
    }
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
