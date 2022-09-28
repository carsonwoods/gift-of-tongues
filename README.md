# Gift of Tongues
Or simply called "tongues", is a very small application written in Rust to quickly lookup the meaning of a word.
Currently, only english is supported, though this may change over time as more dictionaries are added.

    
## Installation Guide
To install, simply run `cargo install gift-of-tongues`.
  
## Usage
To use tongues, simply run:
```bash
$ tongues <word>
```

Doing so will retrieve the definition of the value found in the `<word>` parameter.


## FAQs
*Q: What languages are supported by tongues?"*

A: Currently, only english. This might change as the app evolves. To add a new language, an API or database would be required containing the information for that language.

## Acknowledgements
Dictionary data is provided by [https://dictionaryapi.dev](https://dictionaryapi.dev/)
