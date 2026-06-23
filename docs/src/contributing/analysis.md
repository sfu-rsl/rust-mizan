# Update the analysis

The leaderboard's contamination verdicts come from a Docent rubric (see [Trajectory analysis](../analysis.md)). To refresh it after adding runs:

1. You need access to the RustMizan Docent collection.
2. Upload the new `.eval` files to the collection and run the rubric on them.
3. Fetch the verdicts and commit the result:
   ```bash
   python fetch_docent.py   # needs DOCENT_API_KEY (or a DOCENT_TOKEN file)
   ```
   This writes `data/docent.json`; commit it and open a pull request.
