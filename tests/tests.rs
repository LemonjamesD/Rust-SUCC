use rust_succ::to_string;
use serde::Serialize;
use std::collections::HashMap;

#[test]
fn test_struct() {
    #[derive(Serialize)]
    struct Test<'a> {
        int: u32,
        float: f32,
        tuple: (String, &'a str),
        vec: Vec<u8>,
        test2: Test2,
        funny: HashMap<String, u32>
    }

    #[derive(Serialize)]
    struct Test2 {
        int2: u32,
        float2: f64
    }

    let mut hash = HashMap::new();
    hash.insert("Cool".to_string(), 10);

    let test = Test {
        int: 1,
        float: 2.5,
        tuple: (String::from("Hello"), "  Stringified  "),
        vec: vec![1, 2, 3, 4, 5],
        test2: Test2 {
            int2: 2,
            float2: 2.75
        },
        funny: hash
    };

    let expected = r#"Test: # Gen: Test Struct
    int: 1
    float: 2.5
    tuple: # Gen: Tuple
        - Hello
        - "  Stringified  "
    vec: # Gen: Vec
        - 1
        - 2
        - 3
        - 4
        - 5
    test2: # Gen: Test2 Struct
        int2: 2
        float2: 2.75
    funny: # Gen: HashMap
        Cool: 10
"#;
    let string = to_string(&test).unwrap();
    println!("```\n{string}```\n-----\n```\n{expected}```");
    
    assert_eq!(string, expected);
}

