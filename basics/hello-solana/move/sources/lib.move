module 0x10::debug {
  native public fun print<T>(x: &T);
}

module hello::hello {
    use 0x10::debug;
    use 0x1::string;

    public entry fun main() : u64 {
        let rv = 0;
        let s = string::utf8(b"Hello Solana");
        debug::print(&s);
        rv
    }
}
