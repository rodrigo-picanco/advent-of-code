# Advent of Code 2025 🎄

Streamlined setup optimized for Claude Code on mobile!

## Quick Start

### First Time Setup
```bash
cd 2025
bundle install
```

### Mobile-Friendly Slash Commands

Use these in Claude Code for quick access:

- `/aoc-new` - Create a new day
- `/aoc-run` - Start the interactive runner
- `/aoc-status` - Check your progress
- `/aoc-solve` - Get help solving a puzzle

### Manual Workflow

**1. Create a new day:**
```bash
cd 2025
ruby runner.rb
# Select the day from menu
```

**2. Add your inputs:**
- Paste puzzle input into `day-{number}/input`
- Paste example input into `day-{number}/example-one`
- Update `example_output:` in `part-one.rb` with expected result

**3. Implement solution:**
Edit `day-{number}/part-one.rb`:
- Implement `parse(input)` - parse the input string
- Implement `run(parsed)` - solve and return the answer

**4. Run and test:**
```bash
ruby runner.rb
```
The runner will:
- Test with example input first
- If example passes, run with real input
- Auto-reload when you save changes
- Track completion with result files

## Structure

```
2025/
├── runner.rb          # Interactive runner with file watching
├── puzzle.rb          # Base Puzzle class
├── template/          # Template for new days
└── day-{number}/      # One directory per day
    ├── part-one.rb    # Part 1 solution
    ├── part-two.rb    # Part 2 solution
    ├── example-one    # Example input for part 1
    ├── example-two    # Example input for part 2
    ├── input          # Real puzzle input
    ├── part-one-result  # Saved result (created when solved)
    └── part-two-result  # Saved result (created when solved)
```

## Tips for Mobile

- Use slash commands to avoid typing long paths
- The file watcher auto-reloads - no need to restart
- Runner tracks progress with result files
- Ask Claude to help implement solutions with `/aoc-solve`

Happy coding! 🎅
