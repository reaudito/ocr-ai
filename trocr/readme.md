# candle-trocr

```bash
cargo run -p trocr --release -- --which large --image trocr/src/assets/trocr.png
```

`TrOCR` is a transformer OCR Model. In this example it is used to
transcribe image text. See the associated [model
card](https://huggingface.co/microsoft/trocr-base-printed) for details on
the model itself.

Supported models include:

- `--which base`: small handwritten OCR model.
- `--which large`: large handwritten OCR model.
- `--which base-printed`: small printed OCR model.
- `--which large-printed`: large printed OCR model.

## Running an example

```bash
cargo run --example trocr --release -- --image candle-examples/examples/trocr/assets/trocr.png
cargo run --example trocr --release -- --which large --image candle-examples/examples/trocr/assets/trocr.png
cargo run --example trocr --release -- --which base-printed --image candle-examples/examples/trocr/assets/noto.png
cargo run --example trocr --release -- --which large-printed --image candle-examples/examples/trocr/assets/noto.png
```

### Outputs

```
industry , Mr. Brown commented icily . " Let us have a
industry , " Mr. Brown commented icily . " Let us have a
THE QUICK BROWN FOR JUMPS OVER THE LAY DOG
THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG
```




## hf_hub

hf_hub Rust set folder path

To set the folder path for the HF Hub in Rust, you can use the `HF_HOME` environment variable. This variable configures where huggingface_hub will locally store data, including the cache and token. By default, it is set to `~/.cache/huggingface` unless `XDG_CACHE_HOME` is set.

You can set the `HF_HOME` environment variable to a custom path before running your Rust application. For example:

```bash
export HF_HOME="/path/to/custom/folder"
```

This custom folder will be used for storing the cache and token locally. The `HF_HOME` environment variable is also used by the Python package `huggingface_hub` for the same purpose

To configure where repositories from the Hub will be cached locally, you can set the `HF_HUB_CACHE` environment variable. By default, it is set to `$HF_HOME/hub` (e.g., `~/.cache/huggingface/hub` by default)

```bash
export HF_HUB_CACHE="/path/to/custom/cache/folder"
```

This will ensure that the downloaded files are stored in the specified folder, maintaining their original file structure


