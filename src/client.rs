use crate::data_types::{Border, ColorScheme, WinId};
use crate::xconnection::XConn;

/**
 * Meta-data around a client window that we are handling.
 * Primarily state flags and information used when determining which clients
 * to show for a given monitor and how they are tiled.
 */
#[derive(Debug, PartialEq, Clone)]
pub struct Client {
    id: WinId,
    wm_class: String,
    // state flags
    focused: bool,
    floating: bool,
    fullscreen: bool,
}

impl Client {
    pub fn new(id: WinId, wm_class: String, floating: bool) -> Client {
        Client {
            id,
            wm_class,
            focused: false,
            floating: floating,
            fullscreen: false,
        }
    }

    pub fn id(&self) -> WinId {
        self.id
    }

    pub fn class(&self) -> &str {
        &self.wm_class
    }

    pub fn is_focused(&self) -> bool {
        self.focused
    }

    pub fn focus(&mut self, conn: &dyn XConn, scheme: &ColorScheme) {
        conn.focus_client(self.id);
        self.set_window_border(conn, Border::Focused, scheme);
        self.focused = true;
    }

    pub fn unfocus(&mut self, conn: &dyn XConn, scheme: &ColorScheme) {
        self.set_window_border(conn, Border::Unfocused, scheme);
        self.focused = false;
    }

    fn set_window_border(&mut self, conn: &dyn XConn, border: Border, scheme: &ColorScheme) {
        let color = match border {
            Border::Urgent => scheme.urgent,
            Border::Focused => scheme.highlight,
            Border::Unfocused => scheme.fg_1,
        };

        conn.set_client_border_color(self.id, color);
    }
}
