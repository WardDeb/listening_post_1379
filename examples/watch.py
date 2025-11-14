from pathlib import Path
from lp1379.lp1379 import rswatcher

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

    def handle_event(event):
        print(event)

    rswatcher(str(path), handle_event)

watch()