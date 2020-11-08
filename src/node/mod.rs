#[derive(Debug, Clone)]
pub struct Node{
    pub rgb:       (u8, u8, u8),
    pub coords:    (u32, u32),
    pub max_links: usize,
    pub links:     Vec<(u32, u32)>,
    pub usable:    bool,
}

impl Node{

    //Creates a new node
    pub fn new_node(color: (u8, u8, u8), coords: (u32, u32)) -> Self{
        Node{
            rgb:       color,
            coords,
            max_links: 0,
            links:     Vec::new(),
            usable:    true
        }
    }

    pub fn assign_links(&mut self, vector: &[Self]){
        if let Some(index) = self.up_pixel(vector) {
            self.links.push(vector[index].coords);
            self.max_links += 1;
        }

        if let Some(index) = self.down_pixel(vector) {
            self.links.push(vector[index].coords);
            self.max_links += 1;
        }

        if let Some(index) = self.left_pixel(vector) {
            self.links.push(vector[index].coords);
            self.max_links += 1;
        }

        if let Some(index) = self.right_pixel(vector) {
            self.links.push(vector[index].coords);
            self.max_links += 1;
        }
    }


    pub fn check_valid_links(&mut self, vector: &[Self]){

        let mut new_links = Vec::new();

        for link in &self.links{
            if vector.iter()
                .filter(|node| node.coords == *link && node.usable && node.coords != self.coords)
                .count() == 1{
                new_links.push(*link);
            }
        }


        self.links.clear();

        for i in &new_links{
            let mut is_inside = false;

            for j in &self.links{
                if i == j{
                    is_inside = true;
                }
            }

            if !is_inside{
                self.links.push(*i);
            }
        }

        //self.links = new_links;

        if self.links.is_empty(){
            self.usable = false;
        }
    }

    pub fn find_and_replace(&mut self, to_be_found: (u32, u32), replace_with: (u32, u32)){
        
        self.links = self.links
            .clone()
            .into_iter()
            .filter(|link| *link != to_be_found)
            .collect();

        self.links.push(replace_with);
        
        

        /*
        let temp: Vec<(u32, u32)>= self.links
            .clone()
            .into_iter()
            .filter(|link| *link != to_be_found)
            .collect();

        self.links.clear();

        for i in &temp{
            let mut is_inside = false;

            for j in &self.links{
                if i == j{
                    is_inside = true;
                }
            }

            if !is_inside{
                self.links.push(*i);
            }
        }
        self.links.push(replace_with);
        */
        
    }


    //Returns the index in the vector of the element that is up-down-left-right
    pub fn up_pixel(&self, vector: &[Self]) -> Option<usize>{
        for (index, node) in vector.iter().enumerate(){
            if node.coords.0 == self.coords.0 && node.coords.1 as i64 == self.coords.1 as i64 - 1{
                return Some(index)
            }
        }
        None
    }

    pub fn down_pixel(&self, vector: &[Self]) -> Option<usize>{
        for (index, node) in vector.iter().enumerate(){
            if node.coords.0 == self.coords.0 && node.coords.1 as i64 == self.coords.1 as i64 + 1{
                return Some(index)
            }
        }

        None
    }

    pub fn left_pixel(&self, vector: &[Self]) -> Option<usize>{
        for (index, node) in vector.iter().enumerate(){
            if node.coords.0 as i64 == self.coords.0 as i64 - 1 && node.coords.1 as i64 == self.coords.1 as i64{
                return Some(index)
            }
        }

        None
    }

    pub fn right_pixel(&self, vector: &[Self]) -> Option<usize>{
        for (index, node) in vector.iter().enumerate(){
            if node.coords.0 as i64 == self.coords.0 as i64 + 1 && node.coords.1 as i64 == self.coords.1 as i64{
                return Some(index)
            }
        }
        None
    }
    
}