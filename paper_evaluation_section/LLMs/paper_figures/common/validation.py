from typing import Dict, Any


def validate_json_schema(parsed_response: Dict[str, Any]) -> bool:
    try:
        if not isinstance(parsed_response, dict):
            return False

        required_fields = [
            "is_vulnerable",
            "cwe_type",
            "vulnerable_functions",
            "vulnerable_lines",
        ]

        if not all(field in parsed_response for field in required_fields):
            return False

        if not isinstance(parsed_response["is_vulnerable"], bool):
            return False

        if not isinstance(parsed_response["cwe_type"], list):
            return False
        if not all(isinstance(cwe, str) for cwe in parsed_response["cwe_type"]):
            return False

        if not isinstance(parsed_response["vulnerable_functions"], dict):
            return False
        if not all(
            isinstance(v, list)
            for v in parsed_response["vulnerable_functions"].values()
        ):
            return False

        if not isinstance(parsed_response["vulnerable_lines"], dict):
            return False
        if not all(
            isinstance(v, list) for v in parsed_response["vulnerable_lines"].values()
        ):
            return False
        if not all(
            all(isinstance(line, int) for line in v)
            for v in parsed_response["vulnerable_lines"].values()
        ):
            return False

        return True
    except:
        return False
