extern crate day4;

#[cfg(test)]
mod test_module {
    use day4::functions;
    #[test]
    fn test1() {
        let sut = functions::Passport::parse(
            r"hgt:159cm
            pid:561068005 eyr:2025 iyr:2017 cid:139 ecl:blu hcl:#ceb3a1
            byr:1940",
        );

        assert_eq!(sut.birth_year, "1940");
        assert_eq!(sut.country_id, "139");
        assert_eq!(sut.expiration_year, "2025");
        assert_eq!(sut.eye_color, "blu");
        assert_eq!(sut.hair_color, "#ceb3a1");
        assert_eq!(sut.height, "159cm");
        assert_eq!(sut.issue_year, "2017");
        assert_eq!(sut.passport_id, "561068005");
        assert_eq!(sut.is_valid(), true);
        assert_eq!(sut.is_valid2(), true);
    }
    #[test]
    fn test2() {
        let sut = functions::Passport::parse(
            r"iyr:2014
        byr:1986 pid:960679613 eyr:2025 ecl:hzl",
        );

        assert_eq!(sut.birth_year, "1986");
        assert_eq!(sut.country_id, "");
        assert_eq!(sut.expiration_year, "2025");
        assert_eq!(sut.eye_color, "hzl");
        assert_eq!(sut.hair_color, "");
        assert_eq!(sut.height, "");
        assert_eq!(sut.issue_year, "2014");
        assert_eq!(sut.passport_id, "960679613");
        assert_eq!(sut.is_valid(), false);
        assert_eq!(sut.is_valid2(), false);
    }
    #[test]
    fn test3() {
        let sut = functions::Passport::parse(
            r"eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
        );

        assert_eq!(sut.birth_year, "1926");
        assert_eq!(sut.country_id, "100");
        assert_eq!(sut.expiration_year, "1972");
        assert_eq!(sut.eye_color, "amb");
        assert_eq!(sut.hair_color, "#18171d");
        assert_eq!(sut.height, "170");
        assert_eq!(sut.issue_year, "2018");
        assert_eq!(sut.passport_id, "186cm");
        assert_eq!(sut.is_valid(), true);
        assert_eq!(sut.is_valid2(), false);
    }
    #[test]
    fn test4() {
        let sut = functions::Passport::parse(
            r"iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946",
        );

        assert_eq!(sut.is_valid(), true);
        assert_eq!(sut.is_valid2(), false);
    }

    #[test]
    fn test5() {
        let sut = functions::Passport::parse(
            r"hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
        );

        assert_eq!(sut.is_valid(), true);
        assert_eq!(sut.is_valid2(), false);
    }
    #[test]
    fn test6() {
        let sut = functions::Passport::parse(
            r"hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007",
        );

        assert_eq!(sut.is_valid(), true);
        assert_eq!(sut.is_valid2(), false);
    }

    #[test]
    fn test7() {
        let sut = functions::Passport::parse(
            r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
            hcl:#623a2f",
        );

        assert_eq!(sut.is_valid(), true);
        assert_eq!(sut.is_valid2(), true);
    }

    #[test]
    fn test8() {
        let sut = functions::Passport::parse(
            r"eyr:2029 ecl:blu cid:129 byr:1989
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
        );

        assert_eq!(sut.is_valid(), true);
        assert_eq!(sut.is_valid2(), true);
    }

    #[test]
    fn test9() {
        let sut = functions::Passport::parse(
            r"
            hcl:#888785
            hgt:164cm byr:2001 iyr:2015 cid:88
            pid:545766238 ecl:hzl
            eyr:2022",
        );

        assert_eq!(sut.is_valid(), true);
        assert_eq!(sut.is_valid2(), true);
    }

    #[test]
    fn test10() {
        let sut = functions::Passport::parse(
            r"iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        );

        assert_eq!(sut.is_valid(), true);
        assert_eq!(sut.is_valid2(), true);
    }
}
