#![cfg(test)]

mod m001 {
    #[test]
    fn test() {
        let a = Some("a");
        let b = Some("b");
        let c = Some("c");
        let d = Some("d");

        let a_or_d = a.unwrap_or(d.unwrap());
        let default_not = || b.iter().next();
        let c_mapped = |_| Some(&a_or_d);
        let result = c.map_or_else(default_not, c_mapped);

        eprintln!("{:?}", result);

        eprintln!("{:?}", &("01234")[1..4]);

        eprintln!("{}", true);
    }
}
