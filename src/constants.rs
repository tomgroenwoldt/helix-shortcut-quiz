pub const NORMAL_MODE_MOVEMENT: &[(&str, &[&str], &str, &[&str])] = &[
    ("normal-mode/movement/h.gif", &["h"], "Move left", &[]),
    ("normal-mode/movement/j.gif", &["j"], "Move down", &[]),
    ("normal-mode/movement/k.gif", &["k"], "Move up", &[]),
    ("normal-mode/movement/l.gif", &["l"], "Move right", &[]),
    (
        "normal-mode/movement/w.gif",
        &["w"],
        "Move next word start",
        &[],
    ),
    (
        "normal-mode/movement/b.gif",
        &["b"],
        "Move previous word start",
        &[],
    ),
    (
        "normal-mode/movement/e.gif",
        &["e"],
        "Move next word end",
        &[],
    ),
    (
        "normal-mode/movement/W.gif",
        &["W"],
        "Move next WORD start",
        &[],
    ),
    (
        "normal-mode/movement/B.gif",
        &["B"],
        "Move previous WORD start",
        &[],
    ),
    (
        "normal-mode/movement/E.gif",
        &["E"],
        "Move next WORD end",
        &[],
    ),
    (
        "normal-mode/movement/t-l.gif",
        &["t", "l"],
        "Find 'till next 'l'",
        &[],
    ),
    (
        "normal-mode/movement/f-l.gif",
        &["f", "l"],
        "Find next 'l'",
        &[],
    ),
    (
        "normal-mode/movement/T-l.gif",
        &["T", "l"],
        "Find 'till previous 'l'",
        &[],
    ),
    (
        "normal-mode/movement/F-l.gif",
        &["F", "l"],
        "Find previous 'l'",
        &[],
    ),
    (
        "normal-mode/movement/13-G.gif",
        &["1", "3", "G"],
        "Go to line number 13",
        &[],
    ),
    (
        "normal-mode/movement/alt-dot.gif",
        &["."],
        "Repeat last motion (f, t or m)",
        &["Alt"],
    ),
    (
        "normal-mode/movement/ctrl-b.gif",
        &["b"],
        "Move page up",
        &["Control"],
    ),
    (
        "normal-mode/movement/ctrl-f.gif",
        &["f"],
        "Move page down",
        &["Control"],
    ),
    (
        "normal-mode/movement/ctrl-u.gif",
        &["u"],
        "Move half page up",
        &["Control"],
    ),
    (
        "normal-mode/movement/ctrl-d.gif",
        &["d"],
        "Move half page down",
        &["Control"],
    ),
    (
        "normal-mode/movement/ctrl-i.gif",
        &["i"],
        "Jump forward on the jumplist",
        &["Control"],
    ),
    (
        "normal-mode/movement/ctrl-o.gif",
        &["o"],
        "Jump backward on the jumplist",
        &["Control"],
    ),
    (
        "normal-mode/movement/ctrl-s.gif",
        &["s"],
        "Save the current selection to the jumplist",
        &["Control"],
    ),
];

pub const NORMAL_MODE_CHANGES: &[(&str, &[&str], &str, &[&str])] = &[
    (
        "normal-mode/changes/r.gif",
        &["r", "k"],
        "Replace with character 'k'",
        &[],
    ),
    (
        "normal-mode/changes/R.gif",
        &["R"],
        "Replace with yanked text",
        &[],
    ),
    (
        "normal-mode/changes/swung-dash.gif",
        &["~"],
        "Switch case of the selected text",
        &[],
    ),
    (
        "normal-mode/changes/`.gif",
        &["`"],
        "Set the selected text to lower case",
        &[],
    ),
    (
        "normal-mode/changes/alt-`.gif",
        &["`"],
        "Set the selected text to upper case",
        &["Alt"],
    ),
    (
        "normal-mode/changes/i.gif",
        &["i"],
        "Insert before selection",
        &[],
    ),
    (
        "normal-mode/changes/a.gif",
        &["a"],
        "Insert after selection",
        &[],
    ),
    (
        "normal-mode/changes/I.gif",
        &["I"],
        "Insert at the start of the line",
        &[],
    ),
    (
        "normal-mode/changes/A.gif",
        &["A"],
        "Insert at the end of the line",
        &[],
    ),
    (
        "normal-mode/changes/o.gif",
        &["o"],
        "Open new line below selection",
        &[],
    ),
    (
        "normal-mode/changes/O.gif",
        &["O"],
        "Open new line above selection",
        &[],
    ),
    (
        "normal-mode/changes/dot.gif",
        &["."],
        "Repeat last insert",
        &[],
    ),
    ("normal-mode/changes/u.gif", &["u"], "Undo change", &[]),
    ("normal-mode/changes/U.gif", &["U"], "Redo change", &[]),
    ("normal-mode/changes/y.gif", &["y"], "Yank selection", &[]),
    (
        "normal-mode/changes/p.gif",
        &["p"],
        "Paste after selection",
        &[],
    ),
    (
        "normal-mode/changes/P.gif",
        &["P"],
        "Paste before selection",
        &[],
    ),
    ("normal-mode/changes/>.gif", &[">"], "Indent selection", &[]),
    (
        "normal-mode/changes/<.gif",
        &["<"],
        "Unindent selection",
        &[],
    ),
    ("normal-mode/changes/d.gif", &["d"], "Delete selection", &[]),
    (
        "normal-mode/changes/c.gif",
        &["c"],
        "Change selection (delete and enter insert mode)",
        &[],
    ),
    (
        "normal-mode/changes/alt-c.gif",
        &["c"],
        "Change selection (delete and enter insert mode, without yanking)",
        &["Alt"],
    ),
    (
        "normal-mode/changes/ctrl-a.gif",
        &["a"],
        "Increment object (number) under cursor",
        &["Control"],
    ),
    (
        "normal-mode/changes/ctrl-x.gif",
        &["x"],
        "Decrement object (number) under cursor",
        &["Control"],
    ),
];

// TODO: Uncomment elements when `vhs` adds support for Alt + Shift + char combinations.
pub const NORMAL_MODE_SELECT: &[(&str, &[&str], &str, &[&str])] = &[
    (
        "normal-mode/select/s.gif",
        &["s", "p", "r", "i", "n", "t"],
        "Select all 'print' matches inside selection",
        &[],
    ),
    (
        "normal-mode/select/S.gif",
        &["S", "g", "u", "e", "s", "s"],
        "Split selection into sub selections on 'guess' matches",
        &[],
    ),
    (
        "normal-mode/select/&.gif",
        &["&"],
        "Align selection in columns",
        &[],
    ),
    (
        "normal-mode/select/underscore.gif",
        &["_"],
        "Trim whitespace from the selection",
        &[],
    ),
    (
        "normal-mode/select/semicolon.gif",
        &[";"],
        "Collapse selection onto a single cursor",
        &[],
    ),
    (
        "normal-mode/select/alt-semicolon.gif",
        &[";"],
        "Flip selection cursor and anchor",
        &["Alt"],
    ),
    (
        "normal-mode/select/alt-colon.gif",
        &[":"],
        "Ensures the selection is in forward direction",
        &["Alt"],
    ),
    (
        "normal-mode/select/comma.gif",
        &[","],
        "Keep only the primary selection",
        &[],
    ),
    (
        "normal-mode/select/alt-comma.gif",
        &[","],
        "Remove the primary selection",
        &["Alt"],
    ),
    (
        "normal-mode/select/C.gif",
        &["C"],
        "Copy selection onto the next line (Add cursor below)",
        &[],
    ),
    // (
    //     "normal-mode/select/alt-C.gif",
    //     &["C"],
    //     "Copy selection onto the previous line (Add cursor above)",
    //     &["Alt"],
    // ),
    (
        "normal-mode/select/opening-bracket.gif",
        &["("],
        "Rotate main selection backward",
        &[],
    ),
    (
        "normal-mode/select/closing-bracket.gif",
        &[")"],
        "Rotate main selection forward",
        &[],
    ),
    // (
    //     "normal-mode/select/alt-opening-bracket.gif",
    //     &["("],
    //     "Rotate selection content backward",
    //     &["Alt"],
    // ),
    // (
    //     "normal-mode/select/alt-closing-bracket.gif",
    //     &[")"],
    //     "Rotate selection content forward",
    //     &["Alt"],
    // ),
    (
        "normal-mode/select/percent.gif",
        &["%"],
        "Select entire file",
        &[],
    ),
    (
        "normal-mode/select/x.gif",
        &["x"],
        "Select current line, if already selected, extend to next line",
        &[],
    ),
    // (
    //     "normal-mode/select/X.gif",
    //     &["X"],
    //     "Extend selection to line bounds (line-wise selection)",
    //     &[],
    // ),
    // (
    //     "normal-mode/select/alt-x.gif",
    //     &["x"],
    //     "Shrink selection to line bounds (line-wise selection)",
    //     &["Alt"],
    // ),
    (
        "normal-mode/select/J.gif",
        &["J"],
        "Join lines inside selection",
        &[],
    ),
    // (
    //     "normal-mode/select/alt-J.gif",
    //     &["J"],
    //     "Join lines inside selection and select the inserted space",
    //     &["Alt"],
    // ),
    // (
    //     "normal-mode/select/K.gif",
    //     &["K"],
    //     "Keep selections matching the regex",
    //     &[],
    // ),
    // (
    //     "normal-mode/select/alt-K.gif",
    //     &["K"],
    //     "Remove selections matching the regex",
    //     &["Alt"],
    // ),
    (
        "normal-mode/select/ctrl-c.gif",
        &["c"],
        "Comment/uncomment selection",
        &["Control"],
    ),
];

pub const NORMAL_MODE_SEARCH: &[(&str, &[&str], &str, &[&str])] = &[
    (
        "normal-mode/search/slash.gif",
        &["/", "n", "e", "w"],
        "Search for 'new'",
        &[],
    ),
    (
        "normal-mode/search/question-mark.gif",
        &["?"],
        "Search for previous pattern",
        &[],
    ),
    (
        "normal-mode/search/n.gif",
        &["n"],
        "Select next search match",
        &[],
    ),
    (
        "normal-mode/search/N.gif",
        &["N"],
        "Select previous search match",
        &[],
    ),
    (
        "normal-mode/search/asterisk.gif",
        &["*"],
        "Use current selection as the search pattern",
        &[],
    ),
];

pub const NORMAL_MODE_WINDOW_MODE: &[(&str, &[&str], &str, &[&str])] = &[
    (
        "normal-mode/minor-modes/window-mode/w.gif",
        &["w"],
        "Switch to next window",
        &["Control", "w"],
    ),
    (
        "normal-mode/minor-modes/window-mode/v.gif",
        &["v"],
        "Vertical right split",
        &["Control", "w"],
    ),
    (
        "normal-mode/minor-modes/window-mode/s.gif",
        &["s"],
        "Horizontal bottom split",
        &["Control", "w"],
    ),
    (
        "normal-mode/minor-modes/window-mode/f.gif",
        &["f"],
        "Go to files in the selection in horizontal splits",
        &["Control", "w"],
    ),
    (
        "normal-mode/minor-modes/window-mode/F.gif",
        &["F"],
        "Go to files in the selection in vertical splits",
        &["Control", "w"],
    ),
    (
        "normal-mode/minor-modes/window-mode/h.gif",
        &["h"],
        "Move to left split",
        &["Control", "w"],
    ),
    (
        "normal-mode/minor-modes/window-mode/j.gif",
        &["j"],
        "Move to split below",
        &["Control", "w"],
    ),
    (
        "normal-mode/minor-modes/window-mode/k.gif",
        &["k"],
        "Move to split above",
        &["Control", "w"],
    ),
    (
        "normal-mode/minor-modes/window-mode/l.gif",
        &["l"],
        "Move to right split",
        &["Control", "w"],
    ),
    (
        "normal-mode/minor-modes/window-mode/q.gif",
        &["q"],
        "Close current window",
        &["Control", "w"],
    ),
    (
        "normal-mode/minor-modes/window-mode/o.gif",
        &["o"],
        "Only keep the current window, closing all the others",
        &["Control", "w"],
    ),
    (
        "normal-mode/minor-modes/window-mode/H.gif",
        &["H"],
        "Swap window to the left",
        &["Control", "w"],
    ),
    (
        "normal-mode/minor-modes/window-mode/J.gif",
        &["J"],
        "Swap window downwards",
        &["Control", "w"],
    ),
    (
        "normal-mode/minor-modes/window-mode/K.gif",
        &["K"],
        "Swap window upwards",
        &["Control", "w"],
    ),
    (
        "normal-mode/minor-modes/window-mode/L.gif",
        &["L"],
        "Swap window to the right",
        &["Control", "w"],
    ),
];

pub const NORMAL_MODE_MATCH_MODE: &[(&str, &[&str], &str, &[&str])] = &[
    (
        "normal-mode/minor-modes/match-mode/m.gif",
        &["m", "m"],
        "Goto matching bracket",
        &[],
    ),
    (
        "normal-mode/minor-modes/match-mode/s.gif",
        &["m", "s", "{"],
        "Surround current selection with '{'",
        &[],
    ),
    (
        "normal-mode/minor-modes/match-mode/r.gif",
        &["m", "r", "{", "("],
        "Replace surround character '{' with '('",
        &[],
    ),
    (
        "normal-mode/minor-modes/match-mode/d.gif",
        &["m", "d", "{"],
        "Delete surround character '{'",
        &[],
    ),
    (
        "normal-mode/minor-modes/match-mode/a.gif",
        &["m", "a", "f"],
        "Select around function",
        &[],
    ),
    (
        "normal-mode/minor-modes/match-mode/i.gif",
        &["m", "i", "W"],
        "Select inside WORD",
        &[],
    ),
];

pub const NORMAL_MODE_VIEW_MODE: &[(&str, &[&str], &str, &[&str])] = &[
    (
        "normal-mode/minor-modes/view-mode/z.gif",
        &["z", "z"],
        "Vertically center the line",
        &[],
    ),
    (
        "normal-mode/minor-modes/view-mode/t.gif",
        &["z", "t"],
        "Align the line to the top of the screen",
        &[],
    ),
    (
        "normal-mode/minor-modes/view-mode/b.gif",
        &["z", "b"],
        "Align the line to the bottom of the screen",
        &[],
    ),
    (
        "normal-mode/minor-modes/view-mode/m.gif",
        &["z", "m"],
        "Align the line to the middle of the screen (horizontally)",
        &[],
    ),
    (
        "normal-mode/minor-modes/view-mode/j.gif",
        &["z", "j"],
        "Scroll the view downwards",
        &[],
    ),
    (
        "normal-mode/minor-modes/view-mode/k.gif",
        &["z", "k"],
        "Scroll the view upwards",
        &[],
    ),
];

pub const NORMAL_MODE_GOTO_MODE: &[(&str, &[&str], &str, &[&str])] = &[
    (
        "normal-mode/minor-modes/goto-mode/g.gif",
        &["g", "g"],
        "Go to the start of the file",
        &[],
    ),
    (
        "normal-mode/minor-modes/goto-mode/25-g.gif",
        &["2", "5", "g", "g"],
        "Go to line number 25",
        &[],
    ),
    (
        "normal-mode/minor-modes/goto-mode/e.gif",
        &["g", "e"],
        "Go to the end of the file",
        &[],
    ),
    (
        "normal-mode/minor-modes/goto-mode/h.gif",
        &["g", "h"],
        "Go to the start of the line",
        &[],
    ),
    (
        "normal-mode/minor-modes/goto-mode/l.gif",
        &["g", "l"],
        "Go to the end of the line",
        &[],
    ),
    (
        "normal-mode/minor-modes/goto-mode/s.gif",
        &["g", "s"],
        "Go to the first non-whitespace character of the line",
        &[],
    ),
    (
        "normal-mode/minor-modes/goto-mode/t.gif",
        &["g", "t"],
        "Go to the top of the screen",
        &[],
    ),
    (
        "normal-mode/minor-modes/goto-mode/c.gif",
        &["g", "c"],
        "Go to the middle of the screen",
        &[],
    ),
    (
        "normal-mode/minor-modes/goto-mode/b.gif",
        &["g", "b"],
        "Go to the bottom of the screen",
        &[],
    ),
    (
        "normal-mode/minor-modes/goto-mode/a.gif",
        &["g", "a"],
        "Go to the last accessed/alternate file",
        &[],
    ),
    (
        "normal-mode/minor-modes/goto-mode/m.gif",
        &["g", "m"],
        "Go to the last modified/alternate file",
        &[],
    ),
    (
        "normal-mode/minor-modes/goto-mode/dot.gif",
        &["g", "."],
        "Go to last modification in current file",
        &[],
    ),
];

pub const PICKER: &[(&str, &[&str], &str, &[&str])] = &[
    ("picker/ctrl-p.gif", &["p"], "Previous entry", &["Control"]),
    ("picker/ctrl-n.gif", &["n"], "Next entry", &["Control"]),
    ("picker/ctrl-u.gif", &["u"], "Page up", &["Control"]),
    ("picker/ctrl-d.gif", &["d"], "Page down", &["Control"]),
    (
        "picker/ctrl-s.gif",
        &["s"],
        "Open horizontally",
        &["Control"],
    ),
    ("picker/ctrl-v.gif", &["v"], "Open vertically", &["Control"]),
    ("picker/ctrl-t.gif", &["t"], "Toggle preview", &["Control"]),
    ("picker/ctrl-c.gif", &["c"], "Close picker", &["Control"]),
];

pub const COMMANDS: &[(&str, &str)] = &[
    ("Esc", "Clear your current input"),
    ("Backspace", "Pop your current input"),
    ("Enter", "Go to next shortcut after success"),
    ("Left arrow", "Skip backward"),
    ("Right arrow", "Skip forward"),
];

pub const END_PLACEHOLDER: (&str, &[&str], &str, &[&str]) = (
    "placeholder/ferris-rust.gif",
    &[],
    "Congratulations! Thank you for playing!",
    &[],
);

pub const EMPTY_PLACEHOLDER: (&str, &[&str], &str, &[&str]) = (
    "placeholder/intense-stare-crabby-crab.gif",
    &[],
    "Choose a category!",
    &[],
);
