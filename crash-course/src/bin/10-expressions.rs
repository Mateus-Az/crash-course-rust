enum Access {
    Admin,
    Guest,
}

fn main() {
    // secret file: admins only
    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };
    println!("can access: {:?}", can_access_file);
}
