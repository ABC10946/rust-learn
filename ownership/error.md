ムーブ元の値を参照しようとしたときのエラー

```
D:\nextcloud\Local\Programs\rust-learn\ownership>rustc ownership.rs
warning: unused variable: `s2`
 --> ownership.rs:3:6
  |
3 |     let s2 = s1;
  |         ^^ help: if this is intentional, prefix it with an underscore: `_s2`
  |
  = note: `#[warn(unused_variables)]` on by default

error[E0382]: borrow of moved value: `s1`
 --> ownership.rs:4:22
  |
2 |     let s1 = String::from("Hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |     println!("s1 = {}", s1);
  |                         ^^ value borrowed here after move

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0382`.
```