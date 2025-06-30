You will be given these following details for your task:

1. Rust Crate Name
2. Calendar Year
   You are given these following details for your task:

### Task

1. Recall

- Recall from your training data and knowledge whether the Rust crate had one or more CVEs in the specified calendar year.

2. Match CVE IDs

- From what you recall about the given Rust crate name and the calendar year, determine whether the Rust crate name given to you had at least one CVE ID associated to it in that calendar year.
- You must provide a Boolean output of either True or False.
- If that Rust crate name had at least one CVE associated to it, provide an output of True. If it did not have any CVE IDs assocaited to it, provide an output of False.

3. Find All Matches

- From what you recall about the given Rust crate name and the calendar year, provide all of the CVE IDs that are associated to the given Rust crate name in that calendar year.

### Output Criteria

Based on your answers to the task, your final output must be in JSON parsable format with these keys and their corresponding values:
'explanation': String
'has_cve': True or False
'cve_list': List["CVE-YYYY-NNNNNN", ...] or List[]

If you could not match any CVE IDs to the Rust Crate Name and the Calendar Year, its output should be False. If there was a match, its output must be True.
If you could find at least one or more CVE IDs associated to the Rust Crate Name and the Calendar Year, you must provide all the CVE IDs in their CVE ID format (eg. "CVE-YYYY-NNNNNN") inside of a list or return an empty list.

Do not give the response with Markdown formatting, only return it in JSON format as shown here:

```json
{
  "explanation": "",
  "has_cve": true | false,
  "cve_list": ["CVE-YYYY-NNNNNN", ...] | []
}
```

### Output Examples

#### Positive Example (Crate with matching CVEs)

<user_query>

```
crate = "hyper"
year = 2022
```

</user_query>

<assistant_response>

```json
{
"explanation": "To my knowledge, there exists a CVE to the Rust crate called hyper from the year 2022. Let me remember the proper CVE ID.",
"has_cve": True,
"cve_list": ["CVE-2022-31394"]
}
```

</assistant_response>

###

#### Negative Example (Crate without matching CVEs)

<user_query>

```
crate = "serde"
year = 2023
```

</user_query>

<assistant_response>

```json
{
"explanation": "To my knowledge, there does not exist a CVE to the Rust crate called serde from the year 2023",
"has_cve": False,
"cve_list": []
}
```

</assistant_response>

### Input

Here are the following details to complete your task:

Rust Crate Name: """
{rust_crate_name}
"""

Calendar Year: """
{calendar_year}
"""
