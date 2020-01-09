extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const BOOLEAN: &'static [&'static str] = &[
	"auto_left_margin", "auto_right_margin", "no_esc_ctlc", "ceol_standout_glitch",
	"eat_newline_glitch", "erase_overstrike", "generic_type", "hard_copy", "has_meta_key",
	"has_status_line", "insert_null_glitch", "memory_above", "memory_below", "move_insert_mode",
	"move_standout_mode", "over_strike", "status_line_esc_ok", "dest_tabs_magic_smso",
	"tilde_glitch", "transparent_underline", "xon_xoff", "needs_xon_xoff", "prtr_silent",
	"hard_cursor", "non_rev_rmcup", "no_pad_char", "non_dest_scroll_region", "can_change",
	"back_color_erase", "hue_lightness_saturation", "col_addr_glitch", "cr_cancels_micro_mode",
	"has_print_wheel", "row_addr_glitch", "semi_auto_right_margin", "cpi_changes_res",
	"lpi_changes_res", "backspaces_with_bs", "crt_no_scrolling", "no_correctly_working_cr",
	"gnu_has_meta_key", "linefeed_is_newline", "has_hardware_tabs", "return_does_clr_eol"
];

const NUMBER: &'static [&'static str] = &[
	"columns", "init_tabs", "lines", "lines_of_memory", "magic_cookie_glitch", "padding_baud_rate",
	"virtual_terminal", "width_status_line", "num_labels", "label_height", "label_width",
	"max_attributes", "maximum_windows", "max_colors", "max_pairs", "no_color_video",
	"buffer_capacity", "dot_vert_spacing", "dot_horz_spacing", "max_micro_address",
	"max_micro_jump", "micro_col_size", "micro_line_size", "number_of_pins", "output_res_char",
	"output_res_line", "output_res_horz_inch", "output_res_vert_inch", "print_rate",
	"wide_char_size", "buttons", "bit_image_entwining", "bit_image_type", "magic_cookie_glitch_ul",
	"carriage_return_delay", "new_line_delay", "backspace_delay", "horizontal_tab_delay",
	"number_of_function_keys"
];

const STRING: &'static [&'static str] = &[
	"back_tab", "bell", "carriage_return", "change_scroll_region", "clear_all_tabs",
	"clear_screen", "clr_eol", "clr_eos", "column_address", "command_character", "cursor_address",
	"cursor_down", "cursor_home", "cursor_invisible", "cursor_left", "cursor_mem_address",
	"cursor_normal", "cursor_right", "cursor_to_ll", "cursor_up", "cursor_visible",
	"delete_character", "delete_line", "dis_status_line", "down_half_line",
	"enter_alt_charset_mode", "enter_blink_mode", "enter_bold_mode", "enter_ca_mode",
	"enter_delete_mode", "enter_dim_mode", "enter_insert_mode", "enter_secure_mode",
	"enter_protected_mode", "enter_reverse_mode", "enter_standout_mode", "enter_underline_mode",
	"erase_chars", "exit_alt_charset_mode", "exit_attribute_mode", "exit_ca_mode",
	"exit_delete_mode", "exit_insert_mode", "exit_standout_mode", "exit_underline_mode",
	"flash_screen", "form_feed", "from_status_line", "init_1string", "init_2string",
	"init_3string", "init_file", "insert_character", "insert_line", "insert_padding",
	"key_backspace", "key_catab", "key_clear", "key_ctab", "key_dc", "key_dl", "key_down",
	"key_eic", "key_eol", "key_eos", "key_f0", "key_f1", "key_f10", "key_f2", "key_f3", "key_f4",
	"key_f5", "key_f6", "key_f7", "key_f8", "key_f9", "key_home", "key_ic", "key_il", "key_left",
	"key_ll", "key_npage", "key_ppage", "key_right", "key_sf", "key_sr", "key_stab", "key_up",
	"keypad_local", "keypad_xmit", "lab_f0", "lab_f1", "lab_f10", "lab_f2", "lab_f3", "lab_f4",
	"lab_f5", "lab_f6", "lab_f7", "lab_f8", "lab_f9", "meta_off", "meta_on", "newline", "pad_char",
	"parm_dch", "parm_delete_line", "parm_down_cursor", "parm_ich", "parm_index",
	"parm_insert_line", "parm_left_cursor", "parm_right_cursor", "parm_rindex", "parm_up_cursor",
	"pkey_key", "pkey_local", "pkey_xmit", "print_screen", "prtr_off", "prtr_on", "repeat_char",
	"reset_1string", "reset_2string", "reset_3string", "reset_file", "restore_cursor",
	"row_address", "save_cursor", "scroll_forward", "scroll_reverse", "set_attributes", "set_tab",
	"set_window", "tab", "to_status_line", "underline_char", "up_half_line", "init_prog", "key_a1",
	"key_a3", "key_b2", "key_c1", "key_c3", "prtr_non", "char_padding", "acs_chars", "plab_norm",
	"key_btab", "enter_xon_mode", "exit_xon_mode", "enter_am_mode", "exit_am_mode",
	"xon_character", "xoff_character", "ena_acs", "label_on", "label_off", "key_beg", "key_cancel",
	"key_close", "key_command", "key_copy", "key_create", "key_end", "key_enter", "key_exit",
	"key_find", "key_help", "key_mark", "key_message", "key_move", "key_next", "key_open",
	"key_options", "key_previous", "key_print", "key_redo", "key_reference", "key_refresh",
	"key_replace", "key_restart", "key_resume", "key_save", "key_suspend", "key_undo", "key_sbeg",
	"key_scancel", "key_scommand", "key_scopy", "key_screate", "key_sdc", "key_sdl", "key_select",
	"key_send", "key_seol", "key_sexit", "key_sfind", "key_shelp", "key_shome", "key_sic",
	"key_sleft", "key_smessage", "key_smove", "key_snext", "key_soptions", "key_sprevious",
	"key_sprint", "key_sredo", "key_sreplace", "key_sright", "key_srsume", "key_ssave",
	"key_ssuspend", "key_sundo", "req_for_input", "key_f11", "key_f12", "key_f13", "key_f14",
	"key_f15", "key_f16", "key_f17", "key_f18", "key_f19", "key_f20", "key_f21", "key_f22",
	"key_f23", "key_f24", "key_f25", "key_f26", "key_f27", "key_f28", "key_f29", "key_f30",
	"key_f31", "key_f32", "key_f33", "key_f34", "key_f35", "key_f36", "key_f37", "key_f38",
	"key_f39", "key_f40", "key_f41", "key_f42", "key_f43", "key_f44", "key_f45", "key_f46",
	"key_f47", "key_f48", "key_f49", "key_f50", "key_f51", "key_f52", "key_f53", "key_f54",
	"key_f55", "key_f56", "key_f57", "key_f58", "key_f59", "key_f60", "key_f61", "key_f62",
	"key_f63", "clr_bol", "clear_margins", "set_left_margin", "set_right_margin", "label_format",
	"set_clock", "display_clock", "remove_clock", "create_window", "goto_window", "hangup",
	"dial_phone", "quick_dial", "tone", "pulse", "flash_hook", "fixed_pause", "wait_tone", "user0",
	"user1", "user2", "user3", "user4", "user5", "user6", "user7", "user8", "user9", "orig_pair",
	"orig_colors", "initialize_color", "initialize_pair", "set_color_pair", "set_foreground",
	"set_background", "change_char_pitch", "change_line_pitch", "change_res_horz",
	"change_res_vert", "define_char", "enter_doublewide_mode", "enter_draft_quality",
	"enter_italics_mode", "enter_leftward_mode", "enter_micro_mode", "enter_near_letter_quality",
	"enter_normal_quality", "enter_shadow_mode", "enter_subscript_mode", "enter_superscript_mode",
	"enter_upward_mode", "exit_doublewide_mode", "exit_italics_mode", "exit_leftward_mode",
	"exit_micro_mode", "exit_shadow_mode", "exit_subscript_mode", "exit_superscript_mode",
	"exit_upward_mode", "micro_column_address", "micro_down", "micro_left", "micro_right",
	"micro_row_address", "micro_up", "order_of_pins", "parm_down_micro", "parm_left_micro",
	"parm_right_micro", "parm_up_micro", "select_char_set", "set_bottom_margin",
	"set_bottom_margin_parm", "set_left_margin_parm", "set_right_margin_parm", "set_top_margin",
	"set_top_margin_parm", "start_bit_image", "start_char_set_def", "stop_bit_image",
	"stop_char_set_def", "subscript_characters", "superscript_characters", "these_cause_cr",
	"zero_motion", "char_set_names", "key_mouse", "mouse_info", "req_mouse_pos", "get_mouse",
	"set_a_foreground", "set_a_background", "pkey_plab", "device_type", "code_set_init",
	"set0_des_seq", "set1_des_seq", "set2_des_seq", "set3_des_seq", "set_lr_margin",
	"set_tb_margin", "bit_image_repeat", "bit_image_newline", "bit_image_carriage_return",
	"color_names", "define_bit_image_region", "end_bit_image_region", "set_color_band",
	"set_page_length", "display_pc_char", "enter_pc_charset_mode", "exit_pc_charset_mode",
	"enter_scancode_mode", "exit_scancode_mode", "pc_term_options", "scancode_escape",
	"alt_scancode_esc", "enter_horizontal_hl_mode", "enter_left_hl_mode", "enter_low_hl_mode",
	"enter_right_hl_mode", "enter_top_hl_mode", "enter_vertical_hl_mode", "set_a_attributes",
	"set_pglen_inch", "termcap_init2", "termcap_reset", "linefeed_if_not_lf",
	"backspace_if_not_bs", "other_non_function_keys", "arrow_key_map", "acs_ulcorner",
	"acs_llcorner", "acs_urcorner", "acs_lrcorner", "acs_ltee", "acs_rtee", "acs_btee", "acs_ttee",
	"acs_hline", "acs_vline", "acs_plus", "memory_lock", "memory_unlock", "box_chars_1"
];

const TERMINFO: &'static [(&'static str, &'static str)] = &[
	// Boolean names.
	("auto_left_margin", "bw"),
	("auto_right_margin", "am"),
	("back_color_erase", "bce"),
	("can_change", "ccc"),
	("ceol_standout_glitch", "xhp"),
	("col_addr_glitch", "xhpa"),
	("cpi_changes_res", "cpix"),
	("cr_cancels_micro_mode", "crxm"),
	("dest_tabs_magic_smso", "xt"),
	("eat_newline_glitch", "xenl"),
	("erase_overstrike", "eo"),
	("generic_type", "gn"),
	("hard_copy", "hc"),
	("hard_cursor", "chts"),
	("has_meta_key", "km"),
	("has_print_wheel", "daisy"),
	("has_status_line", "hs"),
	("hue_lightness_saturation", "hls"),
	("insert_null_glitch", "in"),
	("lpi_changes_res", "lpix"),
	("memory_above", "da"),
	("memory_below", "db"),
	("move_insert_mode", "mir"),
	("move_standout_mode", "msgr"),
	("needs_xon_xoff", "nxon"),
	("no_esc_ctlc", "xsb"),
	("no_pad_char", "npc"),
	("non_dest_scroll_region", "ndscr"),
	("non_rev_rmcup", "nrrmc"),
	("over_strike", "os"),
	("prtr_silent", "mc5i"),
	("row_addr_glitch", "xvpa"),
	("semi_auto_right_margin", "sam"),
	("status_line_esc_ok", "eslok"),
	("tilde_glitch", "hz"),
	("transparent_underline", "ul"),
	("xon_xoff", "xon"),

	// Number names.
	("bit_image_entwining", "bitwin"),
	("bit_image_type", "bitype"),
	("buffer_capacity", "bufsz"),
	("buttons", "btns"),
	("columns", "cols"),
	("dot_horz_spacing", "spinh"),
	("dot_vert_spacing", "spinv"),
	("init_tabs", "it"),
	("label_height", "lh"),
	("label_width", "lw"),
	("lines", "lines"),
	("lines_of_memory", "lm"),
	("max_attributes", "ma"),
	("magic_cookie_glitch", "xmc"),
	("max_colors", "colors"),
	("max_micro_address", "maddr"),
	("max_micro_jump", "mjump"),
	("max_pairs", "pairs"),
	("maximum_windows", "wnum"),
	("micro_col_size", "mcs"),
	("micro_line_size", "mls"),
	("no_color_video", "ncv"),
	("num_labels", "nlab"),
	("number_of_pins", "npins"),
	("output_res_char", "orc"),
	("output_res_line", "orl"),
	("output_res_horz_inch", "orhi"),
	("output_res_vert_inch", "orvi"),
	("padding_baud_rate", "pb"),
	("print_rate", "cps"),
	("virtual_terminal", "vt"),
	("wide_char_size", "widcs"),
	("width_status_line", "wsl"),

	// String names.
	("acs_chars", "acsc"),
	("alt_scancode_esc", "scesa"),
	("back_tab", "cbt"),
	("bell", "bel"),
	("bit_image_carriage_return", "bicr"),
	("bit_image_newline", "binel"),
	("bit_image_repeat", "birep"),
	("carriage_return", "cr"),
	("change_char_pitch", "cpi"),
	("change_line_pitch", "lpi"),
	("change_res_horz", "chr"),
	("change_res_vert", "cvr"),
	("change_scroll_region", "csr"),
	("char_padding", "rmp"),
	("char_set_names", "csnm"),
	("clear_all_tabs", "tbc"),
	("clear_margins", "mgc"),
	("clear_screen", "clear"),
	("clr_bol", "el1"),
	("clr_eol", "el"),
	("clr_eos", "ed"),
	("code_set_init", "csin"),
	("color_names", "colornm"),
	("column_address", "hpa"),
	("command_character", "cmdch"),
	("create_window", "cwin"),
	("cursor_address", "cup"),
	("cursor_down", "cud1"),
	("cursor_home", "home"),
	("cursor_invisible", "civis"),
	("cursor_left", "cub1"),
	("cursor_mem_address", "mrcup"),
	("cursor_normal", "cnorm"),
	("cursor_right", "cuf1"),
	("cursor_to_ll", "ll"),
	("cursor_up", "cuu1"),
	("cursor_visible", "cvvis"),
	("define_bit_image_region", "defbi"),
	("define_char", "defc"),
	("delete_character", "dch1"),
	("delete_line", "dl1"),
	("device_type", "devt"),
	("dial_phone", "dial"),
	("dis_status_line", "dsl"),
	("display_clock", "dclk"),
	("display_pc_char", "dispc"),
	("down_half_line", "hd"),
	("ena_acs", "enacs"),
	("end_bit_image_region", "endbi"),
	("enter_alt_charset_mode", "smacs"),
	("enter_am_mode", "smam"),
	("enter_blink_mode", "blink"),
	("enter_bold_mode", "bold"),
	("enter_ca_mode", "smcup"),
	("enter_delete_mode", "smdc"),
	("enter_dim_mode", "dim"),
	("enter_doublewide_mode", "swidm"),
	("enter_draft_quality", "sdrfq"),
	("enter_horizontal_hl_mode", "ehhlm"),
	("enter_insert_mode", "smir"),
	("enter_italics_mode", "sitm"),
	("enter_left_hl_mode", "elhlm"),
	("enter_leftward_mode", "slm"),
	("enter_low_hl_mode", "elohlm"),
	("enter_micro_mode", "smicm"),
	("enter_near_letter_quality", "snlq"),
	("enter_normal_quality", "snrmq"),
	("enter_pc_charset_mode", "smpch"),
	("enter_protected_mode", "prot"),
	("enter_reverse_mode", "rev"),
	("enter_right_hl_mode", "erhlm"),
	("enter_scancode_mode", "smsc"),
	("enter_secure_mode", "invis"),
	("enter_shadow_mode", "sshm"),
	("enter_standout_mode", "smso"),
	("enter_subscript_mode", "ssubm"),
	("enter_superscript_mode", "ssupm"),
	("enter_top_hl_mode", "ethlm"),
	("enter_underline_mode", "smul"),
	("enter_upward_mode", "sum"),
	("enter_vertical_hl_mode", "evhlm"),
	("enter_xon_mode", "smxon"),
	("erase_chars", "ech"),
	("exit_alt_charset_mode", "rmacs"),
	("exit_am_mode", "rmam"),
	("exit_attribute_mode", "sgr0"),
	("exit_ca_mode", "rmcup"),
	("exit_delete_mode", "rmdc"),
	("exit_doublewide_mode", "rwidm"),
	("exit_insert_mode", "rmir"),
	("exit_italics_mode", "ritm"),
	("exit_leftward_mode", "rlm"),
	("exit_micro_mode", "rmicm"),
	("exit_pc_charset_mode", "rmpch"),
	("exit_scancode_mode", "rmsc"),
	("exit_shadow_mode", "rshm"),
	("exit_standout_mode", "rmso"),
	("exit_subscript_mode", "rsubm"),
	("exit_superscript_mode", "rsupm"),
	("exit_underline_mode", "rmul"),
	("exit_upward_mode", "rum"),
	("exit_xon_mode", "rmxon"),
	("fixed_pause", "pause"),
	("flash_hook", "hook"),
	("flash_screen", "flash"),
	("form_feed", "ff"),
	("from_status_line", "fsl"),
	("get_mouse", "getm"),
	("goto_window", "wingo"),
	("hangup", "hup"),
	("init_1string", "is1"),
	("init_2string", "is2"),
	("init_3string", "is3"),
	("init_file", "if"),
	("init_prog", "iprog"),
	("initialize_color", "initc"),
	("initialize_pair", "initp"),
	("insert_character", "ich1"),
	("insert_line", "il1"),
	("insert_padding", "ip"),

	("key_a1", "ka1"),
	("key_a3", "ka3"),
	("key_b2", "kb2"),
	("key_backspace", "kbs"),
	("key_beg", "kbeg"),
	("key_btab", "kcbt"),
	("key_c1", "kc1"),
	("key_c3", "kc3"),
	("key_cancel", "kcan"),
	("key_catab", "ktbc"),
	("key_clear", "kclr"),
	("key_close", "kclo"),
	("key_command", "kcmd"),
	("key_copy", "kcpy"),
	("key_create", "kcrt"),
	("key_ctab", "kctab"),
	("key_dc", "kdch1"),
	("key_dl", "kdl1"),
	("key_down", "kcud1"),
	("key_eic", "krmir"),
	("key_end", "kend"),
	("key_enter", "kent"),
	("key_eol", "kel"),
	("key_eos", "ked"),
	("key_exit", "kext"),
	("key_f0", "kf0"),
	("key_f1", "kf1"),
	("key_f62", "kf62"),
	("key_f63", "kf63"),
	("key_find", "kfnd"),
	("key_help", "khlp"),
	("key_home", "khome"),
	("key_ic", "kich1"),
	("key_il", "kil1"),
	("key_left", "kcub1"),
	("key_ll", "kll"),
	("key_mark", "kmrk"),
	("key_message", "kmsg"),
	("key_mouse", "kmous"),
	("key_move", "kmov"),
	("key_next", "knxt"),
	("key_npage", "knp"),
	("key_open", "kopn"),
	("key_options", "kopt"),
	("key_ppage", "kpp"),
	("key_previous", "kprv"),
	("key_print", "kprt"),
	("key_redo", "krdo"),
	("key_reference", "kref"),
	("key_refresh", "krfr"),
	("key_replace", "krpl"),
	("key_restart", "krst"),
	("key_resume", "kres"),
	("key_right", "kcuf1"),
	("key_save", "ksav"),
	("key_sbeg", "kBEG"),
	("key_scancel", "kCAN"),
	("key_scommand", "kCMD"),
	("key_scopy", "kCPY"),
	("key_screate", "kCRT"),
	("key_sdc", "kDC"),
	("key_sdl", "kDL"),
	("key_select", "kslt"),
	("key_send", "kEND"),
	("key_seol", "kEOL"),
	("key_sexit", "kEXT"),
	("key_sf", "kind"),
	("key_sfind", "kFND"),
	("key_shelp", "kHLP"),
	("key_shome", "kHOM"),
	("key_sic", "kIC"),
	("key_sleft", "kLFT"),
	("key_smessage", "kMSG"),
	("key_smove", "kMOV"),
	("key_snext", "kNXT"),
	("key_soptions", "kOPT"),
	("key_sprevious", "kPRV"),
	("key_sprint", "kPRT"),
	("key_sr", "kri"),
	("key_sredo", "kRDO"),
	("key_sreplace", "kRPL"),
	("key_sright", "kRIT"),
	("key_srsume", "kRES"),
	("key_ssave", "kSAV"),
	("key_ssuspend", "kSPD"),
	("key_stab", "khts"),
	("key_sundo", "kUND"),
	("key_suspend", "kspd"),
	("key_undo", "kund"),
	("key_up", "kcuu1"),
	("keypad_local", "rmkx"),
	("keypad_xmit", "smkx"),
	("lab_f0", "lf0"),
	("lab_f1", "lf1"),
	("lab_f2", "lf2"),
	("lab_f3", "lf3"),
	("lab_f4", "lf4"),
	("lab_f5", "lf5"),
	("lab_f6", "lf6"),
	("lab_f7", "lf7"),
	("lab_f8", "lf8"),
	("lab_f9", "lf9"),
	("lab_f10", "lf10"),
	("label_format", "fln"),
	("label_off", "rmln"),
	("label_on", "smln"),
	("meta_off", "rmm"),
	("meta_on", "smm"),
	("micro_column_address", "mhpa"),
	("micro_down", "mcud1"),
	("micro_left", "mcub1"),
	("micro_right", "mcuf1"),
	("micro_row_address", "mvpa"),
	("micro_up", "mcuu1"),
	("mouse_info", "minfo"),
	("newline", "nel"),
	("order_of_pins", "porder"),
	("orig_colors", "oc"),
	("orig_pair", "op"),
	("pad_char", "pad"),
	("parm_dch", "dch"),
	("parm_delete_line", "dl"),
	("parm_down_cursor", "cud"),
	("parm_down_micro", "mcud"),
	("parm_ich", "ich"),
	("parm_index", "indn"),
	("parm_insert_line", "il"),
	("parm_left_cursor", "cub"),
	("parm_left_micro", "mcub"),
	("parm_right_cursor", "cuf"),
	("parm_right_micro", "mcuf"),
	("parm_rindex", "rin"),
	("parm_up_cursor", "cuu"),
	("parm_up_micro", "mcuu"),
	("pc_term_options", "pctrm"),
	("pkey_key", "pfkey"),
	("pkey_local", "pfloc"),
	("pkey_plab", "pfxl"),
	("pkey_xmit", "pfx"),
	("plab_norm", "pln"),
	("print_screen", "mc0"),
	("prtr_non", "mc5p"),
	("prtr_off", "mc4"),
	("prtr_on", "mc5"),
	("pulse", "pulse"),
	("quick_dial", "qdial"),
	("remove_clock", "rmclk"),
	("repeat_char", "rep"),
	("req_for_input", "rfi"),
	("req_mouse_pos", "reqmp"),
	("reset_1string", "rs1"),
	("reset_2string", "rs2"),
	("reset_3string", "rs3"),
	("reset_file", "rf"),
	("restore_cursor", "rc"),
	("row_address", "vpa"),
	("save_cursor", "sc"),
	("scancode_escape", "scesc"),
	("scroll_forward", "ind"),
	("scroll_reverse", "ri"),
	("select_char_set", "scs"),
	("set0_des_seq", "s0ds"),
	("set1_des_seq", "s1ds"),
	("set2_des_seq", "s2ds"),
	("set3_des_seq", "s3ds"),
	("set_a_attributes", "sgr1"),
	("set_a_background", "setab"),
	("set_a_foreground", "setaf"),
	("set_attributes", "sgr"),
	("set_background", "setb"),
	("set_bottom_margin", "smgb"),
	("set_bottom_margin_parm", "smgbp"),
	("set_clock", "sclk"),
	("set_color_band", "setcolor"),
	("set_color_pair", "scp"),
	("set_foreground", "setf"),
	("set_left_margin", "smgl"),
	("set_left_margin_parm", "smglp"),
	("set_lr_margin", "smglr"),
	("set_page_length", "slines"),
	("set_pglen_inch", "slength"),
	("set_right_margin", "smgr"),
	("set_right_margin_parm", "smgrp"),
	("set_tab", "hts"),
	("set_tb_margin", "smgtb"),
	("set_top_margin", "smgt"),
	("set_top_margin_parm", "smgtp"),
	("set_window", "wind"),
	("start_bit_image", "sbim"),
	("start_char_set_def", "scsd"),
	("stop_bit_image", "rbim"),
	("stop_char_set_def", "rcsd"),
	("subscript_characters", "subcs"),
	("superscript_characters", "supcs"),
	("tab", "ht"),
	("these_cause_cr", "docr"),
	("to_status_line", "tsl"),
	("tone", "tone"),
	("user0", "u0"),
	("user1", "u1"),
	("user2", "u2"),
	("user3", "u3"),
	("user4", "u4"),
	("user5", "u5"),
	("user6", "u6"),
	("user7", "u7"),
	("user8", "u8"),
	("user9", "u9"),
	("underline_char", "uc"),
	("up_half_line", "hu"),
	("wait_tone", "wait"),
	("xoff_character", "xoffc"),
	("xon_character", "xonc"),
	("zero_motion", "zerom"),
];

const TERMCAP: &'static [(&'static str, &'static str)] = &[
	// Boolean names.
	("auto_left_margin", "bw"),
	("auto_right_margin", "am"),
	("back_color_erase", "ut"),
	("can_change", "cc"),
	("ceol_standout_glitch", "xs"),
	("col_addr_glitch", "YA"),
	("cpi_changes_res", "YF"),
	("cr_cancels_micro_mode", "YB"),
	("dest_tabs_magic_smso", "xt"),
	("eat_newline_glitch", "xn"),
	("erase_overstrike", "eo"),
	("generic_type", "gn"),
	("hard_copy", "hc"),
	("hard_cursor", "HC"),
	("has_meta_key", "km"),
	("has_print_wheel", "YC"),
	("has_status_line", "hs"),
	("hue_lightness_saturation", "hl"),
	("insert_null_glitch", "in"),
	("lpi_changes_res", "YG"),
	("memory_above", "da"),
	("memory_below", "db"),
	("move_insert_mode", "mi"),
	("move_standout_mode", "ms"),
	("needs_xon_xoff", "nx"),
	("no_esc_ctlc", "xb"),
	("no_pad_char", "NP"),
	("non_dest_scroll_region", "ND"),
	("non_rev_rmcup", "NR"),
	("over_strike", "os"),
	("prtr_silent", "5i"),
	("row_addr_glitch", "YD"),
	("semi_auto_right_margin", "YE"),
	("status_line_esc_ok", "es"),
	("tilde_glitch", "hz"),
	("transparent_underline", "ul"),
	("xon_xoff", "xo"),

	// Number names.
	("bit_image_entwining", "Yo"),
	("bit_image_type", "Yp"),
	("buffer_capacity", "Ya"),
	("buttons", "BT"),
	("columns", "co"),
	("dot_horz_spacing", "Yc"),
	("dot_vert_spacing", "Yb"),
	("init_tabs", "it"),
	("label_height", "lh"),
	("label_width", "lw"),
	("lines", "li"),
	("lines_of_memory", "lm"),
	("max_attributes", "ma"),
	("magic_cookie_glitch", "sg"),
	("max_colors", "Co"),
	("max_micro_address", "Yd"),
	("max_micro_jump", "Ye"),
	("max_pairs", "pa"),
	("maximum_windows", "MW"),
	("micro_col_size", "Yf"),
	("micro_line_size", "Yg"),
	("no_color_video", "NC"),
	("num_labels", "Nl"),
	("number_of_pins", "Yh"),
	("output_res_char", "Yi"),
	("output_res_line", "Yj"),
	("output_res_horz_inch", "Yk"),
	("output_res_vert_inch", "Yl"),
	("padding_baud_rate", "pb"),
	("print_rate", "Ym"),
	("virtual_terminal", "vt"),
	("wide_char_size", "Yn"),
	("width_status_line", "ws"),

	// String names.
	("acs_chars", "ac"),
	("alt_scancode_esc", "S8"),
	("back_tab", "bt"),
	("bell", "bl"),
	("bit_image_carriage_return", "Yv"),
	("bit_image_newline", "Zz"),
	("bit_image_repeat", "Xy"),
	("carriage_return", "cr"),
	("change_char_pitch", "ZA"),
	("change_line_pitch", "ZB"),
	("change_res_horz", "ZC"),
	("change_res_vert", "ZD"),
	("change_scroll_region", "cs"),
	("char_padding", "rP"),
	("char_set_names", "Zy"),
	("clear_all_tabs", "ct"),
	("clear_margins", "MC"),
	("clear_screen", "cl"),
	("clr_bol", "cb"),
	("clr_eol", "ce"),
	("clr_eos", "cd"),
	("code_set_init", "ci"),
	("color_names", "Yw"),
	("column_address", "ch"),
	("command_character", "CC"),
	("cursor_address", "cm"),
	("cursor_down", "do"),
	("cursor_home", "ho"),
	("cursor_invisible", "vi"),
	("cursor_left", "le"),
	("cursor_mem_address", "CM"),
	("cursor_normal", "ve"),
	("cursor_right", "nd"),
	("cursor_to_ll", "ll"),
	("cursor_up", "up"),
	("cursor_visible", "vs"),
	("define_bit_image_region", "Yx"),
	("define_char", "ZE"),
	("delete_character", "dc"),
	("delete_line", "dl"),
	("device_type", "dv"),
	("dial_phone", "DI"),
	("dis_status_line", "ds"),
	("display_clock", "DK"),
	("display_pc_char", "S1"),
	("down_half_line", "hd"),
	("ena_acs", "eA"),
	("end_bit_image_region", "Yy"),
	("enter_alt_charset_mode", "as"),
	("enter_am_mode", "SA"),
	("enter_blink_mode", "mb"),
	("enter_bold_mode", "md"),
	("enter_ca_mode", "ti"),
	("enter_delete_mode", "dm"),
	("enter_dim_mode", "mh"),
	("enter_doublewide_mode", "ZF"),
	("enter_draft_quality", "ZG"),
	("enter_insert_mode", "im"),
	("enter_italics_mode", "ZH"),
	("enter_leftward_mode", "ZI"),
	("enter_micro_mode", "ZJ"),
	("enter_near_letter_quality", "ZK"),
	("enter_normal_quality", "ZL"),
	("enter_pc_charset_mode", "S2"),
	("enter_protected_mode", "mp"),
	("enter_reverse_mode", "mr"),
	("enter_scancode_mode", "S4"),
	("enter_secure_mode", "mk"),
	("enter_shadow_mode", "ZM"),
	("enter_standout_mode", "so"),
	("enter_subscript_mode", "ZN"),
	("enter_superscript_mode", "ZO"),
	("enter_underline_mode", "us"),
	("enter_upward_mode", "ZP"),
	("enter_xon_mode", "SX"),
	("erase_chars", "ec"),
	("exit_alt_charset_mode", "ae"),
	("exit_am_mode", "RA"),
	("exit_attribute_mode", "me"),
	("exit_ca_mode", "te"),
	("exit_delete_mode", "ed"),
	("exit_doublewide_mode", "ZQ"),
	("exit_insert_mode", "ei"),
	("exit_italics_mode", "ZR"),
	("exit_leftward_mode", "ZS"),
	("exit_micro_mode", "ZT"),
	("exit_pc_charset_mode", "S3"),
	("exit_scancode_mode", "S5"),
	("exit_shadow_mode", "ZU"),
	("exit_standout_mode", "se"),
	("exit_subscript_mode", "ZV"),
	("exit_superscript_mode", "ZW"),
	("exit_underline_mode", "ue"),
	("exit_upward_mode", "ZX"),
	("exit_xon_mode", "RX"),
	("fixed_pause", "PA"),
	("flash_hook", "fh"),
	("flash_screen", "vb"),
	("form_feed", "ff"),
	("from_status_line", "fs"),
	("get_mouse", "Gm"),
	("goto_window", "WG"),
	("hangup", "HU"),
	("init_1string", "i1"),
	("init_2string", "is"),
	("init_3string", "i3"),
	("init_file", "if"),
	("init_prog", "iP"),
	("initialize_color", "Ic"),
	("initialize_pair", "Ip"),
	("insert_character", "ic"),
	("insert_line", "al"),
	("insert_padding", "ip"),

	("key_a1", "K1"),
	("key_a3", "K3"),
	("key_b2", "K2"),
	("key_backspace", "kb"),
	("key_btab", "kB"),
	("key_c1", "K4"),
	("key_c3", "K5"),
	("key_catab", "ka"),
	("key_clear", "kC"),
	("key_ctab", "kt"),
	("key_dc", "kD"),
	("key_dl", "kL"),
	("key_down", "kd"),
	("key_eic", "kM"),
	("key_eol", "kE"),
	("key_eos", "kS"),
	("key_f0", "k0"),
	("key_f1", "k1"),
	("key_f62", "Fq"),
	("key_f63", "Fr"),
	("key_help", "%1"),
	("key_home", "kh"),
	("key_ic", "kI"),
	("key_il", "kA"),
	("key_left", "kl"),
	("key_ll", "kH"),
	("key_mark", "%2"),
	("key_message", "%3"),
	("key_mouse", "Km"),
	("key_move", "%4"),
	("key_next", "%5"),
	("key_npage", "kN"),
	("key_open", "%6"),
	("key_options", "%7"),
	("key_ppage", "kP"),
	("key_previous", "%8"),
	("key_print", "%9"),
	("key_redo", "%0"),
	("key_reference", "&1"),
	("key_refresh", "&2"),
	("key_replace", "&3"),
	("key_restart", "&4"),
	("key_resume", "&5"),
	("key_right", "kr"),
	("key_save", "&6"),
	("key_sbeg", "&9"),
	("key_scancel", "&0"),
	("key_scommand", "*1"),
	("key_scopy", "*2"),
	("key_screate", "*3"),
	("key_sdc", "*4"),
	("key_sdl", "*5"),
	("key_select", "*6"),
	("key_send", "*7"),
	("key_seol", "*8"),
	("key_sexit", "*9"),
	("key_sf", "kF"),
	("key_sfind", "*0"),
	("key_shelp", "#1"),
	("key_shome", "#2"),
	("key_sic", "#3"),
	("key_sleft", "#4"),
	("key_smessage", "%a"),
	("key_smove", "%b"),
	("key_snext", "%c"),
	("key_soptions", "%d"),
	("key_sprevious", "%e"),
	("key_sprint", "%f"),
	("key_sr", "kR"),
	("key_sredo", "%g"),
	("key_sreplace", "%h"),
	("key_sright", "%i"),
	("key_srsume", "%j"),
	("key_ssave", "!1"),
	("key_ssuspend", "!2"),
	("key_stab", "kT"),
	("key_sundo", "!3"),
	("key_suspend", "&7"),
	("key_undo", "&8"),
	("key_up", "ku"),
	("keypad_local", "ke"),
	("keypad_xmit", "ks"),
	("lab_f0", "l0"),
	("lab_f1", "l1"),
	("lab_f2", "l2"),
	("lab_f3", "l3"),
	("lab_f4", "l4"),
	("lab_f5", "l5"),
	("lab_f6", "l6"),
	("lab_f7", "l7"),
	("lab_f8", "l8"),
	("lab_f9", "l9"),
	("lab_f10", "la"),
	("label_format", "Lf"),
	("label_off", "LF"),
	("label_on", "LO"),
	("meta_off", "mo"),
	("meta_on", "mm"),
	("micro_column_address", "ZY"),
	("micro_down", "ZZ"),
	("micro_left", "Za"),
	("micro_right", "Zb"),
	("micro_row_address", "Zc"),
	("micro_up", "Zd"),
	("mouse_info", "Mi"),
	("newline", "nw"),
	("order_of_pins", "Ze"),
	("orig_colors", "oc"),
	("orig_pair", "op"),
	("pad_char", "pc"),
	("parm_dch", "DC"),
	("parm_delete_line", "DL"),
	("parm_down_cursor", "DO"),
	("parm_down_micro", "Zf"),
	("parm_ich", "IC"),
	("parm_index", "SF"),
	("parm_insert_line", "AL"),
	("parm_left_cursor", "LE"),
	("parm_left_micro", "Zg"),
	("parm_right_cursor", "RI"),
	("parm_right_micro", "Zh"),
	("parm_rindex", "SR"),
	("parm_up_cursor", "UP"),
	("parm_up_micro", "Zi"),
	("pc_term_options", "S6"),
	("pkey_key", "pk"),
	("pkey_local", "pl"),
	("pkey_plab", "xl"),
	("pkey_xmit", "px"),
	("plab_norm", "pn"),
	("print_screen", "ps"),
	("prtr_non", "pO"),
	("prtr_off", "pf"),
	("prtr_on", "po"),
	("pulse", "PU"),
	("quick_dial", "QD"),
	("remove_clock", "RC"),
	("repeat_char", "rp"),
	("req_for_input", "RF"),
	("req_mouse_pos", "RQ"),
	("reset_1string", "r1"),
	("reset_2string", "r2"),
	("reset_3string", "r3"),
	("reset_file", "rf"),
	("restore_cursor", "rc"),
	("row_address", "cv"),
	("save_cursor", "sc"),
	("scancode_escape", "S7"),
	("scroll_forward", "sf"),
	("scroll_reverse", "sr"),
	("select_char_set", "Zj"),
	("set0_des_seq", "s0"),
	("set1_des_seq", "s1"),
	("set2_des_seq", "s2"),
	("set3_des_seq", "s3"),
	("set_a_background", "AB"),
	("set_a_foreground", "AF"),
	("set_attributes", "sa"),
	("set_background", "Sb"),
	("set_bottom_margin", "Zk"),
	("set_bottom_margin_parm", "Zl"),
	("set_clock", "SC"),
	("set_color_band", "Yz"),
	("set_color_pair", "sp"),
	("set_foreground", "Sf"),
	("set_left_margin", "ML"),
	("set_left_margin_parm", "Zm"),
	("set_page_length", "YZ"),
	("set_pglen_inch", "YI"),
	("set_right_margin", "MR"),
	("set_right_margin_parm", "Zn"),
	("set_tab", "st"),
	("set_tb_margin", "MT"),
	("set_top_margin", "Zo"),
	("set_top_margin_parm", "Zp"),
	("set_window", "wi"),
	("start_bit_image", "Zq"),
	("start_char_set_def", "Zr"),
	("stop_bit_image", "Zs"),
	("stop_char_set_def", "Zt"),
	("subscript_characters", "Zu"),
	("superscript_characters", "Zv"),
	("tab", "ta"),
	("these_cause_cr", "Zw"),
	("to_status_line", "ts"),
	("tone", "TO"),
	("user0", "u0"),
	("user1", "u1"),
	("user2", "u2"),
	("user3", "u3"),
	("user4", "u4"),
	("user5", "u5"),
	("user6", "u6"),
	("user7", "u7"),
	("user8", "u8"),
	("user9", "u9"),
	("underline_char", "uc"),
	("up_half_line", "hu"),
	("wait_tone", "WA"),
	("xoff_character", "XF"),
	("xon_character", "XN"),
	("zero_motion", "Zx"),
];

fn main() {
	let     path = Path::new(&env::var("OUT_DIR").unwrap()).join("names.rs");
	let mut file = BufWriter::new(File::create(&path).unwrap());

	write!(&mut file, "pub static BOOLEAN: ::phf::Map<u16, &'static str> = ").unwrap();
	let mut builder = phf_codegen::Map::new();
	for (index, name) in BOOLEAN.iter().enumerate() {
		builder.entry(index as u16, &format!("\"{}\"", name));
	}
	write!(&mut file, "{}", builder.build()).unwrap();
	write!(&mut file, ";\n").unwrap();

	let keys = BOOLEAN.iter().map(|n| format!("\"{}\"", n)).collect::<Vec<_>>();
	write!(&mut file, "pub static BOOLEAN_INDEX: ::phf::Map<&'static str, u16> = ").unwrap();
	let mut builder = phf_codegen::Map::<&str>::new();
	for (index, name) in keys.iter().enumerate() {
		builder.entry(name, &index.to_string());
	}
	write!(&mut file, "{}", builder.build()).unwrap();
	write!(&mut file, ";\n").unwrap();

	write!(&mut file, "pub static NUMBER: ::phf::Map<u16, &'static str> = ").unwrap();
	let mut builder = phf_codegen::Map::new();
	for (index, name) in NUMBER.iter().enumerate() {
		builder.entry(index as u16, &format!("\"{}\"", name));
	}
	write!(&mut file, "{}", builder.build()).unwrap();
	write!(&mut file, ";\n").unwrap();

	let keys = NUMBER.iter().map(|n| format!("\"{}\"", n)).collect::<Vec<_>>();
	write!(&mut file, "pub static NUMBER_INDEX: ::phf::Map<&'static str, u16> = ").unwrap();
	let mut builder = phf_codegen::Map::<&str>::new();
	for (index, name) in keys.iter().enumerate() {
		builder.entry(name, &index.to_string());
	}
	write!(&mut file, "{}", builder.build()).unwrap();
	write!(&mut file, ";\n").unwrap();

	write!(&mut file, "pub static STRING: ::phf::Map<u16, &'static str> = ").unwrap();
	let mut builder = phf_codegen::Map::new();
	for (index, name) in STRING.iter().enumerate() {
		builder.entry(index as u16, &format!("\"{}\"", name));
	}
	write!(&mut file, "{}", builder.build()).unwrap();
	write!(&mut file, ";\n").unwrap();

	let keys = STRING.iter().map(|n| format!("\"{}\"", n)).collect::<Vec<_>>();
	write!(&mut file, "pub static STRING_INDEX: ::phf::Map<&'static str, u16> = ").unwrap();
	let mut builder = phf_codegen::Map::<&str>::new();
	for (index, name) in keys.iter().enumerate() {
		builder.entry(name, &index.to_string());
	}
	write!(&mut file, "{}", builder.build()).unwrap();
	write!(&mut file, ";\n").unwrap();

	write!(&mut file, "pub static TERMINFO: ::phf::Map<&'static str, &'static str> = ").unwrap();
	let mut builder = phf_codegen::Map::new();
	for &(name, value) in TERMINFO {
		builder.entry(name, &format!("\"{}\"", value));
	}
	write!(&mut file, "{}", builder.build()).unwrap();
	write!(&mut file, ";\n").unwrap();

	write!(&mut file, "pub static TERMCAP: ::phf::Map<&'static str, &'static str> = ").unwrap();
	let mut builder = phf_codegen::Map::new();
	for &(name, value) in TERMCAP {
		builder.entry(name, &format!("\"{}\"", value));
	}
	write!(&mut file, "{}", builder.build()).unwrap();
	write!(&mut file, ";\n").unwrap();

	write!(&mut file, "pub static ALIASES: ::phf::Map<&'static str, &'static str> = ").unwrap();
	let mut builder = phf_codegen::Map::new();
	for &(value, name) in TERMINFO {
		builder.entry(name, &format!("\"{}\"", value));
	}
	for &(value, name) in TERMCAP {
		if TERMINFO.iter().find(|entry| name == entry.1).is_none() {
			builder.entry(name, &format!("\"{}\"", value));
		}
	}
	write!(&mut file, "{}", builder.build()).unwrap();
	write!(&mut file, ";\n").unwrap();
}
