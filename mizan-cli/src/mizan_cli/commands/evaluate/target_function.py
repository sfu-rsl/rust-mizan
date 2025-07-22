import json
import re
from typing import Dict, Any
import openai
import anthropic
from mizan_cli.utils.logging import get_logger

logger = get_logger()


def create_target_function(provider: str, model: str, temperature: float, experiment_dir=None):
    """Create a target function for the specified provider and model


    The term "target function" comes from LangSmith's evaluation framework.
    See https://docs.smith.langchain.com/evaluation/how_to_guides/define_target
    for more details.
    """

    def target_function(inputs: Dict[str, Any]) -> Dict[str, Any]:
        system_prompt = inputs.get("system_prompt", "")
        prompt = inputs.get("prompt", "")
        
        # Set up logging file if experiment_dir is provided
        full_responses_file = None
        if experiment_dir:
            from pathlib import Path
            full_responses_file = Path(experiment_dir) / "full_responses.log"

        try:
            if provider == "openai":
                client = openai.OpenAI(max_retries=5)

                messages = []
                if system_prompt:
                    messages.append({"role": "system", "content": system_prompt})
                messages.append({"role": "user", "content": prompt})
                response = client.chat.completions.create(
                    model=model, messages=messages, temperature=temperature
                )

                raw_response = response.choices[0].message.content
                
                # Log full response to file
                if full_responses_file:
                    with open(full_responses_file, "a") as f:
                        f.write(str(response))
                        f.write("\n")

            elif provider == "anthropic":
                client = anthropic.Anthropic(max_retries=5)

                # Anthropic API handles messages differently
                if system_prompt:
                    response = client.messages.create(
                        model=model,
                        system=system_prompt,
                        messages=[{"role": "user", "content": prompt}],
                        temperature=temperature,
                        max_tokens=2000,
                    )
                else:
                    response = client.messages.create(
                        model=model,
                        messages=[{"role": "user", "content": prompt}],
                        temperature=temperature,
                        max_tokens=2000,
                    )

                raw_response = response.content[0].text
                
                # Log full response to file
                if full_responses_file:
                    with open(full_responses_file, "a") as f:
                        f.write(str(response))
                        f.write("\n")

            else:
                raise ValueError(f"Unsupported provider: {provider}")

            # Try to parse JSON from the response
            try:
                # Look for JSON in the response
                json_match = re.search(r"\{.*\}", raw_response, re.DOTALL)
                if json_match:
                    json_str = json_match.group(0)
                    parsed_response = json.loads(json_str)
                else:
                    return {
                        "raw_response": raw_response,
                        "parsed_response": None,
                        "errors": "No JSON found in response",
                    }
            except json.JSONDecodeError as e:
                return {
                    "raw_response": raw_response,
                    "parsed_response": None,
                    "errors": f"JSON parsing failed: {str(e)}",
                }

            return {
                "raw_response": raw_response,
                "parsed_response": parsed_response,
                "errors": None,
            }

        except Exception as e:
            return {
                "raw_response": None,
                "parsed_response": None,
                "errors": str(e),
            }

    return target_function
