import os
import toml
from typing import TypedDict, Literal, Any

TOML_CONFIG_PATH = "pseudo-enum.toml"
DEFAULT_CONFIG_FILE = {
	"build_path": "out/Shared/PseudoEnum.luau",
	"enums": {},
}

class PseudoEnumConfig(TypedDict):
	build_path: str
	import_as_class: bool
	enums: dict[str, list[str]]

def init_config():
	assert not os.path.exists(TOML_CONFIG_PATH), "pseudo-enum already exists"
	config_file = open(TOML_CONFIG_PATH, "w")
	config_file.write(toml.dumps(DEFAULT_CONFIG_FILE))
	config_file.close()

def get_config_data() -> PseudoEnumConfig:
	config_file = open(TOML_CONFIG_PATH, "r")
	config_data: Any = toml.loads(config_file.read())
	return config_data