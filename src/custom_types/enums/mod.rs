mod c_like;

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    password: String,
    sign_in_count: u64,
    active: bool,
}


enum UserEvent { 
    LogOut,
    LogIn(String, String),
    SignUp {
        username: String,
        email: String,
        password: String,
    },
    Delete(String),
}

fn event(event: UserEvent, user: Option<User>, users: &mut Vec<User>) {
    match event {
        UserEvent::LogOut => {
            match user {
                Some(user) => {
                    println!("--Logging out: {}", user.username);
                },
                None => println!("--No user logged in"),
            }
        },
        UserEvent::LogIn(email, password) => {
            let mut is_user_find = false;

            users.iter().for_each(|user| {
                if user.email == email && user.password == password {
                    println!("--Logging in: {}", user.username);

                    is_user_find = true;

                    return;
                }
            });

            if !is_user_find {
                println!("--User not found");
            }
        },
        UserEvent::SignUp { username, email, password } => {
            users.iter().for_each(|user| {
                if user.email == email {
                    println!("--Username already exists");

                    return;
                }
            });

            users.push(User {
                username,
                email,
                password,
                sign_in_count: 0,
                active: true,
            });

            println!("--User signed up");
        },
        UserEvent::Delete(email) => {
            let index = users.iter().position(|user| user.email == email);

            match index {
                Some(index) => {
                    users.remove(index);
                    println!("--Deleting user: {}", users.get(index).unwrap().username);
                },
                None => println!("--User not found"),
            }
        }
    }
}

type Event = UserEvent;

pub(super) fn enums() {
    let john_doe = User {
        username: "JohnDoe".to_string(),
        active: true,
        email: "jdoe@gmail.com".to_string(),
        password: "password".to_string(),
        sign_in_count: 0,
    };

    let testunio = User {
        username: "Testunio".to_string(),
        active: true,
        email: "testunio@gmail.com".to_string(),
        password: "password".to_string(),
        sign_in_count: 0,
    };

    let mut users = vec![john_doe, testunio];

    let sing_up_event = UserEvent::SignUp { username: "Hur".to_string(), email: "test@gmail.com".to_string(), password: "12345".to_string() };

    println!("--original Vec: {:#?}", users);

    event(sing_up_event, None, &mut users);

    println!("--After sign up: {:#?}", users);

    // event(UserEvent::LogOut, testunio, &mut users);

    // println!("--After logging out: {:#?}", users);

    event(UserEvent::LogIn("jdoe@gmail.com".to_string(), "password".to_string()), None, &mut users);

    event(Event::Delete("testunio@gmail.com".to_string()), None, &mut users);

    println!("--After deleting: {:#?}", users);

    println!("--C Like Enums");
    c_like::c_like_enums();
}
