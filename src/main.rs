mod sysoverview;
mod terminal;
mod theme;

use sysoverview::SysOverview;
use terminal::Terminal;
use theme::{DefaultTheme, Theme};

fn main() {
    let theme = Theme::create_default_theme(DefaultTheme::Alone);
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
