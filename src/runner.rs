#![allow(clippy::too_many_arguments)]

use crate::app;
use crate::auto_refresher;
use crate::explorer;
use crate::pipe_reader;
use crate::pwd_watcher;
use crate::ui;
use anyhow::Result;
use handlebars::{handlebars_helper, Handlebars};
use humansize::{file_size_opts as options, FileSize};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::mpsc;
use tui::Terminal;

handlebars_helper!(to_humansize: |size: i64| size.file_size(options::CONVENTIONAL).unwrap_or_default());

fn call(app: &app::App, cmd: app::Command, silent: bool) -> io::Result<ExitStatus> {
    let focus_index = app
        .directory_buffer()
        .map(|d| d.focus())
        .unwrap_or_default()
        .to_string();

    let (stdin, stdout, stderr) = if silent {
        (Stdio::null(), Stdio::null(), Stdio::null())
    } else {
        (Stdio::inherit(), Stdio::inherit(), Stdio::inherit())
    };

    Command::new(cmd.command().clone())
        .env("XPLR_APP_VERSION", app.version())
        .env("XPLR_CONFIG_VERSION", app.config().version())
        .env("XPLR_PID", &app.pid().to_string())
        .env("XPLR_INPUT_BUFFER", app.input_buffer().unwrap_or_default())
        .env("XPLR_FOCUS_PATH", app.focused_node_str())
        .env("XPLR_FOCUS_INDEX", focus_index)
        .env("XPLR_SESSION_PATH", app.session_path())
        .env("XPLR_PIPE_MSG_IN", app.pipe().msg_in())
        .env("XPLR_PIPE_SELECTION_OUT", app.pipe().selection_out())
        .env("XPLR_PIPE_HISTORY_OUT", app.pipe().history_out())
        .env("XPLR_PIPE_FOCUS_OUT", app.pipe().focus_out())
        .env("XPLR_PIPE_MODE_OUT", app.pipe().mode_out())
        .env("XPLR_PIPE_RESULT_OUT", app.pipe().result_out())
        .env(
            "XPLR_PIPE_GLOBAL_HELP_MENU_OUT",
            app.pipe().global_help_menu_out(),
        )
        .env(
            "XPLR_PIPE_DIRECTORY_NODES_OUT",
            app.pipe().directory_nodes_out(),
        )
        .env("XPLR_PIPE_LOGS_OUT", app.pipe().logs_out())
        .stdin(stdin)
        .stdout(stdout)
        .stderr(stderr)
        .args(cmd.args())
        .status()
}
