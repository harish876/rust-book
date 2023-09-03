fn main(){
    let s = String::from("Foobar");
    println!("Substring is {}" , sub_string(&s,0,3));
    println!("Substring is {}" , &s[0..3]);

}   

fn sub_string(s:&String,start: usize,end: usize) -> String {
    let s_as_bytes = s.as_bytes();
    let mut s_substr = String::new();
    for (i, &item) in s_as_bytes.iter().enumerate() {
        if i >=start && i<end {
            s_substr.push(item as char)
        }
    }
    return s_substr;
}