#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn sorted_rectangles_by_width() -> [Rectangle; 3] {
        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];
        // Sort by key implements FnMut trait
        // Closure is called multiple times for each item in the slice
        // Sort by key function cannot take ownership of any values as it can be called multiple times
        list.sort_by_key(|r| r.width);
        
        list
    }
}
