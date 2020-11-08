//use image::*;
use image::DynamicImage::*;
use std::collections::HashMap;
use std::path::Path;

use crate::node::*;


const RED: (u8, u8, u8) = (255,   0,   0);
const GRN: (u8, u8, u8) = (  0, 255,   0);
//const WHT: (u8, u8, u8) = (255, 255, 255);
const BLK: (u8, u8, u8) = (  0,   0,   0);

pub fn run(filename: String) -> Result<(), ()>{
    println!("Opening image...");
    //Open image
    let image = image::open(filename);

    //Check if valid image
    if let Ok(dyn_img) = image{

        println!("Done\n");

        //Get pixel vector if possible
        let pixel_vector = match dyn_img{

            ImageRgb8(rgb_image) => {
                println!("Creating raw vector...");
                (Some(image::ImageBuffer::into_vec(rgb_image.clone())), Some(rgb_image))
            },
            _ => (None, None)
        };
        println!("Done\n");

        //Check if pixel_vector is Some(x) value
        if let (Some(vector), Some(rgb_image)) = pixel_vector {
            
            println!("Starting 'tree_creation' function...");
            //Check if a tree was extracted from the data
            if let Ok(data) = tree_creation(&vector, rgb_image.dimensions()){

                let (tree, start, finish) = data;

                //To store the path
                let mut path_stack: Vec<(u32, u32)> = Vec::new();

                //To store the "used" nodes
                let mut used_stack: Vec<(u32, u32)> = Vec::new();
                let _ = dfs(
                    &tree,
                    start,
                    finish,
                    &mut path_stack,
                    &mut used_stack
                );

                /*
                println!("Path: {:?}", path_stack);
                */

                let mut new_image: Vec<u8> = Vec::new();

                let to_be_colored = pixels_to_color(path_stack);

                println!("Drawing...");
                for elem in (0..vector.len()).step_by(3){
                    let pixel_coords = ((elem as u32/3)%rgb_image.dimensions().0, (elem as u32/3)/rgb_image.dimensions().0);

                    let pixel = (vector[elem], vector[elem+1], vector[elem+2]);

                    if to_be_colored.contains(&pixel_coords) && pixel != GRN{
                        new_image.push(0);
                        new_image.push(0);
                        new_image.push(255);
                    }else{
                        new_image.push(pixel.0);
                        new_image.push(pixel.1);
                        new_image.push(pixel.2);
                    }
                }

                let _ = image::save_buffer(
                    Path::new("./output.png"),
                    &new_image,
                    rgb_image.dimensions().0,
                    rgb_image.dimensions().1,
                    image::ColorType::Rgb8
                );
                println!("Done!");

            }else{
                println!("Invalid tree: 'start' and/or 'finish' not found");
                return Err(())
            };

        }else{
            println!("Image isn't RGB 8-bit");
            return Err(())
        }

        Ok(())
    }else{
        println!("Image didn't open correctly");
        Err(())
    }
}

fn tree_creation(vector: &[u8], dimensions: (u32, u32)) -> Result<(HashMap<(u32, u32), Vec<(u32, u32)>>, (u32, u32), (u32, u32)), ()>{

    let mut start:  Option<(u32, u32)> = None;
    let mut finish: Option<(u32, u32)> = None;

    //Node "tree"
    let mut node_vector: Vec<Node> = Vec::new();

    println!("Reading image bytes...");

    //Create color_vector
    for i in (0..vector.len()).step_by(3){

        //Calculate in advance color and coordinates
        let color = (vector[i as usize], vector[(i+1) as usize], vector[(i+2) as usize]);

        //Formulae that convert one-dimentional indexes to bi-dimensional coordinates
        let coordinates = ((i as u32/3)%dimensions.0, (i as u32/3)/dimensions.0);

        //Setup start and finish coordinates; needed for dfs implementation
        if color == GRN{
            start  = Some(coordinates);
        }else if color == RED{
            finish = Some(coordinates);
        }

        //Push to the (unfinished) tree
        node_vector.push(
            Node::new_node(color, coordinates)
        );
    }

    if start.is_none() {
        return Err(())
    }

    if finish.is_none() {
        return Err(())
    }
    println!("Done\n");


    println!("Filtering black pixels...");
    //Filter for useful pixels, i.e. the non-black ones
    let mut node_vector: Vec<Node> = node_vector
        .into_iter()
        .filter(|node| node.rgb != BLK)
        .collect();

    println!("Done\n");


    println!("Assigning links to every node...");

    
    // Don't know how to improve this snippet without using clone()-ing
    let cpy = node_vector.clone();

    let length = node_vector.len();
    for element_index in 0..length{
        node_vector[element_index].assign_links(&cpy);
    }
    println!("Done\n");

    /*
    //Debug loop
    for (index, node) in node_vector.iter().enumerate(){
        println!("{}, {:?}", index, node);
    }
    println!("\n\n\n\n");
    */
    
    println!("Preparing tree...");
    let tree = create_nodes(&node_vector);
    println!("Done\n");
    
    Ok(
        (
            tree,
            match start { Some(value) => value, _ => return Err(())}, 
            match finish{ Some(value) => value, _ => return Err(())}
        )
    )
}

fn create_nodes(node_vector: &[Node]) -> HashMap<(u32, u32), Vec<(u32, u32)>>{

    let mut nodes = node_vector.to_owned();
    let mut keep_looping = true;
    
    while keep_looping{

        keep_looping = false;

        //let cpy = nodes.clone();

        for (elem_index, element) in nodes.clone().into_iter().enumerate(){
            //Although clippy says so, it is not possible to collapse these two ifs
            if element.links.len() == 2 && element.usable && element.links.len() <= element.max_links{
                if element.links[0].0 == element.links[1].0 || element.links[0].1 == element.links[1].1{
    
                    keep_looping = true;
    
                    /*
                    if a b c are the nodes, and b is the current node,
                    eliminate in a the link to b and put instead the link to c
                    eliminate in c the link to b and put instead the link to a
                    */
    
                    //Tuple holds the coords of the previous node
                    let tuple: Option<(usize, _)> = nodes
                        .clone()
                        .into_iter()
                        .enumerate()
                        .find(|(_, node)| node.coords == element.links[0]);
                    
                    if let Some((index, _)) = tuple{
                        nodes[index].find_and_replace(element.coords, element.links[1]);
                        nodes[index].usable = true;
                    }
    
                    //Tuple holds the coords of the next node
                    let tuple: Option<(usize, _)> = nodes
                        .clone()
                        .into_iter()
                        .enumerate()
                        .find(|(_, node)| node.coords == element.links[1]);
                    
                    if let Some((index, _)) = tuple{
                        nodes[index].find_and_replace(element.coords, element.links[0]);
                        nodes[index].usable = true;
                    }

                    nodes[elem_index].usable = false;
                }
            }

            let cpy = nodes.clone();
            nodes[elem_index].check_valid_links(&cpy);
            
            
        }
    }

    let mut node_table: HashMap<(u32, u32), Vec<(u32, u32)>> = HashMap::new();

    for i in nodes.iter().filter(|node| node.usable){
        node_table.insert(i.coords, i.links.clone());
    }
    
    /*
    Debug loop
    for (index, node) in node_table.iter().enumerate(){
        //for (index, node) in node_vector.iter().enumerate(){
        println!("{}, {:?}", index, node);
    }
    */
    
    node_table
}

#[derive(PartialEq)]
enum Outcome{
    Found,
    NotFound
}

//Recursive implementation of the depth-first-search algorithm
fn dfs(
    tree: &HashMap<(u32, u32), Vec<(u32, u32)>>,
    current: (u32, u32),
    target: (u32, u32),
    path_stack: &mut Vec<(u32, u32)>,
    used_stack: &mut Vec<(u32, u32)>) -> Outcome{

    
    if used_stack.contains(&current){
        return Outcome::NotFound
    }

    path_stack.push(current);
    if current == target{
        return Outcome::Found
    }
    used_stack.push(current);


    let links = tree.get(&current);

    if let Some(vec) = links{
        for link in vec.iter(){
            let output = dfs(tree, *link, target, path_stack, used_stack);

            if output == Outcome::Found{
                return Outcome::Found
            }
        }

        let _ = path_stack.pop();
        return Outcome::NotFound
    }
    
    Outcome::NotFound
}

fn pixels_to_color(path: Vec<(u32, u32)>) -> Vec<(u32, u32)>{

    let mut pixels_to_color = Vec::new();

    for pixel in 0..(path.len()-1){
        if path[pixel].0 == path[pixel+1].0{

            if path[pixel+1].1 >= path[pixel].1{
                for i in path[pixel].1..path[pixel+1].1{
                    pixels_to_color.push((path[pixel].0, i));
                }
            }else{
                for i in path[pixel+1].1..path[pixel].1{
                    pixels_to_color.push((path[pixel].0, i));
                }
            }
        }else{
            if path[pixel+1].0 >= path[pixel].0{
                for i in path[pixel].0..path[pixel+1].0{
                    pixels_to_color.push((i, path[pixel].1));
                }
            }else{
                for i in path[pixel+1].0..path[pixel].0{
                    pixels_to_color.push((i, path[pixel].1));
                }
            }
            
        }
    }

    pixels_to_color
}