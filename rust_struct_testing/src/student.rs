pub struct Student {
    name: String,
    major: String
    }

impl Student {
    pub fn new_student(name:String, major:String) -> Self {
        Student {
            name: name,
            major: major,
        }
    }
    pub fn introduce_yourself(&self) {
        println!("My name is: {}, I am majoring in: {}", self.name, self.major)
    }
    pub fn change_major(&mut self, new_major:&str) {
        self.major = new_major.to_string();
    }
}

#[cfg(test)]
tests {
    use super::*;

    #[test]
    fn test_student_creation {
        let s = Student::new_student("Alex".to_string(), "Computer Science".to_string());
        assert_eq!(s.name, "Alex".to_string());
    }
}