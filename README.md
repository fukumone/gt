# gt
The OpenAI GPT model allows for easy access to language translation from the command line with a personal touch.

## Installation

You can install gt by running the following command in your terminal.

```
curl -fsSL https://raw.githubusercontent.com/fukumone/gt/main/install.sh | sh -
```

Or, please download the binary that is compatible with your OS from [here](https://github.com/fukumone/gt/releases/tag/v0.1.0).

## Usage

### 1. API key configuration
Get your API key from OpenAI by setting the `OPENAI_API_KEY` environment variable. 

Example: `export OPENAI_API_KEY=xxxxx`

For more information, visit the provided [link](https://openai.com/).

### 2. Specifying the translation language
To set the language you want to translate, use the **-l** option in the options and specify the language.

For example, `gt -t "Hello" -l Japanese`

If there is no value, it will be translated into **English by default**.

You can also specify it by setting a value in the environment variable **GT_LANGUAGE**.

For example, `export GT_LANGUAGE=Japanese && gt -t "Hello"`

### 3. Execute
Try it out by opening the command line and typing `gt -t "Hello!" -l Japanese`

### example

```
# Translated from English to French
$ gt -t "I love Tokyo greatly, it is wonderful to be here." -l French
# => J'aime Tokyo grandement, c'est merveilleux d'être ici.

# Translated from Spanish to Arabic
$ gt -t "Me gusta mucho Tokio; es maravilloso estar aquí." -l Arabic
# => الحب لطوكيو كثيرا؛ هو مدهش أن نكون هنا.

# Translated from Japanese to Italian
$ gt -t "私は東京がとても好きです。素晴らしいところです。" -l Italian
# => Mi piace molto Tokyo. È un posto meraviglioso.

# Translated from Mandarin to Portuguese
$ gt -t "我深深地爱上了东京，来到这里真是太美妙了。" -l Portuguese
# => Eu me apaixonei profundamente por Tóquio, e estar aqui é realmente maravilhoso.
```

Also, you can check the usage with the help command.

```
$ gt -t -help

Translates text

Usage: gt {translate|-t} [OPTIONS] <TEXT>

Arguments:
  <TEXT>  Text to translate

Options:
  -l, --language <LANGUAGE>  Language to use [default: English]
  -h, --help                 Print help
```

## License

This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information.
