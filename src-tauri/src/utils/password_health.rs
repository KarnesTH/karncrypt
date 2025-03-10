use lazy_static::lazy_static;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref COMMON_PASSWORDS: HashSet<&'static str> = {
        let common_passwords = include_str!("common_passwords.txt");
        common_passwords.lines().collect()
    };
}

#[derive(Clone)]
pub struct PasswordHealth {
    password: String,
    length: usize,
    pub score: u8,
    pub strength: PasswordStrength,
    has_uppercase: bool,
    has_lowercase: bool,
    has_numbers: bool,
    has_special_chars: bool,
    repeated_chars: usize,
    sequential_chars: usize,
    unique_chars: usize,
    is_common_password: bool,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum PasswordStrength {
    VeryWeak,
    Weak,
    Moderate,
    Strong,
    VeryStrong,
}

impl PasswordHealth {
    /// Create a new PasswordHealth instance with the given password.
    ///
    /// # Arguments
    ///
    /// * `password` - The password to analyze.
    pub fn new(password: &str) -> Self {
        let length = password.len();
        let score = 0;
        let strength = PasswordStrength::VeryWeak;
        let has_uppercase = false;
        let has_lowercase = false;
        let has_numbers = false;
        let has_special_chars = false;
        let repeated_chars = 0;
        let sequential_chars = 0;
        let unique_chars = 0;
        let is_common_password = false;
        let suggestions = Vec::new();

        PasswordHealth {
            password: password.to_string(),
            length,
            score,
            strength,
            has_uppercase,
            has_lowercase,
            has_numbers,
            has_special_chars,
            repeated_chars,
            sequential_chars,
            unique_chars,
            is_common_password,
            suggestions,
        }
    }

    /// Analyze the password and determine its strength.
    ///
    /// # Returns
    ///
    /// A reference to the PasswordHealth instance.
    ///
    /// # Errors
    ///
    /// Returns a Box<dyn std::error::Error> if an error occurs.
    pub fn analyze(&mut self) -> Result<&Self, Box<dyn std::error::Error>> {
        self.check_length();
        self.check_character_types();
        self.check_complexity();
        self.check_common_password();
        self.determine_strength();
        Ok(self)
    }

    /// Check the length of the password and assign a score based on the length.
    fn check_length(&mut self) {
        let length = self.length;

        if length >= 20 {
            self.score += 35;
        } else if length >= 16 {
            self.score += 25;
        } else if length >= 12 {
            self.score += 15;
        } else if length >= 8 {
            self.score += 5;
        } else {
            self.score += 0;
        }

        if length < 8 {
            self.suggestions
                .push("Das Passwort sollte mindestens 8 Zeichen lang sein.".to_string());
        } else if length < 12 {
            self.suggestions.push(
                "Für mehr Sicherheit sollte das Passwort mindestens 12 Zeichen lang sein."
                    .to_string(),
            );
        } else if length > 64 {
            self.suggestions.push(
                "Das Passwort ist sehr lang. Dies könnte die Benutzbarkeit einschränken"
                    .to_string(),
            );
        }
    }

    /// Check the types of characters in the password and assign a score based on the types.
    fn check_character_types(&mut self) {
        for c in self.password.chars() {
            if c.is_uppercase() {
                self.has_uppercase = true;
            } else if c.is_lowercase() {
                self.has_lowercase = true;
            } else if c.is_ascii_digit() {
                self.has_numbers = true;
            } else if c.is_ascii_punctuation() {
                self.has_special_chars = true;
            }
        }

        let mut type_score = 0;
        if self.has_uppercase {
            type_score += 10;
        }
        if self.has_lowercase {
            type_score += 10;
        }
        if self.has_numbers {
            type_score += 10;
        }
        if self.has_special_chars {
            type_score += 15;
        }
        self.score += type_score;

        if !self.has_uppercase {
            self.suggestions
                .push("Das Passwort sollte mindestens einen Großbuchstaben enthalten.".to_string());
        }
        if !self.has_lowercase {
            self.suggestions.push(
                "Das Passwort sollte mindestens einen Kleinbuchstaben enthalten.".to_string(),
            );
        }
        if !self.has_numbers {
            self.suggestions
                .push("Das Passwort sollte mindestens eine Zahl enthalten.".to_string());
        }
        if !self.has_special_chars {
            self.suggestions
                .push("Das Passwort sollte mindestens ein Sonderzeichen enthalten.".to_string());
        }
    }

    /// Check the complexity of the password and assign a score based on the complexity.
    fn check_complexity(&mut self) {
        self.check_repeated_chars();
        self.check_sequential_chars();
        self.count_unique_chars();
    }

    /// Check for repeated characters in the password and assign a score based on the number of repeats.
    fn check_repeated_chars(&mut self) {
        let mut char_counts = HashMap::new();

        for c in self.password.chars() {
            *char_counts.entry(c).or_insert(0) += 1;
        }

        self.repeated_chars = char_counts.values().filter(|&&count| count > 2).count();

        if self.repeated_chars > 0 {
            let deduction = (self.repeated_chars * 5).min(25);
            self.score = self.score.saturating_sub(deduction as u8);

            self.suggestions.push("Vermeide die häufige Wiederholung von Zeichen - das macht dein Passwort vorhersehbarer.".to_string());
        }
    }

    /// Check for sequential characters in the password and assign a score based on the number of sequences.
    fn check_sequential_chars(&mut self) {
        let chars: Vec<char> = self.password.chars().collect();
        let mut sequences = 0;

        for i in 0..chars.len().saturating_sub(2) {
            let c1 = chars[i] as u32;
            let c2 = chars[i + 1] as u32;
            let c3 = chars[i + 2] as u32;

            if c2 == c1 + 1 && c3 == c2 + 1 {
                sequences += 1;
            } else if c2 == c1 - 1 && c3 == c2 - 1 {
                sequences += 1;
            }
        }

        self.sequential_chars = sequences;

        if sequences > 0 {
            let deduction = (sequences * 5).min(25);
            self.score = self.score.saturating_sub(deduction as u8);

            self.suggestions
                .push("Vermeide einfache Sequenzen wie 'abc' oder '123'".to_string());
        }
    }

    /// Count the number of unique characters in the password and assign a score based on the number of unique characters.
    fn count_unique_chars(&mut self) {
        let mut unique = HashSet::new();

        for c in self.password.chars() {
            unique.insert(c);
        }

        self.unique_chars = unique.len();

        let bonus = match self.unique_chars {
            0..=4 => 0,
            5..=8 => 5,
            9..=12 => 15,
            _ => 20,
        };

        self.score = self.score.saturating_add(bonus);

        if self.unique_chars < 8 {
            self.suggestions.push(
                "Verwende mehr unterschiedliche Zeichen für ein stärkeres Passwort".to_string(),
            );
        }
    }

    /// Check if the password is a common password and assign a score based on the result.
    fn check_common_password(&mut self) {
        self.is_common_password = COMMON_PASSWORDS.contains(self.password.as_str());

        if self.is_common_password {
            self.score = self.score.saturating_sub(50);
            self.suggestions.push(
                "Dieses Passwort ist zu häufig. Wähle ein einzigartiges Passwort.".to_string(),
            );
        }
    }

    /// Determine the strength of the password based on the score.
    fn determine_strength(&mut self) {
        self.strength = match self.score {
            0..=20 => PasswordStrength::VeryWeak,
            21..=40 => PasswordStrength::Weak,
            41..=60 => PasswordStrength::Moderate,
            61..=80 => PasswordStrength::Strong,
            _ => PasswordStrength::VeryStrong,
        };
    }

    /// Get the score of the password.
    ///
    /// # Returns
    ///
    /// The score of the password.
    pub fn get_score(&self) -> u8 {
        self.score
    }

    /// Get the strength of the password.
    ///
    /// # Returns
    ///
    /// The strength of the password.
    pub fn get_strength(&self) -> &PasswordStrength {
        &self.strength
    }

    /// Get the suggestions for improving the password.
    ///
    /// # Returns
    ///
    /// A vector of suggestions for improving the password.
    pub fn get_suggestions(&self) -> &[String] {
        &self.suggestions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weak_password() {
        let mut health = PasswordHealth::new("password");
        health.analyze().unwrap();
        assert_eq!(health.get_score(), 0);
        assert_eq!(*health.get_strength(), PasswordStrength::VeryWeak);
        println!("Weak password score: {}", health.get_score());
        println!("Suggestions: {:?}", health.get_suggestions());
    }

    #[test]
    fn test_medium_password() {
        let mut health = PasswordHealth::new("Password123");
        health.analyze().unwrap();
        println!("Medium password score: {}", health.get_score());
        println!("Suggestions: {:?}", health.get_suggestions());
    }

    #[test]
    fn test_strong_password() {
        let mut health = PasswordHealth::new("P@ssw0rd!2023#");
        health.analyze().unwrap();
        println!("Strong password score: {}", health.get_score());
        println!("Suggestions: {:?}", health.get_suggestions());
    }

    #[test]
    fn test_repeated_chars() {
        let mut health = PasswordHealth::new("aaaPassword123!");
        health.analyze().unwrap();
        println!("Password with repeats score: {}", health.get_score());
        println!("Suggestions: {:?}", health.get_suggestions());
    }

    #[test]
    fn test_sequential_chars() {
        let mut health = PasswordHealth::new("abc123Password!");
        health.analyze().unwrap();
        println!("Password with sequences score: {}", health.get_score());
        println!("Suggestions: {:?}", health.get_suggestions());
    }

    #[test]
    fn test_perfect_password() {
        let mut health = PasswordHealth::new("Kx9$-mN7#pL4@jR2&vB5!");
        health.analyze().unwrap();
        println!("Perfect password score: {}", health.get_score());
        println!("Perfect password strength: {:?}", health.get_strength());
        println!("Suggestions: {:?}", health.get_suggestions());
        assert_eq!(health.get_score(), 100);
        assert!(matches!(
            health.get_strength(),
            PasswordStrength::VeryStrong
        ));
        assert!(health.get_suggestions().is_empty());
    }
}
