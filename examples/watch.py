from lp1379 import rswatcher


def handle_event(event: dict) -> None:
    print(f"{event['kind']:>7}  {', '.join(event['paths'])}")


rswatcher("./", handle_event)
