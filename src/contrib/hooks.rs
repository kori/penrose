//! Additional common hooks that can be used out of the box with minimal config.
use crate::core::client::Client;
use crate::core::data_types::Selector;
use crate::core::helpers::spawn;
use crate::core::hooks::{LayoutChangeHook, NewClientHook, WorkspaceChangeHook};
use crate::core::manager::WindowManager;

/**
 * Automatically set the X root window WM_NAME property to be the WM_NAME of the
 * active window. If WM_NAME is not set for the active window, then a default
 * value of 'n/a' is set instead.
 * This is intended for use with external programs such as Polybar as a way of
 * exposing state.
 * NOTE: currently, WM_NAME is read when the window is first mapped only.
 */
pub struct ActiveClientAsRootName {}
impl ActiveClientAsRootName {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}
impl NewClientHook for ActiveClientAsRootName {
    fn call(&mut self, wm: &mut WindowManager, c: &mut Client) {
        wm.set_root_window_name(c.wm_name());
    }
}

/**
 * Automatically set the X root window WM_NAME property to be the current layout
 * symbol for the active workspace.
 * This is intended for use with external programs such as Polybar as a way of
 * exposing state.
 */
pub struct LayoutSymbolAsRootName {}
impl LayoutSymbolAsRootName {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}
impl LayoutChangeHook for LayoutSymbolAsRootName {
    fn call(&mut self, wm: &mut WindowManager, _: usize, _: usize) {
        wm.set_root_window_name(wm.current_layout_symbol());
    }
}

/**
 * Whenever a focus moves to the workspace 'name' and the workspace is empty,
 * set a specific layout and spawn a set of default clients. The layout is set
 * first and then clients are spawned in the order they are defined using the
 * penrose::core::helpers::spawn function. This means that the final client will
 * have focus and the the clients will be arranged based on the order they are
 * spawned.
 */
pub struct DefaultWorkspace<'a> {
    defaults: Vec<&'a str>,
    layout: &'static str,
    name: &'static str,
}
impl<'a> DefaultWorkspace<'a> {
    /// Create a new DefaultWorkspace that is pre-boxed for adding to your workspace_hooks
    pub fn new(name: &'static str, layout: &'static str, defaults: Vec<&'a str>) -> Box<Self> {
        Box::new(Self {
            name,
            layout,
            defaults,
        })
    }
}
impl<'a> WorkspaceChangeHook for DefaultWorkspace<'a> {
    fn call(&mut self, wm: &mut WindowManager, _: usize, new: usize) {
        let ws = wm.workspace_mut(Selector::Index(new)).unwrap();
        if ws.name() == self.name && ws.len() == 0 {
            // can fail if the layout symbol is wrong
            ws.try_set_layout(self.layout);
            self.defaults.iter().for_each(|prog| spawn(*prog));
        }
    }
}
