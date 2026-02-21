#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Coloring {
    Colorful,
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Spacing {
    Full,
    Compact,
}

// Very similar to Miette
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Theme {
    pub coloring: Coloring,
    pub spacing: Spacing,

    pub h_bar: char,
    pub v_bar: char,
    pub x_bar: char,
    pub h_dots_bar: char,
    pub v_dots_bar: char,
    pub u_arrow: char,
    pub d_arrow: char,
    pub l_arrow: char,
    pub r_arrow: char,
    pub ul_bend: char,
    pub dl_bend: char,
    pub ur_bend: char,
    pub dr_bend: char,
    pub u_conn: char,
    pub d_conn: char,
    pub l_conn: char,
    pub r_conn: char,
}

impl Theme {
    pub const fn round(coloring: Coloring, spacing: Spacing) -> Self {
        Self {
            coloring,
            spacing,
            h_bar: '─',
            v_bar: '│',
            x_bar: '┼',
            h_dots_bar: '⋯',
            v_dots_bar: '⋮',
            u_arrow: '▲',
            d_arrow: '▼',
            l_arrow: '◀',
            r_arrow: '▶',
            ul_bend: '╭',
            dl_bend: '╰',
            ur_bend: '╮',
            dr_bend: '╯',
            u_conn: '┬',
            d_conn: '┴',
            l_conn: '├',
            r_conn: '┤',
        }
    }

    pub const fn sharp(coloring: Coloring, spacing: Spacing) -> Self {
        Self {
            coloring,
            spacing,
            h_bar: '─',
            v_bar: '│',
            x_bar: '┼',
            h_dots_bar: '┉',
            v_dots_bar: '┆',
            u_arrow: '^',
            d_arrow: 'v', // or '⌄'?
            l_arrow: '<',
            r_arrow: '>',
            ul_bend: '┌',
            dl_bend: '└',
            ur_bend: '┐',
            dr_bend: '┘',
            u_conn: '┬',
            d_conn: '┴',
            l_conn: '├',
            r_conn: '┤',
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::round(Coloring::Colorful, Spacing::Full)
    }
}
