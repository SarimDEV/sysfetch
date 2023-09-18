use crossterm::style::{self, Stylize, Color};

pub struct Theme<'a> {
    art: &'a str,
    primary: Color,
    secondary: Color,
    longest_line_len: usize,
}

#[allow(dead_code)]
pub enum DefaultTheme {
    Wavey,
    Alone,
    Desert,
}

impl<'a> Theme<'a> {
    pub fn new(art: &'a str, primary: style::Color, secondary: style::Color) -> Theme<'a> {
        let longest_line_len = art.lines().map(|line| line.len()).max().unwrap_or(0);

        Self {
            art,
            primary,
            secondary,
            longest_line_len,
        }
    }
    pub fn get_art(&self) -> String {
        self.art.to_string()
    }

    pub fn get_primary(&self) -> style::Color {
        self.primary
    }

    pub fn get_secondary(&self) -> style::Color {
        self.secondary
    }

    pub fn get_longest_line_len(&self) -> usize {
        self.longest_line_len
    }

    pub fn get_theme_visual(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}",
            "███".with(Color::Black),
            "███".with(Color::Red),
            "███".with(Color::Green),
            "███".with(Color::Yellow),
            "███".with(Color::Blue),
            "███".with(Color::Magenta),
            "███".with(Color::Cyan),
            "███".with(Color::White),
        )
    }

    pub fn create_default_theme(theme: DefaultTheme) -> Self {
        match theme {
            DefaultTheme::Wavey => Theme::new(
                "                                  __
                               _.-~  )
                    _..--~~~~,'   ,-/     _
                 .-'. . . .'   ,-','    ,' )
               ,'. . . _   ,--~,-'__..-'  ,'
             ,'. . .  (@)' ---~~~~      ,'
            /. . . . '~~             ,-'
           /. . . . .             ,-'
          ; . . . .  - .        ,'
         : . . . .       _     /
        . . . . .          `-.:
       . . . ./  - .          )
      .  . . |  _____..---.._/ ____ Seal _
~---~~~~----~~~~             ~~",
                Color::Blue,
                Color::White,
            ),
            DefaultTheme::Alone => Theme::new(
                "     _                  _
    | '-.            .-' |
    | -. '..\\\\,.//,.' .- |
    |   \\  \\\\\\||///  /   |
   /|    )M\\/%%%%/\\/(  . |\\
  (/\\  MM\\/%/\\||/%\\\\/MM  /\\)
  (//M   \\%\\\\\\%%//%//   M\\\\)
(// M________ /\\ ________M \\\\)
 (// M\\ \\(',)|  |(',)/ /M \\\\) \\\\\\\\
  (\\\\ M\\.  /,\\\\//,\\  ./M //)
    / MMmm( \\\\||// )mmMM \\  \\\\
     // MMM\\\\\\||///MMM \\\\ \\\\
      \\//''\\)/||\\(/''\\\\/ \\\\
      mrf\\\\( \\oo/ )\\\\\\/\\
           \\'-..-'\\/\\\\
              \\\\/ \\\\
        ",
                Color::DarkRed,
                Color::White,
            ),
            DefaultTheme::Desert => Theme::new(
                " ___.-''''-.
/___  @    |
',,,,.     |         _.'''''''._
     '     |        /           \\
     |     \\    _.-'             \\
     |      '.-'                  '-.
     |                               ',
     |                                '',
      ',,-,                           ':;
           ',,| ;,,                 ,' ;;
              ! ; !'',,,',',,,,'!  ;   ;:
             : ;  ! !       ! ! ;  ;   :;
             ; ;   ! !      ! !  ; ;   ;,
            ; ;    ! !     ! !   ; ;
            ; ;    ! !    ! !     ; ;
           ;,,      !,!   !,!     ;,;
           /_I      L_I   L_I     /_I",
                Color::DarkYellow,
                Color::White,
            ),
        }
    }
}
