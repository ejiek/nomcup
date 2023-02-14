// A library to parse PKGBUILD files with Rust and Nom

use nom::IResult;

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

    fn parse_pkgname(input: &str) -> IResult<&str, String> {
        let (input, _) = nom::bytes::complete::tag("pkgname=")(input)?;
        let (input, pkgname) = nom::bytes::complete::take_until("\n")(input)?;
        Ok((input, pkgname.to_string()))
    }

    fn parse_pkgver(input: &str) -> IResult<&str, String> {
        let (input, _) = nom::bytes::complete::tag("pkgver=")(input)?;
        let (input, pkgver) = nom::bytes::complete::take_until("\n")(input)?;
        Ok((input, pkgver.to_string()))
    }

    fn parse_pkgrel(input: &str) -> IResult<&str, String> {
        let (input, _) = nom::bytes::complete::tag("pkgrel=")(input)?;
        let (input, pkgrel) = nom::bytes::complete::take_until("\n")(input)?;
        Ok((input, pkgrel.to_string()))
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
        assert_eq!(input, "\n");
        assert_eq!(pkgname, expected);
    }
}
