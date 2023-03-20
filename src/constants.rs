pub const NORMAL_MODE_MOVEMENT: &[(&str, &[&str], &str)] = &[
    ("normal-mode/movement/h.gif", &["h"], "Move left"),
    ("normal-mode/movement/j.gif", &["j"], "Move down"),
    ("normal-mode/movement/k.gif", &["k"], "Move up"),
    ("normal-mode/movement/l.gif", &["l"], "Move right"),
    ("normal-mode/movement/w.gif", &["w"], "Move next word start"),
    (
        "normal-mode/movement/b.gif",
        &["b"],
        "Move previous word start",
    ),
    ("normal-mode/movement/e.gif", &["e"], "Move next word end"),
    ("normal-mode/movement/W.gif", &["W"], "Move next WORD start"),
    (
        "normal-mode/movement/B.gif",
        &["B"],
        "Move previous WORD start",
    ),
    ("normal-mode/movement/E.gif", &["E"], "Move next WORD end"),
    (
        "normal-mode/movement/t-l.gif",
        &["t", "l"],
        "Find 'till next 'l'",
    ),
    ("normal-mode/movement/f-l.gif", &["f", "l"], "Find next 'l'"),
    (
        "normal-mode/movement/T-l.gif",
        &["T", "l"],
        "Find 'till previous 'l'",
    ),
    (
        "normal-mode/movement/F-l.gif",
        &["F", "l"],
        "Find previous 'l'",
    ),
    (
        "normal-mode/movement/13-G.gif",
        &["1", "3", "G"],
        "Go to line number 13",
    ),
    (
        "normal-mode/movement/alt-dot.gif",
        &["Alt", "."],
        "Repeat last motion (f, t or m)",
    ),
    (
        "normal-mode/movement/ctrl-b.gif",
        &["Control", "b"],
        "Move page up",
    ),
    (
        "normal-mode/movement/ctrl-f.gif",
        &["Control", "f"],
        "Move page down",
    ),
    (
        "normal-mode/movement/ctrl-u.gif",
        &["Control", "u"],
        "Move half page up",
    ),
    (
        "normal-mode/movement/ctrl-d.gif",
        &["Control", "d"],
        "Move half page down",
    ),
    (
        "normal-mode/movement/ctrl-i.gif",
        &["Control", "i"],
        "Jump forward on the jumplist",
    ),
    (
        "normal-mode/movement/ctrl-o.gif",
        &["Control", "o"],
        "Jump backward on the jumplist",
    ),
    (
        "normal-mode/movement/ctrl-s.gif",
        &["Control", "s"],
        "Save the current selection to the jumplist",
    ),
];

pub const COMMANDS: &[(&str, &str)] = &[
    ("Esc", "Reset your current input"),
    ("Enter", "Go to next shortcut after success"),
    ("Left arrow", "Skip backward"),
    ("Right arrow", "Skip forward"),
    ("Ctrl + R", "Repeat from start"),
];
