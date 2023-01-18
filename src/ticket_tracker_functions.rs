// functional code for ticket tracking module

pub fn vec_iter_reg(reg_tick: &mut Vec<String>) -> String {
    if reg_tick.is_empty() {
        return "No Tickets Saved.".to_string();
    } else {
        for i in 0..reg_tick.len() {
            if !reg_tick[i].starts_with("#") {
                reg_tick[i] = "#".to_owned() + &reg_tick[i];
            }
        }
        let last_index = reg_tick.len() - 1;
        let parsed_list = reg_tick.iter().enumerate()
            .map(|(i, tick_num)| {
                if i == last_index {
                    return tick_num.to_string();
                } else {
                    return tick_num.to_string() + ", ";
                }
            })
            .collect::<String>();
        return parsed_list;
    }
}