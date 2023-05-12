use std::{os::fd::RawFd, process::Command};

use nix::{
    pty::{forkpty, Winsize},
    unistd::ForkResult,
};

// holds grid and later on will hold the cursor position
// Also will hold psuedo terminal
use crate::{
    display::display_ansi_text,
    layout::{Grid, Layout},
};

#[derive(Debug)]
pub struct Terminal {
    pub visible_grid: Grid,
    pub width: u32,
    pub height: u32,
    pub layout: Layout,
    pub stdout_fd: RawFd,
}

impl Terminal {
    pub fn new(layout: Layout) -> Self {
        let (width, height) = layout.calculate();

        let mut visible_grid = Grid::new(width, height);

        let default_shell = std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string());

        let stdout_fd = Self::spawn_pty_with_shell(default_shell, &layout, width, height);

        let mut read_buffer = vec![];

        loop {
            match Self::read_fd(stdout_fd) {
                Some(mut x) => read_buffer.append(&mut x),
                None => {
                    let string = String::from_utf8(read_buffer).expect("Invalid UTF-8");
                    display_ansi_text(&mut visible_grid, string);
                    break;
                }
            }
        }

        Self {
            visible_grid,
            width,
            height,
            layout,
            stdout_fd,
        }
    }

    fn spawn_pty_with_shell(shell: String, layout: &Layout, width: u32, height: u32) -> RawFd {
        let winsize = Winsize {
            ws_row: width as u16,
            ws_col: height as u16,
            ws_xpixel: layout.px_width as u16,
            ws_ypixel: layout.px_height as u16,
        };

        unsafe {
            match forkpty(Some(&winsize), None) {
                Ok(x) => {
                    let stout_fd = x.master;

                    if let ForkResult::Child = x.fork_result {
                        Command::new(&shell).spawn().expect("Failed to spawn shell");
                        std::thread::sleep(std::time::Duration::from_millis(200));
                        std::process::exit(0);
                    }
                    stout_fd
                }
                Err(e) => panic!("Error: {}", e),
            }
        }
    }

    fn read_fd(fd: RawFd) -> Option<Vec<u8>> {
        let mut buf = [0; 65536];
        let read_result = nix::unistd::read(fd, &mut buf);
        match read_result {
            Ok(bytes_read) => Some(buf[..bytes_read].to_vec()),
            Err(_) => None,
        }
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.layout.px_height = size.height as f32;
        self.layout.px_width = size.width as f32;

        (self.width, self.height) = self.layout.calculate();
        println!("height: {}, width: {}", self.height, self.width);
        self.visible_grid.resize(self.width, self.height);
    }
}
