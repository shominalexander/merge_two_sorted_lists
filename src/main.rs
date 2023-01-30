struct Item { next: Option<Box<Item>>, value: i32 }

impl Item {
 fn new(mut vector: Vec<i32>) -> Option<Item> {
  if vector.len() > 0 {
   let mut item: Item = Item { next: None, value: 0 };

   vector.sort();

   for (index, value) in vector.into_iter().rev().enumerate() {
    if index == 0 {
     item.next = None;

    } else {//if index == 0 {
     item.next = Some(Box::new(Item { next: item.next.take(), value: item.value }));

    }//} else {//if index == 0 {

    item.value = value;
   }//for (index, value) in vector.into_iter().rev().enumerate() {

   Some(item)

  } else {//if vector.len() > 0 {
   None

  }//} else {//if vector.len() > 0 {
 }//fn new(mut vector: Vec<i32>) -> Option<Item> {
}//impl List {

fn request() -> String {
 let mut value: String = String::new();

 std::io::stdin().read_line(&mut value).expect("Input failed");

 value = value.trim().to_string();
 value = value.replace("\n", "") ;
 value = value.replace("\r", "") ;

 value
}//fn request() -> String {

fn reverse(mut item: Item) -> Item {
 let mut first: Item = Item { next: None, value: item.value };

 while let Some(rest) = item.next {
  first.next = Some(Box::new(Item { next: first.next.take(), value: first.value }));

  first.value = rest.value;

  item = *rest;
 }//while let Some(rest) = first.next {

 first
}//fn reverse(mut item: Item) -> Item {

fn union(mut first: Item, mut second: Item) -> Item {
 let mut union: Item = Item { next: None, value: 0 };

 loop {
  if first.value < second.value {
   union.value = first.value;

   if let Some(rest) = first.next {
    union.next = Some(Box::new(Item { next: union.next.take(), value: union.value }));

    first = *rest;

   } else {//if let Some(rest) = first.next {
    union.next = Some(Box::new(Item { next: union.next.take(), value: union.value }));

    union.value = second.value;

    while let Some(rest) = second.next {
     union.next = Some(Box::new(Item { next: union.next.take(), value: union.value }));

     union.value = rest.value;

     second = *rest;
    }//while let Some(rest) = second.next {
 
    break;
   }//} else {//if let Some(rest) = first.next {

  } else {//if first.value < second.value {
   union.value = second.value;

   if let Some(rest) = second.next {
    union.next = Some(Box::new(Item { next: union.next.take(), value: union.value }));

    second = *rest;

   } else {//if let Some(rest) = second.next {
    union.next = Some(Box::new(Item { next: union.next.take(), value: union.value }));

    union.value = first.value;

    while let Some(rest) = first.next {
     union.next = Some(Box::new(Item { next: union.next.take(), value: union.value }));

     union.value = rest.value;

     first = *rest;
    }//while let Some(rest) = first.next {
 
    break;
   }//} else {//if let Some(rest) = second.next {
  }//} else {//if first.value < second.value {
 }//loop {

 reverse(union)
}//fn union(mut first: Item, mut second: Item) -> Item {

fn vector(mut item: Item) -> Vec<i32> {
 let mut vector: Vec<i32> = Vec::new();

 vector.push(item.value);

 while let Some(rest) = item.next {
  vector.push(rest.value);

  item = *rest;
 }//while let Some(rest) = item.next {

 vector
}//fn vector(mut item: Item) -> Vec<i32> {

fn main() {
 loop {
  println!("\r\n\r\nvectors:");

  let input: String = request();

  if &input[..] == "exit" {
   break;

  } else {//if &input[..] == "exit" {
   match serde_json::from_str::<Vec<Vec<i32>>>(&input[..]) {
    Ok(mut vectors) => {
     if let Some(first_vector) = vectors.pop() {
      if let Some(first_item) = Item::new(first_vector) {
       if let Some(second_vector) = vectors.pop() {
        if let Some(second_item) = Item::new(second_vector) {
         println!("\r\nunion: {:?}", vector(union(first_item, second_item)));

        } else {//if let Some(second_item) = Item::new(second_vector) {
         println!("\r\nunion: {:?}", vector(first_item));

        }//} else {//if let Some(second_item) = Item::new(second_vector) {

       } else {//if let Some(second_vector) = vectors.pop() {
        println!("\r\nunion: {:?}", vector(first_item));

       }//} else {//if let Some(second_vector) = vectors.pop() {

      } else {//if let Some(first_item) = Item::new(first_vector) {
       println!("\r\nunion: []");

      }//} else {//if let Some(first_item) = Item::new(first_vector) {

     } else {//if let Some(first_vector) = vectors.pop() {
      println!("\r\nunion: []");

     }//} else {//if let Some(first_vector) = vectors.pop() {
    }//Ok(mut vector) => {

    Err(error) => {
     println!("Error: {:?}", error);

    }//Err(error) => {
   }//match serde_json::from_str::<Vec<i32>>(&string[..]) {
  }//} else {//if &input[..] == "exit" {
 }//loop {
}//fn main() {
