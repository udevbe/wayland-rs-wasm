//! Protocols related to window management

#![cfg_attr(rustfmt, rustfmt_skip)]

pub mod shell {
    //! XDG Shell protocol
    //!
    //! Exposes the `xdg_wm_base` global, which deprecates and replaces `wl_shell`.

    wayland_protocol!(
        "./protocols/stable/xdg-shell/xdg-shell.xml",
        []
    );
}
