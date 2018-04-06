    use std::io;
    use std::io::BufRead;

    pub fn expect_i32() -> i32 {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let mut buf = String::new();

        handle.read_line(&mut buf).expect("Failed to read line");

        return buf.trim_right().parse().expect("Failed to parse i_32");
    }
