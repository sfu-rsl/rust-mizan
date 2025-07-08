import logging
from pathlib import Path
from typing import Optional
from rich.console import Console
from rich.logging import RichHandler
from rich.progress import Progress, SpinnerColumn, TextColumn


console = Console()


class MizanLogger:
    def __init__(self, name: str = "mizan-cli"):
        self.logger = logging.getLogger(name)
        self._setup_handlers()

    def _setup_handlers(self):
        self.logger.handlers.clear()

        console_handler = RichHandler(
            console=console,
            show_time=True,
            show_path=False,
            rich_tracebacks=True,
        )
        console_handler.setFormatter(logging.Formatter("%(message)s"))
        self.logger.addHandler(console_handler)

        self.logger.setLevel(logging.INFO)

    def add_file_handler(self, log_file: Path):
        log_file.parent.mkdir(parents=True, exist_ok=True)

        file_handler = logging.FileHandler(log_file)
        file_handler.setFormatter(
            logging.Formatter("%(asctime)s - %(name)s - %(levelname)s - %(message)s")
        )
        self.logger.addHandler(file_handler)

    def set_level(self, level: str):
        numeric_level = getattr(logging, level.upper(), logging.INFO)
        self.logger.setLevel(numeric_level)

    def info(self, msg: str):
        self.logger.info(msg)

    def warning(self, msg: str):
        self.logger.warning(msg)

    def error(self, msg: str):
        self.logger.error(msg)

    def debug(self, msg: str):
        self.logger.debug(msg)

    def success(self, msg: str):
        """Log success message with green color."""
        console.print(f"[green]✓[/green] {msg}")


class ProgressManager:
    """Manages progress bars for operations."""

    def __init__(self):
        self.progress = Progress(
            SpinnerColumn(),
            TextColumn("[progress.description]{task.description}"),
            console=console,
        )

    def __enter__(self):
        self.progress.__enter__()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.progress.__exit__(exc_type, exc_val, exc_tb)

    def add_task(self, description: str, total: Optional[int] = None):
        """Add a new task to track."""
        return self.progress.add_task(description, total=total)

    def update(self, task_id, advance: int = 1):
        """Update task progress."""
        self.progress.update(task_id, advance=advance)


# Global logger instance
_logger: Optional[MizanLogger] = None


def get_logger() -> MizanLogger:
    global _logger
    if _logger is None:
        _logger = MizanLogger()
    return _logger


def init_logger(
    name: str = "mizan-cli", level: str = "INFO", log_file: Optional[Path] = None
) -> MizanLogger:
    global _logger
    _logger = MizanLogger(name)
    _logger.set_level(level)

    if log_file:
        _logger.add_file_handler(log_file)

    return _logger
