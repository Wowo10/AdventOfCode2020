extern crate day5;

#[cfg(test)]
mod test_module {
    use day5::functions;
    #[test]
    fn test1() {
        let sut = functions::Seat::create("FBFBBFFRLR");
        assert_eq!(sut.row(), 44);
        assert_eq!(sut.column(), 5);
        assert_eq!(sut.id(), 357);
    }
    #[test]
    fn test2() {
        let sut = functions::Seat::create("BFFFBBFRRR");
        assert_eq!(sut.row(), 70);
        assert_eq!(sut.column(), 7);
        assert_eq!(sut.id(), 567);
    }
    #[test]
    fn test3() {
        let sut = functions::Seat::create("FFFBBBFRRR");
        assert_eq!(sut.row(), 14);
        assert_eq!(sut.column(), 7);
        assert_eq!(sut.id(), 119);
    }
    #[test]
    fn test4() {
        let sut = functions::Seat::create("BBFFBBFRLL");
        assert_eq!(sut.row(), 102);
        assert_eq!(sut.column(), 4);
        assert_eq!(sut.id(), 820);
    }
}
