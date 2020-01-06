pub fn is_leap_year(year: u64) -> bool {
    if year % 4 == 0 { // if year is divisible by 4
        if year % 100 == 0 && year % 400 != 0 { // if it's divisible by 100 and isn't divisible by 400, return false
            return false
        }
        return true
    }
    return false
}
