class WindowManager:
    def __init__(self) -> None: ...
    def start(self, debug: bool = False) -> None: ...
    def get_port(self) -> int: ...
    def send_html(
        self, html_str: str = "", html_path: str = "", title: str = ""
    ) -> None: ...
    def close(self, reset: bool = False) -> None: ...
