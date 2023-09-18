use crossterm::{style::{Color, Stylize}, terminal};

pub struct Terminal {
    width: u16,
}

impl Terminal {
    pub fn new() -> Self {
        let (width, _) = terminal::size().unwrap();
        Self { width }
    }

    pub fn pprint<'a>(
        &self,
        left: &mut impl Iterator<Item = &'a str>,
        right: &mut impl Iterator<Item = String>,
        longest_left: usize,
        primary: Color,
    ) {
        let half_width = (self.width / 2) as usize;
        let mut to_print = String::new();
        while let Some(left) = left.next() {
            let len_spacer = half_width.saturating_sub(left.len());
            let beg_spacer_len = half_width.saturating_sub(longest_left) / 2;
            let beg_spacer = " ".repeat(beg_spacer_len);
            let ending_spacer = " ".repeat(len_spacer.saturating_sub(beg_spacer_len));
            let default_string = String::from("");
            let info_line = right.next().unwrap_or(default_string);
            to_print.push_str(&format!(
                "{}{}{}{}\n",
                beg_spacer,
                left.with(primary),
                ending_spacer,
                info_line
            ));
        }

        let spacer = " ".repeat(half_width);
        while let Some(right) = right.next() {
            to_print.push_str(&format!("{}{}\n", spacer, right));
        }

        print!("{}", to_print);
    }
}
