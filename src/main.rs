// Structs 
// #[derive(Debug)] 은 정보를 보여주는 함수
// 이 정보는 Debug 타입을 구현하는 데이터타입
#[derive(Debug)]
struct Student {
    name: String, // String은 UTF-8 정수
    age: u8, // u8은 unsigned 8비트 정수
    pass: bool, // bool은 boolean 타입
}
fn main() {
    let x:u8 = 5; // u8은 unsigned 8비트 정수

    // 정수 자료형은 숫자로 표현되는 값을 나타내는 데이터타입
    // to_owned()는 인스턴스를 복사하는 함수
    // "Ralph".to_owned()는 문자열을 새로운 인스턴스로 복사하는 함수
    let stu: Student = Student{
        name: "Ralph".to_owned(),
        age: 12, 
        pass: true
    };

    // {:?} 은 정보를 보여주는 함수
    println!("Student:{:?}", stu);
    println!("Student.name: {}, Student age: {}", stu.name, stu.age)
}
