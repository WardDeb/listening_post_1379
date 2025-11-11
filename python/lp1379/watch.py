from pathlib import Path

def watch(path: str = None) -> None:
    '''
    Watch either the current directory (no path given), or the given path.
    If the latter, it needs to exist.
    '''
    if path is None:
        path = Path.cwd()
    else:
        path = Path(path)
    if not path.exists():
        raise FileNotFoundError(f"Given path does not exist: {path}")
    print(f"Watching path: {path}")