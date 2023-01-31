## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)

## week2 rust demo
Generate a random number between 1 and 100

Prompt the player to enter a guess

After input, the program prompts to guess whether it is bigger or smaller

If correct, print a congratulatory message and the program exits

There are a few things to note here:

1. Cargo.toml must add dependencies of the rand package

2. Introduce rand::Rng and std::cmp::Ordering

3. Rust allows another variable to be defined with the same name after one variable, but the two variables need to be assigned and connected

4.match matches with Ordering type (similar to switch in java)

The things to note here are:

1. loop{} for wireless loop

2. The rust language is the same as other languages ​​break; jump out of the loop continue; directly execute the next loop

3. Error handling in rust generally uses match