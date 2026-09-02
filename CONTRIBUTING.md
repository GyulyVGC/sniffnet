# Contributing Manifesto

To keep Sniffnet quality high, we're very selective about submitted contributions.<br>
For this reason, you're encouraged to read the following points before submitting a pull request (PR):
1. purely LLM-generated code contributions are strongly discouraged and will most likely be rejected: you must understand and be able to defend every line you submit, and disclose AI assistance if any was used in the process
2. before starting to work on a feature, make sure the corresponding issue isn't already assigned to someone else, and prefer picking up issues that are labeled with the tag "[help wanted](https://github.com/GyulyVGC/sniffnet/labels/help%20wanted)"
3. in case there's no GitHub issue for the feature or fix you want to work on, please open an issue: if it's a feature we can discuss it and give you feedback; if it's a bug make sure the problem is real, reproducible, not already reported, and provide as much information as possible (including screenshots if applicable)
4. reuse existing code and libraries where possible, and keep your PR small and focused, prioritizing quality over quantity
5. if you end up adding a UI-facing sentence, make sure to internationalize it by adding a method to the `src/translations` module with the corresponding English translation (only add other languages if you natively speak them)
6. if you end up modifying the `Sniffer` struct, include the new field in the `Conf` struct if it has to be persisted across runs of the app, or else consider cleaning it up in `Sniffer::reset()` if it has to be reset at every capture session
7. if you end up modifying or creating a library in the `lib` folder, make sure to bump its version, update its own `CHANGELOG.md`, and update `Cargo.toml`'s dependencies and workspace members accordingly
8. include unit tests to assert the implementation is sound (if applicable), and make sure that `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --all -- --check` all pass
9. be sure to update the `CHANGELOG.md` file `[UNRELEASED]` section with a one-line description of the change, including a link to the corresponding PR and issue if applicable (following the format of the existing entries)
10. be mindful that reviewing the PR may take a while, especially if it introduces substantial changes
11. be mindful that the contribution can still be rejected at our discretion (even if it satisfies all the points above) if it doesn't align with the project's vision or if it introduces unnecessary complexity

You can read the [_Build from source_](https://github.com/GyulyVGC/sniffnet/wiki/Build-from-source) Wiki page to learn how to set up the development environment, and the [_Code of Conduct_](https://github.com/GyulyVGC/sniffnet/blob/main/CODE_OF_CONDUCT.md) for the expected behavior in the community.

Despite our strictness, don't be afraid to share your thoughts: all proposals to enlarge or improve Sniffnet are warmly welcomed!
