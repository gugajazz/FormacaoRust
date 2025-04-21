/* 
Edição de strings: 
Fazer um programa capaz de fazer vários tipos de alterações a strings. 
Remover/adicionar caracteres (ou palavras), colocar em upper case e lower case.
*/

fn main() {
    println!("Hello, world!");

    let mut string = String::from("Hello, World!");
    make_strings_cool(&mut string);
    println!("{string}");

}

fn make_strings_cool(string_to_change: &mut String) {
    *string_to_change = string_to_change.to_uppercase();
    *string_to_change = string_to_change.replace("O", "0");
    *string_to_change = string_to_change.replace("L", "1");
    *string_to_change = string_to_change.replace("E", "3");
    *string_to_change = string_to_change.replace("A", "4");
    *string_to_change = string_to_change.replace("S", "5");
    *string_to_change = string_to_change.replace("T", "7");
    *string_to_change = string_to_change.replace("I", "!");
    *string_to_change = string_to_change.replace("H", "#");
    *string_to_change = string_to_change.replace("G", "9");
    *string_to_change = string_to_change.replace("B", "8");
    *string_to_change = string_to_change.replace("Z", "2");
    *string_to_change = string_to_change.replace("C", "(");
    *string_to_change = string_to_change.replace("J", ")");
    *string_to_change = format!("___-{string_to_change}-___");

    
}

