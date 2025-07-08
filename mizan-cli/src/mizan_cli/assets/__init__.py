import os
from pathlib import Path


def get_asset_path(asset_type: str, asset_name: str) -> Path:
    """Get the path to an asset file."""
    assets_dir = Path(__file__).parent
    asset_path = assets_dir / asset_type / asset_name

    if not asset_path.exists():
        raise FileNotFoundError(f"Asset not found: {asset_path}")

    return asset_path


def get_rustfmt_config(style: str) -> Path:
    """Get the path to a rustfmt configuration file."""
    valid_styles = ["compact", "expanded"]
    if style not in valid_styles:
        raise ValueError(
            f"Invalid rustfmt style: {style}. Must be one of {valid_styles}"
        )

    return get_asset_path("rustfmt", f"{style}.toml")
