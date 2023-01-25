# gt
The OpenAI GPT model allows for easy access to language translation from the command line with a personal touch.

## Installation

You can install gt by running the following command in your terminal.

```
curl -fsSL https://raw.githubusercontent.com/fukumone/gt/main/install.sh | sh -
```

Or, please download the binary that is compatible with your OS from [here](https://github.com/fukumone/gt/releases/tag/v0.1.0).

## Usage

1. Get your API key from OpenAI by setting the `OPENAI_API_KEY` environment variable. Example: `export OPENAI_API_KEY=xxxxx`
For more information, visit the provided [link](https://openai.com/).

2. To set the language you want to translate to, use the `GT_LANGUAGE` environment variable. For instance, if you want to translate to French, type `export GT_LANGUAGE=French` By default, the GT_LANGUAGE is set to translate to English in any language.

3. Try it out by opening the command line and typing `gt -t "text"`


#### example
```

# Translated from English to French
$ export GT_LANGUAGE=French
$ gt -t "I love Tokyo greatly, it is wonderful to be here."
# => J'aime Tokyo grandement, c'est merveilleux d'être ici.

# Translated from Spanish to Arabic
$ export GT_LANGUAGE=Arabic
$ gt -t "Me gusta mucho Tokio; es maravilloso estar aquí."
# => الحب لطوكيو كثيرا؛ هو مدهش أن نكون هنا.

# Translated from Japanese to Italian
$ export GT_LANGUAGE=Italian
$ gt -t "私は東京がとても好きです。素晴らしいところです。"
# => Mi piace molto Tokyo. È un posto meraviglioso.

# Translated from Mandarin to Portuguese
$ export GT_LANGUAGE=Portuguese
$ gt -t "我深深地爱上了东京，来到这里真是太美妙了。"
# => Eu me apaixonei profundamente por Tóquio, e estar aqui é realmente maravilhoso.
```

## License

This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information.
