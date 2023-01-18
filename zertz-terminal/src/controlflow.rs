#[macro_export]
macro_rules! terminal_loop {
    ($($stmt: stmt)+) => {
        #[allow(unused_labels)]
        'main: loop {
            #[allow(unused_macros)]
            macro_rules! unwrap {
                ($expr: expr) => {
                    match $expr {
                        Ok(val) => val,
                        Err(err) => {
                            eprintln!("{:?}\r", err);
                            break 'main;
                        }
                    }
                };
            }
            #[allow(unused_macros)]
            macro_rules! unwrap_continue {
                ($expr: expr) => {
                    match $expr {
                        Ok(val) => val,
                        Err(err) => {
                            eprintln!("{:?}\r", err);
                            continue 'main;
                        }
                    }
                };
            }
            #[allow(unused_macros)]
            macro_rules! unwrap_except {
                ($expr: expr, $pat: pat) => {
                    match $expr {
                        Ok(val) => val,
                        Err($pat) => continue 'main,
                        Err(err) => {
                            eprintln!("{:?}\r", err);
                            break 'main;
                        }
                    }
                };
                ($expr: expr, $pat: pat,) => {
                    match $expr {
                        Ok(val) => val,
                        Err($pat) => continue 'main,
                        Err(err) => {
                            eprintln!("{:?}\r", err);
                            break 'main;
                        }
                    }
                };
            }
            #[allow(unused_macros)]
            macro_rules! jmp {
                ("continue") => { continue 'main };
                ("break") => { break 'main };
            }

            $($stmt)+
        }
    };
}

#[macro_export]
macro_rules! controlflow {
    ($tag: lifetime: $($stmt: stmt)+) => {
        #[allow(unused_labels)]
        $tag: {
            #[allow(unused_macros)]
            macro_rules! unwrap {
                ($expr: expr) => {
                    match $expr {
                        Ok(val) => val,
                        Err(_) => {
                            break $tag;
                        }
                    }
                };
            }
            #[allow(unused_macros)]
            macro_rules! unwrap_yell {
                ($expr: expr) => {
                    match $expr {
                        Ok(val) => val,
                        Err(err) => {
                            eprintln!("{:?}\r", err);
                            break $tag;
                        }
                    }
                };
            }

            $($stmt)+
        }
    };
}
