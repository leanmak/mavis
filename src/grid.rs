use std::{cell::RefCell, rc::Rc};

use ratatui::{style::{Color, Style}, text::Span};

use crate::algorithm::Algorithm;

pub enum GridState {
    Idle,
    Generating(Rc<RefCell<dyn Algorithm>>)
}

pub enum NodeType {
    Empty,
    Wall,
    Visited,
    Path,
}

impl NodeType {
    pub fn to_span(&self) -> Span {
        match self {
            Self::Empty => Span::styled(" ", Style::default().fg(Color::White)),
            Self::Wall => Span::styled("█", Style::default().fg(Color::White)),
            Self::Visited => Span::styled("&", Style::default().fg(Color::DarkGray)),
            Self::Path => Span::styled(".", Style::default().fg(Color::LightGreen)),
        }
    }
}

pub struct Node {
    pub node_type: NodeType
}

pub struct Grid {
    pub state: GridState,
    pub content: Vec<Vec<Node>>
}

impl Grid {
    pub fn new() -> Self {
        Self {
            state: GridState::Idle,
            content: Vec::new()
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