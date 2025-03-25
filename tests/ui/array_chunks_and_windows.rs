#![warn(clippy::array_chunks_and_windows)]

fn use_slice_windows_literal() {
    // test code goes here
    let slice = [1, 2, 3];
    for window in slice.windows(2) {
        println!("{} {}", window[0], window[1]);
    }
}

fn use_slice_windows_variable() {
    // test code goes here
    let slice = [1, 2, 3];
    let i = 2;
    for window in slice.windows(i) {
        println!("{} {}", window[0], window[1]);
    }
}

fn use_slice_chunks_literal() {
    // test code goes here
    let slice = [1, 2, 3];
    for chunk in slice.chunks(2) {
        println!("{} {}", chunk[0], chunk[1]);
    }
}

fn use_slice_chunks_variable() {
    // test code goes here
    let slice = [1, 2, 3];
    let i = 2;
    for chunk in slice.chunks(i) {
        println!("{} {}", chunk[0], chunk[1]);
    }
}

fn use_long_slice_windows_literal() {
    // test code goes here
    let slice = [1, 2, 3];
    for window in slice.windows(27) {
        println!("{} {}", window[0], window[1]);
    }
}

fn main() {}
