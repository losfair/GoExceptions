enum State {
    Normal,
    Comment,
    Try,
    CatchArgName,
    CatchArgType,
    CatchBody
}

macro_rules! may_be_identifier_char {
    ($c: expr) => {
        if ($c >= 'a' && $c <= 'z') || ($c >= 'A' && $c <= 'Z') || ($c >= '0' && $c <= '9') || $c == '_' {
            true
        } else {
            false
        }
    }
}

pub fn transpile(input: &String) -> String {
    let mut state = State::Normal;
    let mut may_be_comment_begin = false;
    let mut may_be_comment_end = false;
    let mut expecting_catch = false;
    let mut prev_not_identifier_char = false;
    let mut catch_arg_type_ended = false;
    let mut block_depth = 0;
    let mut kw_start_pos = 0;

    let mut kw = String::new();
    let mut try_block = String::new();
    let mut catch_arg_name = String::new();
    let mut catch_arg_type = String::new();
    let mut catch_block = String::new();
    let mut output = String::new();

    for ch in input.chars() {
        match state {
            State::Normal => {
                let mut no_push = false;

                match may_be_comment_begin {
                    true => {
                        if ch == '*' {
                            state = State::Comment;
                        }
                        may_be_comment_begin = false;
                    },
                    false => {
                        if ch == '/' {
                            may_be_comment_begin = true;
                        }
                    }
                }
                if !may_be_identifier_char!(ch) {
                    prev_not_identifier_char = true;

                    let mut should_match = false;
                    if ch == '{' || expecting_catch {
                        should_match = true;
                    }
                    match should_match {
                        true => {
                            match kw.trim() {
                                "try" => {
                                    state = State::Try;
                                    block_depth = 1;
                                    try_block.clear();
                                    output = output[0..kw_start_pos].to_string();
                                    kw.clear();
                                    no_push = true;
                                },
                                "catch" => {
                                    if !expecting_catch {
                                        panic!("Unexpected catch");
                                    }
                                    expecting_catch = false;
                                    state = State::CatchArgName;

                                    catch_arg_name.clear();

                                    block_depth = 1;
                                    output = output[0..kw_start_pos].to_string();
                                    kw.clear();
                                    no_push = true;
                                },
                                _ => {}
                            }
                        },
                        false => {}
                    }
                } else {
                    if prev_not_identifier_char {
                        prev_not_identifier_char = false;
                        kw.clear();
                        kw_start_pos = output.len();
                    }
                    if !no_push {
                        kw.push(ch);
                    }
                }

                if !no_push {
                    output.push(ch);
                }
            },
            State::Comment => {
                match may_be_comment_end {
                    true => {
                        if ch == '/' {
                            state = State::Normal;
                        }
                        may_be_comment_end = false;
                    },
                    false => {
                        if ch == '*' {
                            may_be_comment_end = true;
                        }
                    }
                }
                output.push(ch);
            },
            State::Try => {
                //println!("In try {}", ch);
                match ch {
                    '{' => block_depth += 1,
                    '}' => block_depth -= 1,
                    _ => {}
                }
                if block_depth == 0 {
                    state = State::Normal;
                    expecting_catch = true;
                } else {
                    try_block.push(ch);
                }
            },
            State::CatchArgName => {
                if may_be_identifier_char!(ch) {
                    catch_arg_name.push(ch);
                } else {
                    catch_arg_type.clear();
                    state = State::CatchArgType;
                    catch_arg_type_ended = false;
                }
            },
            State::CatchArgType => {
                match catch_arg_type_ended {
                    true => {
                        if ch == '{' {
                            state = State::CatchBody;
                            catch_block.clear();
                            block_depth = 1;
                        }
                    },
                    false => {
                        if may_be_identifier_char!(ch) {
                            catch_arg_type.push(ch);
                        } else {
                            catch_arg_type_ended = true;
                        }
                    }
                }
            },
            State::CatchBody => {
                match ch {
                    '{' => block_depth += 1,
                    '}' => block_depth -= 1,
                    _ => {}
                }
                if block_depth == 0 {
                    state = State::Normal;

                    output.push_str((r#"
                        func() {
                            defer func() {
                                if err := recover(); err != nil {
                                    switch err.(type) {
                                        case "#.to_string() + catch_arg_type.as_str() + r#":
                                            func("# + catch_arg_name.as_str() + " " + catch_arg_type.as_str() + r#") {
                                                "# + catch_block.as_str() + r#"
                                            }(err.("# + catch_arg_type.as_str() + r#"))
                                        default:
                                            panic(err)
                                    }
                                }
                            }()
                            "# + try_block.as_str() + r#"
                        }()
                    "#).as_str());
                } else {
                    catch_block.push(ch);
                }
            }
        }
    }

    output
}
