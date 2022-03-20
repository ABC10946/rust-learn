fn main() {
	let s1 = String::from("Hello");
	let s2 = s1;
	// println!("s1 = {}", s1); // 動かない

	let s3 = s2.clone(); // deep copyされる。実行コストが高いことが明示的にソースコード上で示される。
	println!("s2 = {}, s3 = {}", s2, s3);
}