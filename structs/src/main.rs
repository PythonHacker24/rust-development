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

    let update_user = UserInfo {
        name: "Georges".to_string(),
        ..user
    };

    let mut admin = admin_info("Leonard Hofstader".to_string(),
    25, true);

    let user_name = user.name;
    let user_age = user.age;
    let user_developer_status = user.developer_status;

    let admin_name = admin.name;
    let admin_age = admin.age;
    let admin_developer_status = admin.developer_status;
    
    
    let update_user_name = update_user.name;
    let update_user_age = update_user.age;
    let update_user_developer_status = update_user.developer_status;

    admin.name = "Sheldon Cooper".to_string();

    println!("Admin name: {} \nAge: {} \nDeveloper Status: {} \n",
             admin_name, admin_age, admin_developer_status);

    println!("Username: {} \nAge: {}, \nDeveloper Status: {} \n", 
             user_name, user_age, user_developer_status);

    println!("Updated Username: {} \nAge: {} \nDeveloper Status: {} \n",
             update_user_name, update_user_age, update_user_developer_status);

    println!("Attempt to get username back: {}", user_name);
}
