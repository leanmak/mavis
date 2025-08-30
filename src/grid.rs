use std::{cell::RefCell, rc::Rc};

use ratatui::{style::{Color, Style}, text::Span};

use crate::algorithm::{Algorithm, Coord};

pub enum GridState {
    Idle,
    Generating(Rc<RefCell<dyn Algorithm>>),
    PlacingMarkers(Rc<RefCell<dyn Algorithm>>)
}

#[derive(PartialEq)]
pub enum NodeType {
    Empty,
    Wall,
    Visited,
    Path,
}

impl NodeType {
    pub fn to_span(&self) -> Span<'_> {
        match self {
            Self::Empty => Span::styled(" ", Style::default().fg(Color::White)),
            Self::Wall => Span::styled("█", Style::default().fg(Color::White)),
            Self::Visited => Span::styled(".", Style::default().fg(Color::DarkGray)),
            Self::Path => Span::styled("@", Style::default().fg(Color::LightGreen)),
        }
    }
}

pub struct Node {
    pub node_type: NodeType
}

pub struct Markers {
    pub start: Option<Coord>,
    pub end: Option<Coord>,
}

impl Markers {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }
}

pub struct Grid {
    pub state: GridState,
    pub content: Vec<Vec<Node>>,
    pub markers: Markers,
    pub grid_start: Option<Coord>,
    pub grid_end: Option<Coord>,
    pub clear: bool,
    pub iter_count: i32,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            state: GridState::Idle,
            content: Vec::new(),
            markers: Markers::new(),
            grid_start: None,
            grid_end: None,
            clear: false,
            iter_count: 0,
        }
    }

    pub fn height(&self) -> usize {
        self.content.len()
    }

    pub fn width(&self) -> usize {
        if self.height() == 0 {
            return 0;
        }

        self.content[0].len()
    }
}