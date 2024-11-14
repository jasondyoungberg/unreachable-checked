`unreachable_checked!` is a macro that will tigger a linker error if it
can't be optimized out.

You may need to increase the optimization level to for this to work
correctly

## Usage

```rust
use unreachable_checked::unreachable_checked;

fn main() {
    let a = [1, 2, 3, 4];
    for i in 0..4 {
        println!("{}", a.get(i).unwrap_or_else(|| unreachable_checked!()))
    }
}
```
