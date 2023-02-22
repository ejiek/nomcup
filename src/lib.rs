pub mod ast;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pkgbuild() {
        let input = r#"pkgname=rust
pkgver=1.51.0
pkgrel=1
# comment out of nowhere
arch=('x86_64' 'aarch64')
"#;
        let parsed_pkgbuild = ast::pkgbuild::PkgBuild::new(input);
        assert_eq!(parsed_pkgbuild.to_string(), input);
    }
}
