use std::{cell::RefCell, rc::Rc};

use ratatui::widgets::ListState;

use crate::{algorithm::{maze::noise_map::NoiseMap, pathfinding::a_star::AStar, Algorithm}, grid::{Grid, GridState}};

pub struct Sidebar {
    pub page: SidebarPage,
    pub state: ListState,
}

impl Sidebar {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Self {
            page: SidebarPage::Main,
            state,
        }
    }

    pub fn next(&mut self) {
        if let Some(o) = self.state.selected() {
            if o == self.page.options().len() - 1 {
                self.state.select(Some(0));
            } else {
                self.state.select(Some(o+1));
            }
        } else {
            self.state.select(Some(0));
        }
    }

    pub fn prev(&mut self) {
        if let Some(o) = self.state.selected() {
            if o == 0 {
                self.state.select(Some(self.page.options().len() - 1));
            } else {
                self.state.select(Some(o-1));
            }
        } else {
            self.state.select(Some(0));
        }
    }

    pub fn select(&mut self, grid: &mut Grid) {
        if let Some(o) = self.state.selected() && let Some(action) = &self.page.options()[o].action {
            match action {
                SidebarAction::SwitchPage(page) => {
                    self.state.select(Some(0));
                    self.page = page.clone();
                },
                SidebarAction::InitAlgorithm(algorithm) => {
                    self.page = SidebarPage::Main;
                    self.state.select(Some(0));

                    grid.state = GridState::Generating(Rc::clone(algorithm));
                }
            }
        } else {
            self.state.select(Some(0));
        }
    }
}

#[derive(Clone)]
pub enum SidebarPage {
    Main,
    MazeGenerationAlgorithms,
    PathfindingAlgorithms,
}

impl SidebarPage {
    pub fn options(&self) -> Vec<SidebarOption> {
        match self {
            SidebarPage::Main =>
                vec![
                    SidebarOption::new("View Maze Algorithms", Some(SidebarAction::SwitchPage(SidebarPage::MazeGenerationAlgorithms))),
                    SidebarOption::new("View Pathfinding Algorithms", Some(SidebarAction::SwitchPage(SidebarPage::PathfindingAlgorithms))),
                ],
            SidebarPage::MazeGenerationAlgorithms =>
                vec![
                    SidebarOption::new("Recursive Backtracking", None),
                    SidebarOption::new("Prim's", None),
                    SidebarOption::new("Noise Map", Some(SidebarAction::InitAlgorithm(Rc::new(RefCell::new(NoiseMap::new(10)))))),
                    SidebarOption::new("Back", Some(SidebarAction::SwitchPage(SidebarPage::Main)))
                ],
            SidebarPage::PathfindingAlgorithms =>
                vec![
                    SidebarOption::new("A*", Some(SidebarAction::InitAlgorithm(Rc::new(RefCell::new(AStar::new((0, 0), (106, 37))))))),
                    SidebarOption::new("BFS", None),
                    SidebarOption::new("Dijkstra's", None),
                    SidebarOption::new("Back", Some(SidebarAction::SwitchPage(SidebarPage::Main)))
                ]
        }
    }
}

pub struct SidebarOption {
    pub title: &'static str,
    action: Option<SidebarAction>,
}

impl SidebarOption {
    fn new(title: &'static str, action: Option<SidebarAction>) -> Self {
        Self {
            title,
            action,
        }
    }
}

enum SidebarAction {
    SwitchPage(SidebarPage),
    InitAlgorithm(Rc<RefCell<dyn Algorithm>>)
}
