

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum TileKind {
    HORIZ_BAR,
    VERT_BAR,
    DOWN_RIGHT,
    DOWN_LEFT,
    UP_LEFT,
    UP_RIGHT,
    VERT_RIGHT,
    VERT_LEFT,
    HORIZ_DOWN,
    HORIZ_UP,
    CROSS,
    DOWN_LEFT_ROUND,
    DOWN_RIGHT_ROUND,
    UP_LEFT_ROUND,
    UP_RIGHT_ROUND,
}

//takes a unicode char, and out
fn char_to_TileKind(in_char: char) -> TileKind {
    return match in_char {
        '━' => TileKind::HORIZ_BAR,
        '┃' => TileKind::VERT_BAR,
        '┏' => TileKind::DOWN_RIGHT,
        '┓' => TileKind::DOWN_LEFT,
        '┛' => TileKind::UP_LEFT,
        '┗' => TileKind::UP_RIGHT,
        '┣' => TileKind::VERT_RIGHT,
        '┫' => TileKind::VERT_LEFT,
        '┳' => TileKind::HORIZ_DOWN,
        '┻' => TileKind::HORIZ_UP,
        '╋' => TileKind::CROSS,
        '╮' => TileKind::DOWN_LEFT_ROUND,
        '╭' => TileKind::DOWN_RIGHT_ROUND,
        '╯' => TileKind::UP_LEFT_ROUND,
        '╰' => TileKind::UP_RIGHT_ROUND,
        _ => panic!("BAD INPUT CHAR"),
    };
}

fn TileKind_to_char(kind: TileKind) -> char {
    return match kind {
        TileKind::HORIZ_BAR => '━',
        TileKind::VERT_BAR => '┃',
        TileKind::DOWN_RIGHT => '┏',
        TileKind::DOWN_LEFT => '┓',
        TileKind::UP_LEFT => '┛',
        TileKind::UP_RIGHT => '┗',
        TileKind::VERT_RIGHT => '┣',
        TileKind::VERT_LEFT => '┫',
        TileKind::HORIZ_DOWN => '┳',
        TileKind::HORIZ_UP => '┻',
        TileKind::CROSS => '╋',
        TileKind::DOWN_LEFT_ROUND => '╮',
        TileKind::DOWN_RIGHT_ROUND => '╭',
        TileKind::UP_LEFT_ROUND => '╯',
        TileKind::UP_RIGHT_ROUND => '╰',
    };
}
