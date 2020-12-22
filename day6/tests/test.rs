extern crate day6;

#[cfg(test)]
mod test_module {
    use day6::functions;
    
    #[test]
    fn test1() {
        let sut = functions::Group::create("ax
        bx
        cx");
        assert_eq!(sut.all_answered('a'), false);
        assert_eq!(sut.any_answered('a'), true);
        assert_eq!(sut.all_answered('x'), true);
        assert_eq!(sut.any_answered('x'), true);
        assert_eq!(sut.all_answered('y'), false);
        assert_eq!(sut.any_answered('y'), false);
    }
}
