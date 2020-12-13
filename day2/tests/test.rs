extern crate day2;

#[cfg(test)]
mod test_module {

    use day2::functions;
    #[test]
    fn test1() {
        let sut = functions::PasswordWithRules::parse("11-15 d: ddddddddddddddjd");
        
        assert_eq!(sut.password, String::from("ddddddddddddddjd"));
        assert_eq!(sut.rule_character, 'd');
        assert_eq!(sut.rule_min_amount, 11);
        assert_eq!(sut.rule_max_amount, 15);
        assert_eq!(sut.is_valid(), true);
        assert_eq!(sut.is_valid2(), false);
    }

    #[test]
    fn test2() {
        let sut = functions::PasswordWithRules::parse("1-3 a: abcde");

        assert_eq!(sut.password, String::from("abcde"));
        assert_eq!(sut.rule_character, 'a');
        assert_eq!(sut.rule_min_amount, 1);
        assert_eq!(sut.rule_max_amount, 3);
        assert_eq!(sut.is_valid(), true);
        assert_eq!(sut.is_valid2(), true);
    }
    
    #[test]
    fn test3() {
        let sut = functions::PasswordWithRules::parse("1-3 b: cdefg");

        assert_eq!(sut.password, String::from("cdefg"));
        assert_eq!(sut.rule_character, 'b');
        assert_eq!(sut.rule_min_amount, 1);
        assert_eq!(sut.rule_max_amount, 3);
        assert_eq!(sut.is_valid(), false);
        assert_eq!(sut.is_valid2(), false);
    }

    #[test]
    fn test4() {
        let sut = functions::PasswordWithRules::parse("2-9 c: ccccccccc");

        assert_eq!(sut.password, String::from("ccccccccc"));
        assert_eq!(sut.rule_character, 'c');
        assert_eq!(sut.rule_min_amount, 2);
        assert_eq!(sut.rule_max_amount, 9);
        assert_eq!(sut.is_valid(), true);
        assert_eq!(sut.is_valid2(), false);
    }

    
}
