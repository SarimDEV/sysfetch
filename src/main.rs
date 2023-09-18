mod sysoverview;
mod terminal;
mod theme;

use sysoverview::SysOverview;
use terminal::Terminal;
use theme::{DefaultTheme, Theme};

fn main() {
    let theme = Theme::create_default_theme(DefaultTheme::Wavey);
    let primary = theme.get_primary();
    let secondary = theme.get_secondary();
    let theme_visual = theme.get_theme_visual();

    let system_overview = SysOverview::new();
    let mut output_strs = system_overview.output_strs(primary, secondary);
    output_strs.push(String::new());
    output_strs.push(theme_visual);

    // println!("{:?}", output_strs);

    let theme_art = theme.get_art();
    let mut theme_art_iter = theme_art.lines().into_iter();
    let term = Terminal::new();
    term.pprint(
        &mut theme_art_iter,
        &mut output_strs.into_iter(),
        theme.get_longest_line_len(),
        primary,
    );

    //
    // let half_width = (term.get_width() / 2) as usize;
    // let mut output_strs = output_strs.into_iter();
    //
    // let spaced_camel: String = theme_art
    //     .lines()
    //     .map(|line| {
    //         let len_spacer = half_width.saturating_sub(line.len());
    //         let beg_spacer_len = half_width.saturating_sub(theme.get_longest_line_len()) / 2;
    //         let beg_spacer = " ".repeat(beg_spacer_len);
    //         let ending_spacer = " ".repeat(len_spacer.saturating_sub(beg_spacer_len));
    //
    //         let default_string = String::from("");
    //         let info_line = output_strs.next().unwrap_or(default_string);
    //         // let line = line.to_string().with(theme.get_primary());
    //         let result = format!("{}{}{}{}\n", beg_spacer, line, ending_spacer, info_line);
    //         return result;
    //     })
    //     .collect();
    //
    // println!("{}", spaced_camel);
}

//
//
//    let str = "
//           /(_))
//         _/   /
//        //   /
//       //   /
//       /\\__/
//       \\O_/=-.
//   _  / || \\  ^o
//   \\\\/ ()_) \\.
//    ^^ <__> \\()
//      //||\\\\
//     //_||_\\\\  ds
//    // \\||/ \\\\
//   //   ||   \\\\
//  \\/    |/    \\/
//  /     |      \\
// /      |       \\
//        |
//        ";
//
