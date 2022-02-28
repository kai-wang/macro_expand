use crate::mac::Mac;
mod mac;

def_mac!(TodoMac, "todo", &["id", "title", "description"]);

fn main() {

  let todo_mac = TodoMac::new();
  println!("todo table: {:?}", todo_mac.columns_default());
}
