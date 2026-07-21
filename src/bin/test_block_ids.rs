use NexusCore_MC::block::BlockType;

fn main() {
    let air_id = BlockType::Air.runtime_id();
    let bedrock_id = BlockType::Bedrock { infiniburn: false }.runtime_id();
    let dirt_id = BlockType::Dirt { coarse: false }.runtime_id();
    let grass_id = BlockType::Grass.runtime_id();

    println!("Air ID: {}", air_id);
    println!("Bedrock ID: {}", bedrock_id);
    println!("Dirt ID: {}", dirt_id);
    println!("Grass ID: {}", grass_id);
}
