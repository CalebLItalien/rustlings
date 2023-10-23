// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self, grade_type: &str) -> String {
        let grade = &self.grade;
        if grade_type == "letter"{
            format!("{} ({}) - achieved a grade of {}",
                &self.student_name, &self.student_age, Self::convert_to_letter_grade(self.grade))
        }
        else {
            format!("{} ({}) - achieved a grade of {}",
                &self.student_name, &self.student_age, &self.grade)
        }
    }
    fn convert_to_letter_grade(grade: f32) -> &'static str {
        if grade < 0.0 {
            return "Invalid Grade";
        }
        match grade {
            _ if grade < 5.5 => "F-",
            _ if grade >= 5.5 && grade < 6.0 => "F+",
            _ if grade >= 6.0 && grade < 6.5 => "D-",
            _ if grade >= 6.5 && grade < 7.0 => "D+",
            _ if grade >= 7.0 && grade < 7.5 => "C-",
            _ if grade >= 7.5 && grade < 8.0 => "C+",
            _ if grade >= 8.0 && grade < 8.5 => "B-",
            _ if grade >= 8.5 && grade < 9.0 => "B+",
            _ if grade >= 9.0 && grade < 9.5 => "A-",
            _ => "A+",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print("numeric"),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: 10.0,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print("letter"),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
