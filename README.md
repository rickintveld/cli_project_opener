# CLI project opener for VSCode

- Make sure you installed the Shell command 'code' in PATH for VSCode.
- Add `export PROJECTS_PATH="/Path/To/Projects"` to your bash profile
- Cargo build
- Add a new alias to your bash profile `alias vscode='/Path/To/Project/cli_project_opener/target/debug/vs_code'`
- Execute `source ~/.zshrc` || `source ~/.bash_profile` to reload the bash profile

```bash
$ vs_code

? Which project do you want to open?
> /Path/To/Project/project_1
  /Path/To/Project/project_2
  /Path/To/Project/project_3
  /Path/To/Project/project_4
  /Path/To/Project/project_5
v /Path/To/Project/project_6
[↑↓ to move, enter to select, type to filter]
```
