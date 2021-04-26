#![allow(clippy::too_many_arguments)]

use std::env;
use std::path::PathBuf;
use xplr::app;

use zellij_tile::prelude::*;

#[derive(Default)]
struct State;

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {}

    fn update(&mut self, event: Event) {}

    fn render(&mut self, rows: usize, cols: usize) {
        let mut pwd = PathBuf::from(env::args().nth(1).unwrap_or_else(|| ".".into()))
            .canonicalize()
            .unwrap_or_default();
        let mut focused_path = None;

        if pwd.is_file() {
            focused_path = Some(
                pwd.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
            );
            pwd = pwd.parent().map(|p| p.into()).unwrap_or_default();
        }

        let app = app::App::create(pwd).unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            std::process::exit(1);
        });

        println!("foo");
    }
}
