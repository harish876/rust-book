use std::cmp::Ordering;
use std::error::Error;
#[derive(Debug)]
struct Employee {
    name: String,
    access_level: i32,
}

struct Employees {
    employees: Vec<Employee>,
}
impl Employees {
    fn new() -> Self {
        Self {
            employees: Vec::new(),
        }
    }
    fn add_employee(&mut self, employee: Employee) {
        self.employees.push(employee)
    }
    fn find_employee(&self, input_name: String) -> Option<&Employee> {
        self.employees.iter().find(|&x| x.name == input_name)
    }
}

impl Employee {
    fn new(name: String, access_level: i32) -> Self {
        Self { name, access_level }
    }
}
fn helper() -> Result<(), Box<dyn Error>> {
    let mut all_employees = Employees::new();
    let emp1 = Employee::new(String::from("Girish"), 100);
    let emp2 = Employee::new(String::from("Surish"), 500);
    let emp3 = Employee::new(String::from("Harish"), 1000);

    all_employees.add_employee(emp1);
    all_employees.add_employee(emp2);
    all_employees.add_employee(emp3);

    match all_employees.find_employee(String::from("girish")) {
        Some(employee) => {
            return match employee.access_level.cmp(&200) {
                Ordering::Less => Err("Access Cannot be Granted".into()),
                _ => {
                    println!("{:?}",employee);
                    Ok(())
                }
            };
        }
        None => Err("Employee not found".into()),
    }
    
}
fn main() {
    helper().expect("Woiii");
}
