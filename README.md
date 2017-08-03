# Ketree

This is a small library that allows for the creation of trees that represent symbolic expressions using [Ketos](https://github.com/murarth/ketos).

## Building a Tree

Add the following to `Cargo.toml`:

```toml
[depenencies]
ketree = "0.4.1"
```

Then, to the crate root, add:

```rust
extern crate ketree;
```

You will then need to add [Ketos](https://github.com/murarth/ketos) to your project and write a struct that 
implements [ModuleLoader](https://docs.rs/ketos/0.10.0/ketos/module/trait.ModuleLoader.html). This struct 
can then be passed to an instance of [TreeBuilder](https://docs.rs/ketree/0.4.1/ketree/treebuilder/struct.TreeBuilder.html) 
to create a tree. Check tests for an [example](https://github.com/Wulfsta/ketree/blob/master/tests/build.rs).
