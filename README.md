# docx-template-filler

This small application is intended as a simple office/administration tool. It should help when one needs to create number of DOCX documents from a common template. This usually corresponds to changing only small bits for every output document - e.g. person name, address, phone number, etc. While doing this manually is possible, it can be tiring process. *docx-template-filler* should help and speed up this task.

It does this in several steps:

  - it reads a selected DOCX template document into memory
  - identifies so called "tokens" (variables/placeholders) in the template document
  - takes the input data entered by user
  - generates standalone new DOCX file for each line of the input data, while replacing all the identified tokens with specific input values

For details on usage of application, and an example, please see the [manual](manual.md).

Currently supported languages in the application are: English (default), Czech, Russian.

## installation

Whole application consists of a single file: "`docx-template-filler.exe`".

It should run on "any" recent windows. It does not need to be installed, and does not have any external dependencies.

I recommend to put the application executable into some standalone folder, as it places all the generated DOCX files next to where it is located.

## build - "docx-template-filler.exe"

I do not distribute / provide the application ready for use - as a windows executable. You have to either build it yourself, or ask friend / acquaintance able to do so for help.

cRustaceans supposedly know what to do. To get the rough idea what this means for non-rust users:

- installing Rust programming language tool-chain as instructed on the Rust's [homepage][rust] )
- "checking out" this repository into your computer
- running the "`cargo build --release`" command in the checked out repository folder
- finding the resulting file `docx-template-filler.exe` inside the `target/release` sub-folder<br/>(file size should be around 2.0 MB)
- copying the single executable file anywhere you want to use it / keep it

and, optionally (assuming you will not want to build the application again/repeatedly):

- removing the whole checked out repository folder
- uninstalling whatever you may have needed to install in previous steps

## implementation details

I have written this application in [Rust][rust] programming language, primarily as an attempt to learn / get to know the language. It uses several Rust crates (see `Cargo.toml` file) for its work-flow, that allowed to do its work fairly easily/quickly.

Few points of interest:

- UI is created using the [native-windows-gui][nwg] crate, and thus builds only for windows target platform
- support for UI language is done using [fluent-templates][ft] create, and does only very basic translations, thus might sound a bit rough in specific languages
- app loads the whole DOCX template file into memory, and thus might not be suitable to work with huge DOCX files that have huge file-size / page count / embedded graphical/audio data etc.

# thank you

from the Rust beginner, to the people that helped out, clarified things, and responded to questions:

- to the Rust [community][forum]
- to the authors of the used / referenced creates

<!-- references -->
[rust]: https://www.rust-lang.org/
[forum]: https://users.rust-lang.org/
[nwg]: https://github.com/gabdube/native-windows-gui
[ft]: https://github.com/XAMPPRocky/fluent-templates
