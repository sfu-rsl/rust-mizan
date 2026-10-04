#!/usr/bin/env python3
"""Model and harness configuration for all 16 reported runs, read from the log headers."""

from common import MODEL_ORDER, NAMES, SPLIT_ORDER, load_configs, write_csv, write_table


def main():
    cfgs = load_configs()
    order = {s: i for i, s in enumerate(SPLIT_ORDER)}
    morder = {m: i for i, m in enumerate(MODEL_ORDER)}

    rows = []
    for c in sorted(cfgs, key=lambda c: (order[c["split"]], morder[c["model"]])):
        setting = {**c['plan_config'], **c['model_args']}
        rows.append([c['split'], NAMES[c['model']], c['model_id'],
                     setting.get('reasoning_effort'), setting.get('reasoning_enabled'),
                     c['epochs'], c['message_limit'], c['time_limit'], c['completed']])
    write_csv('configuration.csv', ['split', 'model', 'model_id', 'reasoning_effort',
                                   'reasoning_enabled', 'epochs', 'message_limit',
                                   'time_limit', 'completed_samples'], rows)
    providers = {'Claude': 'Anthropic', 'GPT': 'OpenAI', 'Gemini': 'Google', 'Qwen': 'OpenRouter'}
    rows = []
    for model in MODEL_ORDER:
        configs = [c for c in cfgs if c['model'] == model]
        assert len(configs) == 4
        settings = [{**c['plan_config'], **c['model_args']} for c in configs]
        assert all(s == settings[0] for s in settings)
        assert all(c['model_id'] == configs[0]['model_id'] for c in configs)
        reasoning = 'Disabled' if settings[0].get('reasoning_enabled') is False else settings[0]['reasoning_effort'].title()
        rows.append([NAMES[model], providers[model], r'\texttt{' + configs[0]['model_id'] + '}', reasoning])
    write_table('model_configuration.tex', ['Model', 'Provider', 'Model ID', 'Reasoning'], rows)

    print("%-14s %-8s %-34s %s" % ("Split", "Model", "Model id", "Reasoning setting"))
    for c in sorted(cfgs, key=lambda c: (order[c["split"]], morder[c["model"]])):
        setting = dict(c["plan_config"])
        setting.update(c["model_args"])
        print("%-14s %-8s %-34s %s"
              % (c["split"], c["model"], c["model_id"],
                 ", ".join("%s=%s" % kv for kv in sorted(setting.items()))))

    print()
    for key in ("epochs", "message_limit", "time_limit"):
        vals = {c[key] for c in cfgs}
        print("%-14s %s" % (key, vals.pop() if len(vals) == 1 else "VARIES %s" % vals))
    done = {(c["samples"], c["completed"]) for c in cfgs}
    print("%-14s %s" % ("samples", done.pop() if len(done) == 1 else "VARIES %s" % done))
    print("%-14s %d" % ("runs", len(cfgs)))

    distinct = {(tuple(sorted(c["plan_config"].items())),
                 tuple(sorted(c["model_args"].items()))) for c in cfgs}
    print("%-14s %d" % ("distinct cfgs", len(distinct)))


if __name__ == "__main__":
    main()
