struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
  let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someone"),
    active: true,
    sign_in_count: 1
  };

  let mut user2 = User {
    email: String::from("someone2@example.com"),
    username: String::from("someone2"),
    active: true,
    sign_in_count: 1
  };

  user2.email = String::from("another@exapmle.com");

  let user3 = User {
    email: String::from("user3@example.com"),
    username: String::from("user3"),
    ..user1
  };

  let black = Color(0,0,0);
  let origin = Point(0,0,0);
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}