# jupyterscrub

This tool is a minimal, fast replacement for `jupyter nbconvert --clear-output --inplace`.

`nbconvert` is somewhat slow when run as a pre-commit hook, or in organisations with lots of notebooks.

You can use `jupyterscrub file1.ipynb file2.ipynb ...` as a drop-in replacement.
