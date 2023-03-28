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

pub const NORMAL_MODE_CHANGES: &[(&str, &[&str], &str)] = &[
    (
        "normal-mode/changes/r.gif",
        &["r", "k"],
        "Replace with character 'k'",
    ),
    (
        "normal-mode/changes/R.gif",
        &["R"],
        "Replace with yanked text",
    ),
    (
        "normal-mode/changes/swung-dash.gif",
        &["~"],
        "Switch case of the selected text",
    ),
    (
        "normal-mode/changes/`.gif",
        &["`"],
        "Set the selected text to lower case",
    ),
    (
        "normal-mode/changes/alt-`.gif",
        &["Alt", "`"],
        "Set the selected text to upper case",
    ),
    (
        "normal-mode/changes/i.gif",
        &["i"],
        "Insert before selection",
    ),
    (
        "normal-mode/changes/a.gif",
        &["a"],
        "Insert after selection",
    ),
    (
        "normal-mode/changes/I.gif",
        &["I"],
        "Insert at the start of the line",
    ),
    (
        "normal-mode/changes/A.gif",
        &["A"],
        "Insert at the end of the line",
    ),
    (
        "normal-mode/changes/o.gif",
        &["o"],
        "Open new line below selection",
    ),
    (
        "normal-mode/changes/O.gif",
        &["O"],
        "Open new line above selection",
    ),
    ("normal-mode/changes/dot.gif", &["."], "Repeat last insert"),
    ("normal-mode/changes/u.gif", &["u"], "Undo change"),
    ("normal-mode/changes/U.gif", &["U"], "Redo change"),
    ("normal-mode/changes/y.gif", &["y"], "Yank selection"),
    ("normal-mode/changes/p.gif", &["p"], "Paste after selection"),
    (
        "normal-mode/changes/P.gif",
        &["P"],
        "Paste before selection",
    ),
    ("normal-mode/changes/>.gif", &[">"], "Indent selection"),
    ("normal-mode/changes/<.gif", &["<"], "Unindent selection"),
    ("normal-mode/changes/d.gif", &["d"], "Delete selection"),
    (
        "normal-mode/changes/c.gif",
        &["c"],
        "Change selection (delete and enter insert mode)",
    ),
    (
        "normal-mode/changes/alt-c.gif",
        &["Alt", "c"],
        "Change selection (delete and enter insert mode, without yanking)",
    ),
    (
        "normal-mode/changes/ctrl-a.gif",
        &["Control", "a"],
        "Increment object (number) under cursor",
    ),
    (
        "normal-mode/changes/ctrl-x.gif",
        &["Control", "x"],
        "Decrement object (number) under cursor",
    ),
];

pub const NORMAL_MODE_SEARCH: &[(&str, &[&str], &str)] = &[
    (
        "normal-mode/search/slash.gif",
        &["/", "n", "e", "w"],
        "Search for 'new'",
    ),
    (
        "normal-mode/search/question-mark.gif",
        &["?"],
        "Search for previous pattern",
    ),
    (
        "normal-mode/search/n.gif",
        &["n"],
        "Select next search match",
    ),
    (
        "normal-mode/search/N.gif",
        &["N"],
        "Select previous search match",
    ),
    (
        "normal-mode/search/asterisk.gif",
        &["*"],
        "Use current selection as the search pattern",
    ),
];

pub const NORMAL_MODE_WINDOW_MODE: &[(&str, &[&str], &str)] = &[
    (
        "normal-mode/minor-modes/window-mode/w.gif",
        &["w"],
        "Switch to next window",
    ),
    (
        "normal-mode/minor-modes/window-mode/v.gif",
        &["v"],
        "Vertical right split",
    ),
    (
        "normal-mode/minor-modes/window-mode/s.gif",
        &["s"],
        "Horizontal bottom split",
    ),
    (
        "normal-mode/minor-modes/window-mode/f.gif",
        &["f"],
        "Go to files in the selection in horizontal splits",
    ),
    (
        "normal-mode/minor-modes/window-mode/F.gif",
        &["F"],
        "Go to files in the selection in vertical splits",
    ),
    (
        "normal-mode/minor-modes/window-mode/h.gif",
        &["h"],
        "Move to left split",
    ),
    (
        "normal-mode/minor-modes/window-mode/j.gif",
        &["j"],
        "Move to split below",
    ),
    (
        "normal-mode/minor-modes/window-mode/k.gif",
        &["k"],
        "Move to split above",
    ),
    (
        "normal-mode/minor-modes/window-mode/l.gif",
        &["l"],
        "Move to right split",
    ),
    (
        "normal-mode/minor-modes/window-mode/q.gif",
        &["q"],
        "Close current window",
    ),
    (
        "normal-mode/minor-modes/window-mode/o.gif",
        &["o"],
        "Only keep the current window, closing all the others",
    ),
    (
        "normal-mode/minor-modes/window-mode/H.gif",
        &["H"],
        "Swap window to the left",
    ),
    (
        "normal-mode/minor-modes/window-mode/J.gif",
        &["J"],
        "Swap window downwards",
    ),
    (
        "normal-mode/minor-modes/window-mode/K.gif",
        &["K"],
        "Swap window upwards",
    ),
    (
        "normal-mode/minor-modes/window-mode/L.gif",
        &["L"],
        "Swap window to the right",
    ),
];

pub const COMMANDS: &[(&str, &str)] = &[
    ("Esc", "Clear your current input"),
    ("Backspace", "Pop your current input"),
    ("Enter", "Go to next shortcut after success"),
    ("Left arrow", "Skip backward"),
    ("Right arrow", "Skip forward"),
];

pub const END_PLACEHOLDER: (&str, &[&str], &str) = (
    "placeholder/ferris-rust.gif",
    &[],
    "Congratulations! Thank you for playing!",
);

pub const EMPTY_PLACEHOLDER: (&str, &[&str], &str) = (
    "placeholder/intense-stare-crabby-crab.gif",
    &[],
    "Choose a category!",
);
