extern crate libc;

pub struct TermState(libc::termios);

pub fn get_term_dims() -> (usize, usize)
{
    let mut size = libc::winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0
    };
    unsafe {
        libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &size);
    }
    (size.ws_row as usize, size.ws_col as usize)
}

pub fn gettermstate() -> TermState
{
    let mut term = libc::termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0
    };
    unsafe {
        libc::tcgetattr(libc::STDOUT_FILENO, &mut term as *mut libc::termios);
    }
    TermState(term)
}

pub fn settermstate(&TermState(ref term): &TermState) -> ()
{
    unsafe {
        libc::tcsetattr(libc::STDOUT_FILENO, libc::TCSANOW, term as *const libc::termios);
    }
}

pub fn setraw()
{
    let TermState(mut term) = gettermstate();
    term.c_iflag &= !(libc::IGNBRK | libc::BRKINT | libc::PARMRK | libc::ISTRIP | libc::INLCR | libc::ICRNL | libc::ICRNL);
    term.c_lflag &= !(libc::ECHO | libc::ECHONL | libc::ICANON | libc::ISIG | libc::IEXTEN);
    term.c_cflag &= !(libc::CSIZE | libc::PARENB); 
    term.c_cflag |= libc::CS8;
    settermstate(&TermState(term));
}
