// A library to parse PKGBUILD files with Rust and Nom

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::space0,
    IResult,
};

#[derive(Debug, PartialEq)]
struct PkgBuild {
    //mandatory fields
    pkgname: String,
    pkgver: String,
    pkgrel: String,
    arch: Vec<String>,
}

impl PkgBuild {
    // parsing mandatory fields in any order
    fn parse(input: &str) -> IResult<&str, PkgBuild> {
        let (input, pkgname) = PkgBuild::parse_pkgname(input)?;
        let (input, pkgver) = PkgBuild::parse_pkgver(input)?;
        let (input, pkgrel) = PkgBuild::parse_pkgrel(input)?;
        let (input, arch) = PkgBuild::parse_arch(input)?;
        Ok((
            input,
            PkgBuild {
                pkgname,
                pkgver,
                pkgrel,
                arch,
            },
        ))
    }

    fn parse_field<'a>(input: &'a str, field: &str) -> IResult<&'a str, String> {
        let (input, _) = tag(field)(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, _) = space0(input)?;
        let (input, value) = take_until("\n")(input)?;
        let (input, _) = tag("\n")(input)?;
        Ok((input, value.to_string()))
    }

    fn parse_pkgname(input: &str) -> IResult<&str, String> {
        Self::parse_field(input, "pkgname")
    }

    fn parse_pkgver(input: &str) -> IResult<&str, String> {
        Self::parse_field(input, "pkgver")
    }

    fn parse_pkgrel(input: &str) -> IResult<&str, String> {
        Self::parse_field(input, "pkgrel")
    }

    fn parse_arch(input: &str) -> IResult<&str, Vec<String>> {
        let (input, _) = nom::bytes::complete::tag("arch=")(input)?;
        let (input, arch) = nom::bytes::complete::take_until("\n")(input)?;
        let arch = arch.split(" ").map(|s| s.to_string()).collect();
        Ok((input, arch))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pkgname() {
        let input = "pkgname=foo\n";
        let expected = "foo";
        let (input, pkgname) = super::PkgBuild::parse_pkgname(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(pkgname, expected);
    }

    fn pkgver() {
        let input = "pkgver=1.0\n";
        let expected = "1.0";
        let (input, pkgver) = super::PkgBuild::parse_pkgver(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(pkgver, expected);
    }

    fn pkgrel() {
        let input = "pkgrel=1\n";
        let expected = "1";
        let (input, pkgrel) = super::PkgBuild::parse_pkgrel(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(pkgrel, expected);
    }
}
