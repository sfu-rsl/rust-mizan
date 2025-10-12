import json
import os
import re
from typing import Dict, Any
import openai
import anthropic
from google import genai
from google.genai import types
from google.api_core import retry
from mizan_cli.utils.logging import get_logger

logger = get_logger()


# Google has 2 versions of gemini API. This seems to be the correct way to do this in the newer version we are using.
# Taken from https://discuss.ai.google.dev/t/how-to-implement-retry-logic-in-the-new-python-sdk/83052
@retry.Retry(
    predicate=retry.if_transient_error,
    initial=1.0,
    maximum=60.0,
    multiplier=2.0,
    timeout=120.0,
)
def generate_gemini_content_with_retry(client, model, contents, config):
    """Generate content with Gemini API with retry logic."""
    return client.models.generate_content(
        model=model,
        contents=contents,
        config=config,
    )


def create_target_function(
    provider: str, model: str, temperature: float, experiment_dir=None
):
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

                if full_responses_file:
                    with open(full_responses_file, "a") as f:
                        f.write(str(response))
                        f.write("\n")

            elif provider == "deepseek":
                client = openai.OpenAI(
                    api_key=os.environ.get("DEEPSEEK_API_KEY"),
                    base_url="https://api.deepseek.com",
                    max_retries=5,
                )

                messages = []
                if system_prompt:
                    messages.append({"role": "system", "content": system_prompt})
                messages.append({"role": "user", "content": prompt})

                response = client.chat.completions.create(
                    model=model, messages=messages, temperature=temperature
                )

                raw_response = response.choices[0].message.content

                if full_responses_file:
                    with open(full_responses_file, "a") as f:
                        f.write(str(response))
                        f.write("\n")

            elif provider == "gemini":
                # Create Gemini client - try GOOGLE_API_KEY first, then GEMINI_API_KEY for backward compatibility
                api_key = os.environ.get("GOOGLE_API_KEY") or os.environ.get(
                    "GEMINI_API_KEY"
                )
                if not api_key:
                    raise ValueError(
                        "Missing API key. Set either GOOGLE_API_KEY or GEMINI_API_KEY environment variable."
                    )
                client = genai.Client(api_key=api_key)

                config_params = {
                    # No reasoning
                    "thinking_config": types.ThinkingConfig(thinking_budget=0),
                    "temperature": temperature,
                    "max_output_tokens": 2000,
                }
                if system_prompt:
                    config_params["system_instruction"] = system_prompt

                response = generate_gemini_content_with_retry(
                    client=client,
                    model=model,
                    contents=prompt,
                    config=types.GenerateContentConfig(**config_params),
                )

                raw_response = response.text

                if full_responses_file:
                    with open(full_responses_file, "a") as f:
                        f.write(str(response))
                        f.write("\n")

            elif provider == "local":
                api_key = os.environ.get("LOCAL_LLM_API_KEY")
                base_url = os.environ.get("LOCAL_LLM_BASE_URL")
                client = openai.OpenAI(
                    base_url=base_url,
                    api_key=api_key,
                    max_retries=5,
                )

                messages = []
                if system_prompt:
                    messages.append({"role": "system", "content": system_prompt})
                messages.append({"role": "user", "content": prompt})

                max_attempts = 5
                for attempt in range(max_attempts):
                    try:
                        response = client.chat.completions.create(
                            model=model,
                            messages=messages,
                            temperature=temperature,
                            top_p=0.95,
                            max_tokens=2000,
                        )

                        raw_response = response.choices[0].message.content.strip()

                        if full_responses_file:
                            with open(full_responses_file, "a") as f:
                                f.write(str(response))
                                f.write("\n")
                        break
                    except Exception as e:
                        if attempt < max_attempts - 1:
                            logger.warning(
                                f"Local LLM request failed (attempt {attempt + 1}/{max_attempts}): {e}"
                            )
                            continue
                        else:
                            raise

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
            # Log failure to file
            if full_responses_file:
                with open(full_responses_file, "a") as f:
                    f.write(f"ERROR: {str(e)}\n")
                    f.write("\n")

            return {
                "raw_response": None,
                "parsed_response": None,
                "errors": str(e),
            }

    return target_function
