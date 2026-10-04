#!/usr/bin/env python3
"""Token and message usage per variant, from each log's per-sample model_usage."""

import glob
import json
import os
import zipfile

from common import LOGS, MODEL_ORDER, MODELS, SPLIT_ORDER, NAMES, write_csv


def main():
    per = {}
    for path in sorted(glob.glob(os.path.join(LOGS, "*.eval"))):
        z = zipfile.ZipFile(path)
        header = json.loads(z.read("header.json"))
        key = (header["eval"]["task"].replace("mizan-", ""),
               MODELS[header["eval"]["model"]])
        total = msgs = n = 0
        for name in z.namelist():
            if not name.startswith("samples/"):
                continue
            s = json.loads(z.read(name))
            for usage in s.get("model_usage", {}).values():
                total += usage.get("total_tokens", 0)
            msgs += len(s.get("messages", []))
            n += 1
        per[key] = (total, n, msgs)

    print("%-14s %-8s %14s %14s %13s"
          % ("Split", "Model", "total tokens", "per variant", "msgs/variant"))
    rows = []
    for split in SPLIT_ORDER:
        for model in MODEL_ORDER:
            total, n, msgs = per[(split, model)]
            rows.append([split, NAMES[model], total, n, total / n, msgs / n])
            print("%-14s %-8s %14s %14s %13.1f"
                  % (split, model, "{:,}".format(total),
                     "{:,}".format(round(total / n)), msgs / n))
    write_csv('token_usage.csv', ['split', 'model', 'total_tokens', 'samples',
                                 'tokens_per_variant', 'messages_per_variant'], rows)


if __name__ == "__main__":
    main()
