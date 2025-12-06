use text_io::read;

struct Box3D {
    length: i32,
    width: i32,
    height: i32,
}

impl Box3D {
    fn volume(&self) -> i32 {
        self.length * self.width * self.height
    }
}

fn main() {
    let n: usize = read!();

    for _ in 0..n {
        let length: i32 = read!();
        let width: i32 = read!();
        let height: i32 = read!();

        let b = Box3D { length, width, height };

        if b.height < 41 {
            println!("{}", b.volume());
        }
    }
}
