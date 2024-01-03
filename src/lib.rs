// A library to parse PKGBUILD files with Rust and Nom

use nom::{
    branch::{alt, permutation},
    bytes::complete::{tag, take_until, take_while1},
    character::complete::space0,
    multi::separated_list1,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct PkgBuild {
    //mandatory fields
    pkgname: Vec<String>,
    pkgver: String,
    pkgrel: String,
    arch: Vec<String>,
}

impl PkgBuild {
    // parsing mandatory fields in any order
    pub fn parse(input: &str) -> IResult<&str, PkgBuild> {
        permutation((
            PkgBuild::parse_pkgname,
            PkgBuild::parse_pkgver,
            PkgBuild::parse_pkgrel,
            PkgBuild::parse_arch,
        ))(input)
        .map(|(next_input, (pkgname, pkgver, pkgrel, arch))| {
            (
                next_input,
                PkgBuild {
                    pkgname,
                    pkgver,
                    pkgrel,
                    arch,
                },
            )
        })
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

    pub fn parse_pkgname(input: &str) -> IResult<&str, Vec<String>> {
        let (input, _) = tag("pkgname")(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, _) = space0(input)?;
        alt((Self::parse_pkgname_multiple, Self::parse_pkgname_single))(input)
    }

    fn parse_pkgname_single(input: &str) -> IResult<&str, Vec<String>> {
        let (input, value) = take_until("\n")(input)?;
        let (_, value) = alt((Self::single_quoted, |v| Ok((v, v))))(value)?;
        let (input, _) = tag("\n")(input)?;
        Ok((input, vec![value.to_string()]))
    }

    fn parse_pkgname_multiple(input: &str) -> IResult<&str, Vec<String>> {
        let (input, _) = tag("(")(input)?;
        let (input, pkgnames) = take_until(")")(input)?;
        let (input, _) = tag(")")(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag("\n")(input)?;
        let names: Vec<&str> = pkgnames
            .split_whitespace()
            .map(|x| match Self::single_quoted(x) {
                Ok(v) => v.1,
                Err(_) => x,
            })
            .collect();
        Ok((input, names.iter().map(|s| s.to_string()).collect()))
    }

    pub fn parse_pkgver(input: &str) -> IResult<&str, String> {
        Self::parse_field(input, "pkgver")
    }

    pub fn parse_pkgrel(input: &str) -> IResult<&str, String> {
        Self::parse_field(input, "pkgrel")
    }

    pub fn parse_arch(input: &str) -> IResult<&str, Vec<String>> {
        let (input, _) = tag("arch")(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag("(")(input)?;
        let (input, arches) = take_until(")")(input)?;
        let (_, arches) = separated_list1(tag(" "), Self::single_quoted)(arches)?;
        let (input, _) = tag(")")(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag("\n")(input)?;
        Ok((input, arches.iter().map(|s| s.to_string()).collect()))
    }

    fn single_quoted(input: &str) -> IResult<&str, &str> {
        let (input, _) = tag("'")(input)?;
        let (input, value) = take_while1(|c| c != '\'')(input)?;
        let (input, _) = tag("'")(input)?;
        Ok((input, value))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pkgname() {
        let input = "pkgname=foo\n";
        let expected = vec!["foo"];
        let (input, pkgname) = super::PkgBuild::parse_pkgname(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(pkgname, expected);
    }

    #[test]
    fn pkgname_quoted() {
        let input = "pkgname='foo'\n";
        let expected = vec!["foo"];
        let (input, pkgname) = super::PkgBuild::parse_pkgname(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(pkgname, expected);
    }

    #[test]
    fn pkgname_multi() {
        let input = "pkgname=(foo bar)\n";
        let expected = vec!["foo", "bar"];
        let (input, pkgname) = super::PkgBuild::parse_pkgname(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(pkgname, expected);
    }

    #[test]
    fn pkgname_multi_quoted() {
        let input = "pkgname=('foo' 'bar')\n";
        let expected = vec!["foo", "bar"];
        let (input, pkgname) = super::PkgBuild::parse_pkgname(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(pkgname, expected);
    }

    #[test]
    fn pkgver() {
        let input = "pkgver=1.0\n";
        let expected = "1.0";
        let (input, pkgver) = super::PkgBuild::parse_pkgver(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(pkgver, expected);
    }

    #[test]
    fn pkgrel() {
        let input = "pkgrel=1\n";
        let expected = "1";
        let (input, pkgrel) = super::PkgBuild::parse_pkgrel(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(pkgrel, expected);
    }

    #[test]
    fn arch() {
        let input = "arch=('i686' 'x86_64')\n";
        let expected = vec!["i686".to_string(), "x86_64".to_string()];
        let (input, arch) = super::PkgBuild::parse_arch(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(arch, expected);
    }

    #[test]
    fn pkgbuild() {
        let input = "pkgname=foo\npkgver=1.0\npkgrel=1\narch=('i686' 'x86_64')\n";
        let expected = super::PkgBuild {
            pkgname: vec!["foo".to_string()],
            pkgver: "1.0".to_string(),
            pkgrel: "1".to_string(),
            arch: vec!["i686".to_string(), "x86_64".to_string()],
        };
        let (input, pkgbuild) = super::PkgBuild::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(pkgbuild, expected);
    }
}
