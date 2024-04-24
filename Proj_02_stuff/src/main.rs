use rand::Rng;

/// Generates a random number bewtween 1 and 100 (inclusive)
fn random_number() -> i32 {
    rand::thread_rng().gen_range(1..=100)
}

fn fib(n: u64) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn fib2(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    fib(n - 1) + fib(n - 2)
}

fn fib3(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    fib(n - 1) + fib(n - 2)
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct UserPreferences {
    language: String,
    currency: String,
    session_seconds: u64,
    authorized: Vec<String>,
    timezone: String,
    theme: String,
    notification_sound: String,
    email_notification: bool,
    dark_mode: bool,
    auto_save: bool,
    font_size: u8,
    show_images: bool,
    show_avatars: bool,
    secondary_email: Option<String>,
}

fn main() {
    println!("Random number: {}", random_number());

    let test_num = 8;
    println!("Fibonacci of {}: {}", test_num, fib(test_num));

    let user_1_preferences = UserPreferences {
        language: String::from("pt-BR"),
        currency: String::from("usd"),
        session_seconds: 60 * 60 * 60,
        authorized: vec![String::from("read"), String::from("write")],
        timezone: String::from("dark"),
        theme: String::from("Adwaita"),
        notification_sound: String::from("beep.mp3"),
        email_notification: true,
        dark_mode: true,
        auto_save: true,
        font_size: 10,
        show_images: true,
        show_avatars: true,
        secondary_email: Some(String::from("example@example.com")),
        //  secondary_email: None,
    };

    let UserPreferences {
        language,
        secondary_email,
        ..
    } = &user_1_preferences;

    println!("My language is: {}", language);
    println!(
        "My sec e-mail is: {}",
        secondary_email
            .as_ref()
            .map(|email| email.as_str())
            .unwrap_or_default()
    );

    let user_2_preferences = UserPreferences {
        language: String::from("pt-BR"),
        currency: String::from("usd"),
        session_seconds: 60 * 60 * 60,
        authorized: vec![String::from("read"), String::from("write")],
        timezone: String::from("dark"),
        theme: String::from("Adwaita"),
        notification_sound: String::from("beep.mp3"),
        email_notification: true,
        dark_mode: true,
        auto_save: true,
        font_size: 10,
        show_images: true,
        show_avatars: true,
        //        secondary_email: Some(String::from("example@example.com")),
        secondary_email: None,
    };

    let user_preferences_array = vec![user_1_preferences, user_2_preferences];

    for (index, user_preferences) in user_preferences_array.iter().enumerate() {
        println!("User {} preferences: {:?}", index + 1, user_preferences)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_number_generation() {
        let test_number = random_number();
        assert!(test_number >= 1 && test_number <= 100);
    }

    #[test]
    fn test_fib() {
        assert!(fib(8) == 21)
    }

    #[test]
    fn test_fib2() {
        assert!(fib2(8) == 21)
    }
}
