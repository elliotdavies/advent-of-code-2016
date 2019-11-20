echo "
[[bin]]
name = \"day$DAY\"
path = \"src/day$DAY.rs\"" >> problems/Cargo.toml

sed "s/\$DAY/$DAY/g" template.rs > "problems/src/day$DAY.rs"
touch "problems/src/day$DAY.txt"
