// Создайте структуру Student с полями: name (String), grades (Vec<u32>)
// Реализуйте методы:
// - new() для создания студента
// - add_grade() для добавления оценки
// - average() для вычисления среднего балла
// - highest_grade() для нахождения максимальной оценки

pub trait StudentTrait {
    fn new(name: String, grades: Vec<u32>) -> Self;
    fn add_grade(&mut self, grade: u32) -> String;
    fn average(&self) -> Result<u32, String>;
    fn highest_grade(&self) -> Result<u32, String>;
}

#[derive(Debug)]
pub struct Student {
    pub name: String,
    pub grades: Vec<u32>,
}

impl StudentTrait for Student {
    fn new(name: String, grades: Vec<u32>) -> Self {
        Student { name, grades }
    }

    fn add_grade(&mut self, grade: u32) -> String {
        if self.grades.len() == 0 {
            return "Array for grades is empty".to_string();
        }

        self.grades.push(grade);
        "grade added".to_string()
    }

    fn average(&self) -> Result<u32, String> {
        if self.grades.len() == 0 {
            return Err("Array for grades is empty".to_string());
        }
        Ok(self.grades.iter().sum::<u32>() / (self.grades.len() as u32))
    }

    fn highest_grade(&self) -> Result<u32, String> {
        if self.grades.len() == 0 {
            return Err("Array for grades is empty".to_string());
        }
        Ok(self.grades.clone().into_iter().max().expect("Error"))
    }
}
