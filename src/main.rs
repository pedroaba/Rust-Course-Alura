const PI: f32 = 3.14;
static GLOBAL_VARIABLE: u8 = 1;

fn sum(a: i32, b: i32) -> i32 {
    let result = a + b;

    println!("{a} + {b} = {result}");
    result
}

fn shadow() {
    let a = 123;

    {
        let b = 456;
        let a = 777;
        println!("inside, b = {b}");
        println!("inside, a = {a}");
    }

    println!("outside, a = {a}");
}

fn scope() {
    let variable = 128;
    println!("Variable = {}, length = {}", variable, std::mem::size_of_val(&variable));
    let variable = 121;
    println!("Variable = {}, length = {}", variable, std::mem::size_of_val(&variable));

    let decimal: f32 = 2.5;
    println!("Decimal = {}", decimal);

    let boolean = false;
    println!("Boolean = {}, length = {}", boolean, std::mem::size_of_val(&boolean));

    let letter = 'C';
    println!("Char Length = {}", std::mem::size_of_val(&letter));

    println!("PI = {}", PI);
    println!("Global Variable = {}", GLOBAL_VARIABLE);

    let this_string = "my name";
    println!("{this_string}, length = {}", std::mem::size_of_val(&this_string))
}

fn conditions() {
    let age: u8 = 24;
    let responsible_authorize = true;
    let is_big = age >= 18;

    if is_big {
        println!("Can enter!")
    } else if age > 16 && responsible_authorize {
        println!("Can enter with responsible sign!")
    } else {
        println!("Can't enter!")
    }

    let condition = if is_big { "big" } else { "small" };
    println!("Is {condition} of age");

    let language = "PHP";
    let purpose = match language {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Unknown"
    };

    println!("The Purpose of {language} is {purpose}")
}

fn loops() {
    let multiplier: u8 = 5;
    let mut counter: u8 = 0;

    while counter < 10 {
        counter += 1;

        if counter == 5 {
            continue;
        }

        println!("{} x {} = {}", multiplier, counter, multiplier * counter);
    }

    println!("---------------------");

    counter = 0;
    loop {
        counter += 1;

        println!("{} x {} = {}", multiplier, counter, multiplier * counter);
        if counter == 10 {
            break;
        }
    }

    println!("---------------------");

    for count in 1..=10 {
        println!("{} x {} = {}", multiplier, count, multiplier * count);
    }
}

fn ownership() {
    let mut a_string = String::from("Pedro");

    // was moved and ownership is other
    // stole(a_string);
    stole(&mut a_string);
    println!("{a_string}");
}

fn stole(string: &mut String) {
    string.push_str(" Augusto");

    println!("{string}");
}

fn pattern_matching() {
    for x in 1..=20 {
        println!("{} : {}", x, match x {
            1 => "Pouco",
            2 | 3 => "Um Pouco",
            4..=10 => "Um bocado",
            _ if x % 2 == 0 => "Uma boa quantidade",
            _ => "Muito"
        })
    }
}

fn errors() {
    match result() {
        Ok(s) => println!("String de sucesso = {s}"),
        Err(e) => println!("Código de erro = {e}"),
    }
}

fn result() -> Result<String, u8> {
    Ok(String::from("Tudo deu certo"))
}

fn main() {
    scope();
    shadow();
    conditions();
    loops();

    println!("SUM = {}", sum(2, 3));

    ownership();
    pattern_matching();
    errors();
}
