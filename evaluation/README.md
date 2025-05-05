# Evaluation

This folder contains the scripts and outputs for evaluating LLMs on RustMizan.

## Folder Structure

- [`prompt-templates/`](prompt-templates/) contains the prompt templates used
- [`outputs/`](outputs/) contains the JSON files generated from LLM outputs
- [`results/`](results/) contains the scored results per model, stored in JSON format

## How to Use

### 1. Generate the Prompt and Run the LLM

To run evaluations, we use [`sprout`](https://github.com/sfu-rsl/sprout) shell script

#### Prompt Templates

All prompt templates are stored in [`prompt-templates/`](prompt-templates/). Each prompt file should be named according to its purpose and version. For example:

```
prompt-templates/find_vulnerability_v1.txt
```

#### Output Naming Convention

LLM outputs will be saved under the `outputs/` folder. Each output file should be named in the format:

```
<code-id>--<llm-name>--<prompt-name>.json
```

Example:

```
outputs/vuln-0001-fixed-file--claude-3-7-sonnet-20250219--find_vulnerability_v1.json
```

#### Example Command

To evaluate a code sample using a specific LLM and prompt:

```sh
sprout -d ../vuln-0001/fixed-file \
       -l anthropic \
       -p prompt-templates/find_vulnerability_v1.txt \
       > outputs/vuln-0001-fixed-file--claude-3-7-sonnet-20250219--find_vulnerability_v1.json
```

- `-d`: Path to the code sample
- `-l`: LLM provider (`anthropic` or `openai`)
- `-p`: Prompt file (from `prompt-templates/`)

### 2. Score the LLM Output

Run the following command inside the `evaluation` directory:

```sh
python score_and_append.py \
  claude-3-7-sonnet-20250219 \
  vuln-0001/fixed-function \
  outputs/vuln-0001-fixed-file--claude-3-7-sonnet-20250219--find_vulnerability_v1.json \
  find_vulnerability_v1
```

- First argument: name of the LLM (used to name the results file).
- Second argument: benchmark code sample.
- Third argument: path to the LLM output JSON file (must be a valid JSON object).
- Fourth argument: the name of the prompt template used (e.g., `find_vulnerability_v1`).

This step:

- Parses the LLM output.
- Compares it against the ground truth in `mizan.json`.
- Appends a new evaluation result to a structured JSON array under `results/{llm_name}--{prompt_name}.json`.

An example result entry looks like:

```json
[
  {
    "llm_name": "claude-3-7-sonnet-20250219",
    "prompt_name": "find_vulnerability_v1",
    "code_sample": "vuln-0001/fixed-function",
    "timestamp": "2025-05-05T18:10:44.209293+00:00",
    "is_fixed_version": true,
    "raw_answer": "{\"is_vulnerable\": false, \"cwe_type\": [], \"vulnerable_functions\": {\"src/buffer.rs\": []}, \"vulnerable_lines\": {\"src/buffer.rs\": []}}",
    "parsed_answer": {
      "is_vulnerable": false,
      "cwe_type": [],
      "vulnerable_functions": {
        "src/buffer.rs": []
      },
      "vulnerable_lines": {
        "src/buffer.rs": []
      }
    },
    "scoring": {
      "existence_detection": {
        "is_correct": true
      },
      "cwe_inference": {
        "correct_predictions": [],
        "missed_predictions": [],
        "extra_predictions": []
      },
      "key_objects_identification": {
        "true_positive_keys": [],
        "false_positive_keys": [],
        "missed_keys": []
      },
      "root_cause_location": {
        "true_positive_keys": [],
        "false_positive_keys": [],
        "missed_keys": []
      }
    }
  }
]
```

## Scoring Criteria

Each LLM output is evaluated on four tasks:

| Task                                     | Description                                      | Scoring Rule                                                 |
| :--------------------------------------- | :----------------------------------------------- | :----------------------------------------------------------- |
| Vulnerability Existence Detection        | Whether the code sample contains a vulnerability | Exact match (`true` or `false`)                              |
| CWE Type Inference                       | Identifying the correct CWE types                | Partial credit if at least one ground-truth CWE is predicted |
| Key Objects and Functions Identification | Identifying important vulnerable functions       | Based on matching file and function signatures               |
| Root Cause Location                      | Identifying vulnerable lines of code             | Based on matching file and line numbers                      |

The output is compared strictly:

- Functions and lines are matched as `"filename::function signature"` or `"filename::line number"`.
- Partial credit is allowed for CWE and key object identification if any correct matches are found.

> [!NOTE]
> It is assumed that fixed code samples do not contain any other hidden vulnerabilities.
