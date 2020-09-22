# Notes

As for REST API enabling libraries, the one with the most readily available information was Rocket, and that was the main reason Rocket was chosen for this app. However, it requires the `nightly` verison of Rust and, currently, it has no capabilities for asynchronous functions to work as REST handlers, which is why the `Client` created in `download.rs` is `blocking` instead of asynchronous.

Ideas for improvement:

* Adding actual testing.
* Check that parameters are compatible (city name with the country code, for example).
* Adding some permanence, so requests closer in time than 10 minutes use a local cache'd copy.
  * Either keeping the parameters of the last cache'd call too, to manually convert temperature units if the user asks for a different one, or change queries to always get them in Kelving and always do the manual calculations when presenting the info to the user.
* Adding the alternative API calls available at OpenWeather.
* Reusing the Http Client across API calls.

# Consulted Documentation

* [Rust crash course](https://www.youtube.com/watch?v=zF34dRivLOw), [Learn Rust in Y minutes](https://learnxinyminutes.com/docs/rust/), some entries in the [Rust Programming Tutorials](https://www.youtube.com/playlist?list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL) list, and [How to build a command line app in Rust](https://www.youtube.com/watch?v=DQnLQznJK1Q).
* Official docs for `tokio`, `reqwest`, `rocket` and the different `serde` utilities.
* [Part 1](https://medium.com/swlh/demystifying-closures-futures-and-async-await-in-rust-part-1-closures-97e531e4dc50), [part 2](https://levelup.gitconnected.com/demystifying-closures-futures-and-async-await-in-rust-part-2-futures-abe95ab332a2) and [part 3](https://medium.com/@alistairisrael/demystifying-closures-futures-and-async-await-in-rust-part-3-async-await-9ed20eede7a4) of "Demystifying Closures, Futures, and async-await in Rust"
