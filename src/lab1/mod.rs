// Создайте структуру Student с полями: name (String), grades (Vec<u32>)
// Реализуйте методы:
// - new() для создания студента
// - add_grade() для добавления оценки
// - average() для вычисления среднего балла
// - highest_grade() для нахождения максимальной оценки

pub trait StudentTrait {
    fn new(name: String, grades: Vec<u32>) -> Self;
    fn add_grade(&mut self, grade: u32) -> Result<(), StudentError>;
    fn average(&self) -> Result<u32, String>;
    fn highest_grade(&self) -> Result<u32, StudentError>;
}

#[derive(Debug)]
pub struct Student {
    pub name: String,
    pub grades: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StudentError {
    NoGrades,
    InvalidGrade,
}

impl std::fmt::Display for StudentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StudentError::NoGrades => write!(f, "No grades available"),
            StudentError::InvalidGrade => write!(f, "Grade must be between 0 and 100"),
        }
    }
}

impl StudentTrait for Student {
    fn new(name: String, grades: Vec<u32>) -> Self {
        Student { name, grades }
    }

    fn add_grade(&mut self, grade: u32) -> Result<(), StudentError> {
        if grade > 100 {
            return Err(StudentError::InvalidGrade);
        }

        if self.check_empty() {
            return Err(StudentError::NoGrades);
        }

        self.grades.push(grade);
        Ok(())
    }

    fn average(&self) -> Result<u32, String> {
        self.check_empty();

        Ok(self.grades.iter().sum::<u32>() / (self.grades.len() as u32))
    }

    fn highest_grade(&self) -> Result<u32, StudentError> {
        self.grades
            .iter()
            .max()
            .copied()
            .ok_or(StudentError::NoGrades)
    }
}

impl Student {
    fn check_empty(&self) -> bool {
        self.grades.is_empty()
    }
}
