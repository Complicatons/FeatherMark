# FeatherMark fixture

This file exercises **strong text**, *emphasis*, [an external link](https://example.com), and `inline code`.

> FeatherMark is intentionally small: one document, one reading view.

## Lists and tasks

- A bullet
- Another bullet
  - A nested item

1. First
2. Second

- [x] Rendering works
- [ ] Editing can be tested

## Code

```rust
fn main() {
    println!("Hello from FeatherMark");
}
```

## Table

| Feature | Status |
| --- | --- |
| Tables | Ready |
| Task lists | Ready |

## Relative image

![A small local test image](images/feather-test.png)

---

Raw HTML is shown safely rather than executed:

<button onclick="alert('unsafe')">Unsafe HTML</button>
