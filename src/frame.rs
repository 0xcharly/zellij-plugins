#![allow(dead_code)]
// TODO: use std::iter::Itersperse when available.
// https://doc.rust-lang.org/std/iter/struct.Intersperse.html
#![allow(unstable_name_collisions)]

use std::fmt::{Display, Formatter, Result};

use ansi_term::Colour::{self, RGB};
use ansi_term::{ANSIStrings, Style};
use itertools::Itertools;
use zellij_tile::prelude::{InputMode, ModeInfo};

pub(crate) struct Styles {
    fill: Style,
    mode: Style,
    active_tab: Style,
    inactive_tab: Style,
}

const GREY: u8 = 0;
const RED: u8 = 1;
const GREEN: u8 = 2;
const YELLOW: u8 = 3;
const BLUE: u8 = 4;
const MAGENTA: u8 = 5;
const CYAN: u8 = 6;
const WHITE: u8 = 7;

/// The catppuccin "colorscheme" background.
const CATPPUCCIN_SURFACE_BLUE: Colour = RGB(32, 49, 71);

const CATPPUCCIN_ON_SURFACE_BLUE: Colour = RGB(159, 205, 254);
const CATPPUCCIN_MANTLE: Colour = RGB(17, 22, 29);

const RESET: &str = "\x1B[0m";

impl Default for Styles {
    fn default() -> Self {
        Styles {
            fill: Style::new().dimmed().on(CATPPUCCIN_MANTLE),
            mode: Style::new().bold().on(CATPPUCCIN_MANTLE),
            active_tab: Style::new()
                .fg(CATPPUCCIN_ON_SURFACE_BLUE)
                .on(CATPPUCCIN_SURFACE_BLUE),
            inactive_tab: Style::new().dimmed().on(CATPPUCCIN_MANTLE),
        }
    }
}

fn rgb(colour: Colour) -> (u8, u8, u8) {
    match colour {
        RGB(r, g, b) => return (r, g, b),
        _ => panic!("Expected non-RGB colour variant"),
    }
}

#[derive(Debug, Default)]
pub(crate) struct TabSegment {
    pub(crate) index: usize,
    name: String,
}

impl TabSegment {
    pub(crate) fn new(index: usize) -> Self {
        TabSegment {
            index,
            name: (index + 1).to_string(),
        }
    }
    pub(crate) fn len(&self) -> usize {
        self.name.len()
    }
}

pub(crate) struct Frame<'ui> {
    active_tab_idx: usize,
    mode: &'ui ModeInfo,
    segments: &'ui Vec<TabSegment>,
    styles: &'ui Styles,
}

impl<'ui> Frame<'ui> {
    pub(crate) fn new(
        mode: &'ui ModeInfo,
        active_tab_idx: usize,
        segments: &'ui Vec<TabSegment>,
        styles: &'ui Styles,
    ) -> Self {
        Frame {
            active_tab_idx,
            mode,
            segments,
            styles,
        }
    }
    pub(crate) fn fmt_status_bar(&self, f: &mut Formatter<'_>) -> Result {
        if self.segments.is_empty() {
            return Ok(());
        }

        let mut segments = self
            .segments
            .iter()
            .map(|segment| {
                if segment.index == self.active_tab_idx {
                    self.styles.active_tab.paint(&segment.name)
                } else {
                    self.styles.inactive_tab.paint(&segment.name)
                }
            })
            .map(|s| Vec::from([s]))
            .collect::<Vec<_>>();

        segments.push(Vec::from([
            self.styles.fill.paint("("),
            self.styles.mode.paint(self.fmt_input_mode()),
            self.styles.fill.paint(")"),
        ]));

        let segments = segments
            .into_iter()
            .intersperse(Vec::from([self.styles.fill.paint(" ")]))
            .flatten()
            .collect::<Vec<_>>();

        // Use ANSI escape sequences manually to fill out the line without repeating spaces.
        let (r, g, b) = rgb(CATPPUCCIN_MANTLE);
        write!(
            f,
            "{}\u{1b}[48;2;{};{};{}m\u{1b}[0K",
            ANSIStrings(&segments),
            r,
            g,
            b
        )
    }

    fn fmt_input_mode(&self) -> &str {
        match self.mode.mode {
            InputMode::Normal => "NORMAL",
            InputMode::Locked => "LOCKED",
            InputMode::Resize => "RESIZE",
            InputMode::Pane => "PANE",
            InputMode::Tab => "TAB",
            InputMode::Scroll => "SCROLL",
            InputMode::RenameTab => "RENAME TAB",
            InputMode::RenamePane => "RENAME PANE",
            InputMode::Search => "SEARCH",
            InputMode::EnterSearch => "ENTER SEARCH",
            InputMode::Session => "SESSION",
            InputMode::Move => "MOVE",
            InputMode::Prompt => "PROMPT",
            InputMode::Tmux => "TMUX",
        }
    }
}

impl Display for Frame<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.fmt_status_bar(f)
    }
}
