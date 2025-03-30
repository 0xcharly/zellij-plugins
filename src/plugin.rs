use crate::core::{EventLoopResult, FrameBuffer, ResultIterator};
use crate::renderer::Renderer;
use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[derive(Default)]
pub(crate) struct UltraCompactBar {
    /// All permissions are required to fulfill our purpose.
    permissions_granted: bool,
    /// Events queued until the first `Event::SessionUpdate` is received.
    event_queue: Vec<Event>,

    /// The logic to update and render the UI.
    renderer: Renderer,
}

impl ZellijPlugin for UltraCompactBar {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        // [ChangeApplicationState] is required for logging to Zellij's log and switching session.
        request_permission(&[
            PermissionType::ChangeApplicationState,
            PermissionType::ReadApplicationState,
        ]);

        subscribe(&[
            EventType::ModeUpdate,
            EventType::Mouse,
            EventType::PermissionRequestResult,
            EventType::TabUpdate,
        ]);

        if self.permissions_granted {
            // Initialize the plugin immediatelly since permissions have already been granted.
            self.on_permissions_granted();
        }
    }

    fn update(&mut self, event: Event) -> bool {
        let result = if let Event::PermissionRequestResult(PermissionStatus::Granted) = event {
            self.permissions_granted = true;
            self.on_permissions_granted();
            self.drain_events()
        } else if self.permissions_granted {
            self.handle_event(event)
        } else {
            self.event_queue.push(event);
            return false; // No need to update the UI.
        };

        match result {
            Ok(FrameBuffer::MarkDirty) => true,
            Ok(FrameBuffer::NoUpdates) => false,
            Err(_) => true,
        }
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        print!("{}", self.renderer.next_frame());
    }
}

impl UltraCompactBar {
    fn on_permissions_granted(&self) {
        set_selectable(false);
    }

    /// Consumes as many of the queued events as possible, and returns either the final combined
    /// [EventLoopResult] value, or the first error that occurred.
    ///
    /// If an error occurs, it is propagated back to the caller immediately (short-circuiting). All
    /// remaining queued events are dropped.
    fn drain_events(&mut self) -> EventLoopResult {
        std::mem::take(&mut self.event_queue)
            .into_iter()
            .map(|event| self.handle_event(event))
            .try_consume()
    }

    fn handle_event(&mut self, event: Event) -> EventLoopResult {
        match event {
            Event::PermissionRequestResult(PermissionStatus::Granted) => {
                unreachable!("Already handled in `update(event)`");
            }
            Event::PermissionRequestResult(PermissionStatus::Denied) => {
                self.permissions_granted = false;
                close_self();
                return Ok(FrameBuffer::NoUpdates);
            }
            Event::ModeUpdate(mode) => {
                return Ok(self.renderer.update_mode(mode));
            }
            Event::TabUpdate(tabs) => {
                let Some(active_tab_index) = tabs.iter().position(|t| t.active) else {
                    eprintln!("Internal Zellij error: no active tab found");
                    return Ok(FrameBuffer::NoUpdates);
                };

                return Ok(self.renderer.update_tabs(active_tab_index, tabs));
            }
            Event::Mouse(mouse_event) => match mouse_event {
                Mouse::LeftClick(_, column) => {
                    if let Some(idx) = self.renderer.get_target_tab(column) {
                        switch_tab_to(idx);
                    }
                }
                Mouse::ScrollUp(_) => switch_tab_to(self.renderer.get_next_tab_idx()),
                Mouse::ScrollDown(_) => switch_tab_to(self.renderer.get_prev_tab_idx()),
                _ => (),
            },
            _ => eprintln!("Unexpected event: {event:?}"),
        };

        Ok(FrameBuffer::NoUpdates)
    }
}

// The Zellij `switch_tab_to` API expects a 1-based u32 index.
fn switch_tab_to(index: usize) {
    zellij_tile::prelude::switch_tab_to((index + 1) as u32);
}
