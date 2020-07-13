use rand::Rng;

struct Room {
    room_index: u32,
    connected_rooms: Vec<u32>,
}

struct Map {
    map : Vec<Room>,

}

impl Map {

    fn create(size: u32) -> Map {
        Map {
            map: (0..size).map(|node_index| Room{room_index:node_index, connected_rooms: Vec::new()}).collect()
        }
    }

    fn debug_print(&self) {
        println!("Map size: {}", self.map.len());
        for room in &self.map {
            println!("[{}]", room.room_index)
        }

    }


}

#[cfg(test)]
mod tests
{
    #[test]
    fn map_create() {

    }
}

fn main() {
    println!("Hello, world!");

    let map = Map::create(25);
    map.debug_print();
}
