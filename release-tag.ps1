$version = (cargo pkgid) -replace '.*#',''
git tag "v$version"
git push origin "v$version"