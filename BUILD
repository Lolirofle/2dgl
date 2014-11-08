git checkout master
cargo doc -j 4 --no-default-features --no-deps
git checkout gh-pages

Copy /target/doc/2dgl/* -> /doc/

Regex replace all files that matches /doc/*:
	'(../)+src/2dgl/home/edwin/Programming/Rust/2dgl/(.*).html#(.*)'
	to
	'https://github.com/Lolirofle/2dgl/blob/master/\2#\3'
