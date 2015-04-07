pub fn test() {
    let test1 = 
        "(schedule \"test1\"
             (program (local \"~/htpc/Videos/fsn.webm\") 
                 (tags media_type=\" anime\" studio=\"Studio Deen\" airdate=\"2014-11-15\")
                 (instr (play 00:00:01 00:10:00) 
                     (program 
                        ( network \"https://www.youtube.com/watch?v=foo\" 
                        )
                        (tags ) (instr (play 00:01:00))
                     )
                     (play 00:10:00)
                 )
             )
             (program (local \"~/htpc/Music/Gorillaz/Gorillaz/Punk.ogg\" ) 
                      (tags artist=\"Gorillaz\") (instr (play )))
         )";
    println!("{}", test1);
    let mut test2 = match super::parse::parse(test1) {
        Ok(res) => { 
            println!("{}", res);
            res
        },
        Err(f) => { 
            println!("Error: {}", f);
            return
        }
    };

    test2.change_name("test2".to_string());

    let test3 = test2.to_string();
    println!("{}", test3);
    let mut test4 = match super::parse::parse(&test3) {
        Ok(res) => res,
        Err(f) => {
            println!("{}", f);
            panic!("Crap!")
        }
    };

    assert_eq!(test2,test4);

    test4.modify_program(0).unwrap().tags.director=Some("John Wayne".to_string());

    println!("{}", test4);

    println!("Success!");
}
