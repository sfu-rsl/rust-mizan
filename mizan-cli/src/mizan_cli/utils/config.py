from pathlib import Path
from typing import Optional
from pydantic import BaseModel, Field
import json


class MizanConfig(BaseModel):
    log_level: str = Field("INFO", description="Logging level")
    log_file: Optional[Path] = Field(None, description="Log file path")


class ConfigManager:
    def __init__(self, config_file: Optional[Path] = None):
        self.config_file = config_file or self._find_config_file()
        self.config = self._load_config()

    def _find_config_file(self) -> Optional[Path]:
        """Find configuration file in home directory."""
        config_path = Path.home() / ".config" / "mizan" / "config.json"
        return config_path if config_path.exists() else None

    def _load_config(self) -> MizanConfig:
        """Load configuration from file only."""
        config_data = {}

        if self.config_file and self.config_file.exists():
            with open(self.config_file, "r") as f:
                config_data = json.load(f)

        return MizanConfig(**config_data)

    def save_config(self, path: Optional[Path] = None):
        """Save current configuration to file."""
        save_path = (
            path
            or self.config_file
            or (Path.home() / ".config" / "mizan" / "config.json")
        )
        save_path.parent.mkdir(parents=True, exist_ok=True)

        with open(save_path, "w") as f:
            json.dump(
                self.config.model_dump(exclude_none=True), f, indent=2, default=str
            )


_config_manager: Optional[ConfigManager] = None


def get_config() -> ConfigManager:
    global _config_manager
    if _config_manager is None:
        _config_manager = ConfigManager()
    return _config_manager


def init_config(config_file: Optional[Path] = None) -> ConfigManager:
    global _config_manager
    _config_manager = ConfigManager(config_file)
    return _config_manager
