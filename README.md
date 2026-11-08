# listening_post_1379

<img src="imgs/lp1379.png" width="100" alt="lp1379">

This repository mainly acts as a bridge between [notify](https://github.com/notify-rs/notify) and python.
Ultimate goal is a _lightweight_ python importable function that watches a directory for changes.

```python
from lp1379 import rswatcher

rswatcher("some/dir", print)
```

`rswatcher` watches the directory recursively and blocks until interrupted (Ctrl-C).
The callback receives one dict per event:

```python
{"kind": "modify", "paths": ["some/dir/file.txt"]}
```

where `kind` is one of `create`, `modify`, `remove`, `access` or `other`.
A path that doesn't exist raises an `OSError`.

More example usage is available under the [examples](examples) folder.

## License

MIT

## Installation

Installable via pypi using pip:

```bash
pip install lp1379
```

or with uv:

```bash
uv pip install lp1379
```

The development version can be installed and used via pixi:

```bash
git clone git@github.com:WardDeb/listening_post_1379.git
pixi run python
```

Run the tests with:

```bash
pixi run test
```