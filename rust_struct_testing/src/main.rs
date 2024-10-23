pub mod student;
use student::Student;

fn main() {
    let name = "John".to_string();
    let major = "Computer Science".to_string();
    let mut s = Student::new_student(name, major);

    s.change_major("Data Sciene");
    s.introduce_yourself();

}