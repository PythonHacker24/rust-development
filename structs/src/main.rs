struct UserInfo {
    name: String,
    age: i32,
    developer_status: bool,
}

fn admin_info(username: String, age: i32, developer_status: bool) -> UserInfo {
    UserInfo {
        name: username,
        age,
        developer_status,
    }
}

fn main() {
    let user = UserInfo {
        name: "Aditya Patil".to_string(),
        age: 19,
        developer_status: true,
    };

    let mut admin = admin_info("Leonard Hofstader".to_string(),
    25, true);

    let user_name = user.name;
    let user_age = user.age;
    let user_developer_status = user.developer_status;

    let admin_name = admin.name;
    let admin_age = admin.age;
    let admin_developer_status = admin.developer_status;
    
    admin.name = "Sheldon Cooper".to_string();

    println!("Admin name: {} \nAge: {} \nDeveloper Status: {} \n",
             admin_name, admin_age, admin_developer_status);

    println!("Username: {} \nAge: {}, \nDeveloper Status: {}", 
             user_name, user_age, user_developer_status);
}
