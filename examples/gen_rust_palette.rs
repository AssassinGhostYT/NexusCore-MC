use std::fs::File;
use std::io::Write;
use NexusCore_MC::protocol::packet::block_palette::get_block_palette;

fn main() {
    let palette = get_block_palette();
    let mut file = File::create("/tmp/rust_palette.bin").unwrap();
    file.write_all(palette).unwrap();
    println!("Generated /tmp/rust_palette.bin successfully!");
}
