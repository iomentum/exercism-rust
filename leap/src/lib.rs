pub fn is_leap_year(year: u64) -> bool {
    // if year divisible par 4
    if year % 4 == 0 {
        // le chiffre est divisible par 4 et on check si il l'est par 100
        if year % 100 == 0 {
            // le chiffre est divisible par 100 et on check si il l'est par 400
            if year % 400 == 0 {
                // le chiffre est divisible par 100 et 400 donc ok
                return true;
            }
        } else {
            return true;
        }
    }
    return false;
}
