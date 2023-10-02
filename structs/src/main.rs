struct UserInfo {
    name: String,
    age: i32,
    developer_status: bool,
}

struct Color(i32, i32, i32);
struct Convolve(i32, i32, i32, i32, i32);
struct Origin(i32, i32, i32);

fn admin_info(username: String, age: i32, developer_status: bool) -> UserInfo {
    UserInfo {
        name: username,
        age,
        developer_status,
    }
}

fn convulation(color1: &mut Color, color2: &mut Color) -> Convolve {
    
    let x1 = color1.0 * color2.2;
    let x2 = (color1.0 * color2.1) + (color1.1 * color2.2);
    let x3 = (color1.0 * color2.0) + (color1.1 * color2.1) + (color1.2 * color2.2);
    let x4 = (color1.1 * color2.0) + (color1.2 * color2.1);
    let x5 = color1.2 * color2.0;

    let convolve = Convolve(x1, x2, x3, x4, x5);
    return convolve;
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

    let mut color1_code = Color(1, -1, 1);
    let mut color2_code = Color(0, 1, 0);
    let origin_point = Origin(0, 0, 0);
    // Even though color_code and origin_point look the same, they are totally different
    // They can't be pushed into each others functions 
    
    let convolve: Convolve = convulation(&mut color1_code, &mut color2_code);

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

    println!("Attempt to get username back: {} \n", user_name);

    println!("Origin Point: ({}, {}, {})", 
             origin_point.0, origin_point.1, origin_point.2);

    println!("Convulation result: {}, {}, {}, {}, {} \n", 
             convolve.0, convolve.1, convolve.2, convolve.3, convolve.4);

    let test_list: &[i32] = &vec!(1, 2, 3, 4);
    for (i, k) in test_list.iter().enumerate() {
        println!("{}: {}", i + 1, k);  
    }
}
