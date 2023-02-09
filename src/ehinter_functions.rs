// functional code for the Ehinter app


// creates the email hint (This runs every frame)
pub fn fn_hint(email_og: String) -> String {
    
    // make sure no whitespace
    let email = email_og.trim();

    if email.contains('@') {

    // string stuff
    let at_index = email.find('@').expect("This is not a valid email!");
    
        if at_index <= 3 {
            return "Error: Email is invalid; Cannot index NULL!".to_string();
        }

        let mut hint = email[..2].to_string();
        hint.push_str(&"*".repeat(at_index-3));
        hint.push_str(&email[at_index-1..]);
        
        return hint

        }

        else {
            return "".to_string();
        }
    
}


// handles clipboard copying
pub fn fn_copy(ehint: String) {
    let mut clipboard = arboard::Clipboard::new().unwrap();
    clipboard.set_text(ehint).unwrap();
}

// handles clipboard pasting (solely used for the auto-paste feature)
pub fn fn_paste() -> String {
    let mut clipboard = arboard::Clipboard::new().unwrap();
    let cb_data = clipboard.get_text().unwrap();

    return cb_data.to_string()
}

