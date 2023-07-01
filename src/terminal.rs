use std::{
    ffi::{CStr, CString},
    os::fd::AsRawFd,
    sync::mpsc::Sender,
};

use nix::{
    libc::{ioctl, TIOCSCTTY},
    poll::{PollFd, PollFlags},
    pty::{grantpt, posix_openpt, ptsname, unlockpt},
    sys::{
        select::{select, FdSet},
        termios::{cfmakeraw, tcsetattr, SetArg},
    },
    unistd::{close, dup, execvp, fork, read, setsid},
};

// holds grid and later on will hold the cursor position
// Also will hold psuedo terminal
use crate::layout::{Grid, Layout};

#[derive(Debug)]
pub struct Terminal {
    pub visible_grid: Grid,
    pub width: u32,
    pub height: u32,
    pub cursor: (u32, u32),
    pub visible_cursor: bool,
    pub layout: Layout,
    pub reciever: std::sync::mpsc::Receiver<String>,
    pub transmitter: std::sync::mpsc::Sender<String>,
}

impl Terminal {
    pub fn new(layout: Layout) -> Self {
        let (width, height) = layout.calculate();

        let visible_grid = Grid::new(width, height);

        let default_shell = std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string());

        let (tx, reciever) = std::sync::mpsc::channel();

        let transmitter = Self::spawn_pty_with_shell(default_shell, &layout, width, height, tx);

        let term = Self {
            visible_grid,
            width,
            height,
            layout,
            cursor: (0, 0),
            visible_cursor: true,
            reciever,
            transmitter,
        };

        term
    }

    fn spawn_pty_with_shell(
        shell: String,
        _layout: &Layout,
        _width: u32,
        _height: u32,
        transmitter: Sender<String>,
    ) -> Sender<String> {
        let fdm = posix_openpt(nix::fcntl::OFlag::O_RDWR).unwrap();

        grantpt(&fdm).unwrap();
        unlockpt(&fdm).unwrap();

        let pts_name = unsafe { ptsname(&fdm) }.unwrap();

        let fds = nix::fcntl::open(
            &std::path::PathBuf::from(pts_name),
            nix::fcntl::OFlag::O_RDWR,
            nix::sys::stat::Mode::empty(),
        )
        .unwrap();

        let (tx, reciever) = std::sync::mpsc::channel::<String>();

        std::thread::spawn(move || match unsafe { fork() } {
            Ok(res) => {
                if res.is_parent() {
                    nix::unistd::close(fds).unwrap();

                    loop {
                        if let Ok(x) = reciever.try_recv() {
                            nix::unistd::write(fdm.as_raw_fd(), x.as_bytes()).unwrap();
                        }
                        let pollfd = PollFd::new(fdm.as_raw_fd(), PollFlags::POLLIN);

                        let rc = nix::poll::poll(&mut [pollfd], 10).unwrap();
                        // let rc = 1;

                        if rc > 0 {
                            let mut input = [0u8; 65536];
                            let rc = read(fdm.as_raw_fd(), &mut input).unwrap();
                            if rc > 0 {
                                let s = String::from_utf8(input[..rc].to_vec()).unwrap();

                                match transmitter.send(s) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        panic!("send failed: {}", e);
                                    }
                                };
                            }
                        }
                    }
                } else {
                    close(fdm.as_raw_fd()).unwrap();

                    let slave_orig_settings = nix::sys::termios::tcgetattr(fds).unwrap();
                    let mut new_term_settings = slave_orig_settings;
                    cfmakeraw(&mut new_term_settings);
                    tcsetattr(fds, SetArg::TCSANOW, &new_term_settings).unwrap();

                    close(0).unwrap();
                    close(1).unwrap();
                    close(2).unwrap();

                    dup(fds).unwrap();
                    dup(fds).unwrap();
                    dup(fds).unwrap();

                    close(fds).unwrap();

                    setsid().unwrap();

                    unsafe {
                        ioctl(0, TIOCSCTTY, 1);
                    };

                    {
                        let cmd_arr = shell
                            .split_whitespace()
                            .map(|s| CString::new(s).unwrap())
                            .collect::<Vec<CString>>();

                        let cmd_arr = cmd_arr.iter().map(|s| s.as_c_str()).collect::<Vec<&CStr>>();

                        let _rc = execvp(cmd_arr[0], &cmd_arr);
                    }
                }
            }
            Err(e) => {
                panic!("fork failed: {}", e);
            }
        });

        tx
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.layout.px_height = size.height as f32;
        self.layout.px_width = size.width as f32;

        (self.width, self.height) = self.layout.calculate();
        println!("height: {}, width: {}", self.height, self.width);
        self.visible_grid.resize(self.width, self.height);
    }

    pub fn key_pressed(&mut self, key: &winit::event::VirtualKeyCode) {
        match key {
            k => {
                self.transmitter.send("pwd\n".to_string()).unwrap();
                println!("sent pwd");
            }
            _ => {}
        }
    }
}

