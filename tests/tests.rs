use rust_succ::to_string;
use serde::Serialize;

#[test]
fn test_struct() {
    #[derive(Serialize)]
    struct Test<'a> {
        int: u32,
        float: f32,
        tuple: (String, &'a str),
        vec: Vec<u8>,
        test2: Test2
    }

    #[derive(Serialize)]
    struct Test2 {
        int2: u32,
        float2: f64
    }

    let test = Test {
        int: 1,
        float: 2.5,
        tuple: (String::from("Hello"), "  Stringified  "),
        vec: vec![1, 2, 3, 4, 5],
        test2: Test2 {
            int2: 2,
            float2: 2.75
        }
    };

    let expected = r#"Test:
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
"#;
    let string = to_string(&test).unwrap();
    println!("```\n{string}\n```\n-----\n```\n{expected}```");
    
    assert_eq!(string, expected);
}

