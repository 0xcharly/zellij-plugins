use std::cmp::{max, min};

use crate::{
    core::FrameBuffer,
    frame::{Frame, Styles, TabSegment},
};
use zellij_tile::prelude::{ModeInfo, TabInfo};

#[derive(Default)]
pub(crate) struct Renderer {
    mode: ModeInfo,
    tabs: Vec<TabInfo>,
    segments: Vec<TabSegment>,
    active_tab_idx: usize,

    styles: Styles,
}

impl Renderer {
    pub(crate) fn update_mode(&mut self, mode: ModeInfo) -> FrameBuffer {
        if self.mode == mode {
            return FrameBuffer::NoUpdates;
        }
        self.mode = mode;
        FrameBuffer::MarkDirty
    }

    pub(crate) fn update_tabs(&mut self, active_tab_idx: usize, tabs: Vec<TabInfo>) -> FrameBuffer {
        if self.active_tab_idx == active_tab_idx && self.tabs == tabs {
            return FrameBuffer::NoUpdates;
        }
        self.tabs = tabs;
        self.segments = self
            .tabs
            .iter()
            .enumerate()
            .map(|(index, _)| TabSegment::new(index))
            .collect();
        self.active_tab_idx = active_tab_idx;
        FrameBuffer::MarkDirty
    }

    pub(crate) fn next_frame(&self) -> Frame {
        Frame::new(
            &self.mode,
            self.active_tab_idx,
            &self.segments,
            &self.styles,
        )
    }

    pub(crate) fn get_next_tab_idx(&self) -> usize {
        min(self.active_tab_idx + 1, self.tabs.len())
    }

    pub(crate) fn get_prev_tab_idx(&self) -> usize {
        max(self.active_tab_idx.saturating_sub(1), 0)
    }

    pub(crate) fn get_target_tab(&self, mouse_click_col: usize) -> Option<usize> {
        let clicked_line_part = self.get_clicked_tab_segment(mouse_click_col)?;
        if clicked_line_part.index != self.active_tab_idx {
            Some(clicked_line_part.index)
        } else {
            None
        }
    }

    fn get_clicked_tab_segment(&self, mouse_click_col: usize) -> Option<&TabSegment> {
        let mut len = 0;
        for tab_line_part in &self.segments {
            if mouse_click_col >= len && mouse_click_col < len + tab_line_part.len() {
                return Some(tab_line_part);
            }
            len += tab_line_part.len() + 1; // Add 1 for the space separator.
        }
        None
    }
}
