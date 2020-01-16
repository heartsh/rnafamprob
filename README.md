# Zprob Program, which Predicts Average Posterior Probabilities on RNA Structural alignment
# Installation
This project has been written in mainly Rust, a systems programming language.
So first, you need to install the Rust compiler, package manager, and standard library.
Visit [the Rust website](https://www.rust-lang.org) to see more about this language.
You can install the components with one line as follows:
```bash
$ curl https://sh.rustup.rs -sSf | sh
```
The installation is arranged by [Rustup](https://github.com/rust-lang-nursery/rustup.rs), which enables to switch easily a compiler in use.
Now you can install the ZProb program: 
```bash
$ cargo install zprob
```
Check if the program has been installed properly:
```bash
$ zprob # Its available command options will be displayed.
```
You can run the program with a prepared test set of RNA homologous sequences:
```bash
$ git clone https://github.com/heartsh/zprob && cd zprob
$ cargo test --release -- --nocapture
```
After the test, figures shown in the paper of the program can be reproduced:
```bash
$ cd src
$ ./run_all.py # Install required python packages to the reproduction. Saved figures will appear at the "../assets/images" directory.
```

# Author
[Heartsh](https://github.com/heartsh)

# License
Copyright (c) 2018 Heartsh  
Licensed under [the MIT license](http://opensource.org/licenses/MIT).
