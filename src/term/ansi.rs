use std::io::IoResult;

pub trait Ansi : Writer {
    fn cursor_up(&mut self, n: uint) -> IoResult<()> {
        write!(self, "\x1B[{}A", n)
    }

    fn cursor_down(&mut self, n: uint) -> IoResult<()> {
        write!(self, "\x1B[{}B", n)
    }

    fn cursor_forward(&mut self, n: uint) -> IoResult<()> {
        write!(self, "\x1B[{}C", n)
    }

    fn cursor_back(&mut self, n: uint) -> IoResult<()> {
        write!(self, "\x1B[{}D", n)
    }

    fn cursor_next_line(&mut self, n: uint) -> IoResult<()> {
        write!(self, "\x1B[{}E", n)
    }

    fn cursor_previous_line(&mut self, n: uint) -> IoResult<()> {
        write!(self, "\x1B[{}F", n)
    }

    fn cursor_horizontal_absolute(&mut self, n: uint) -> IoResult<()> {
        write!(self, "\x1B[{}G", n)
    }

    fn cursor_position(&mut self, n: uint, m: uint) -> IoResult<()> {
        write!(self, "\x1B[{};{}H", n, m)
    }

    fn erase_display(&mut self, n: uint) -> IoResult<()> {
        write!(self, "\x1B[{}J", n)
    }

    fn erase_line(&mut self, n: uint) -> IoResult<()> {
        write!(self, "\x1B[{}K", n)
    }

    fn scroll_up(&mut self, n: uint) -> IoResult<()> {
        write!(self, "\x1B[{}S", n)
    }

    fn scroll_down(&mut self, n: uint) -> IoResult<()> {
        write!(self, "\x1B[{}T", n)
    }

    fn horizontal_vertical_position(&mut self, n: uint, m: uint) -> IoResult<()> {
        write!(self, "\x1B[{};{}f", n, m)
    }

    fn select_graphic_rendition(&mut self, ns: &[uint]) -> IoResult<()> {
        // FIXME: We pretty much know how long this is going to be.
        let mut s = String::new();
        for n in ns.iter() {
            // FIXME: Terribly inefficient.
            s.push_str(n.to_string().as_slice());
            s.push_str(";");
        }
        write!(self, "\x1B[{}m", s)
    }

    fn device_status_report(&mut self) -> IoResult<()> {
        write!(self, "\x1B[6n")
    }

    fn save_cursor_position(&mut self) -> IoResult<()> {
        write!(self, "\x1B[s")
    }

    fn restore_cursor_position(&mut self) -> IoResult<()> {
        write!(self, "\x1B[u")
    }

    fn hide_cursor(&mut self) -> IoResult<()> {
        write!(self, "\x1B[?25l")
    }

    fn show_cursor(&mut self) -> IoResult<()> {
        write!(self, "\x1B[?25h")
    }
}

