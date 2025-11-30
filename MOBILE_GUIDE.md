# AOC 2025 Mobile Quick Reference

## Claude Code Slash Commands

Type these in Claude Code chat:

- `/aoc-new` → Create new day
- `/aoc-run` → Start runner
- `/aoc-status` → Check progress
- `/aoc-solve` → Get help solving

## Quick Commands

**Start solving:**
```bash
cd 2025 && ruby runner.rb
```

**Create new day manually:**
```bash
cd 2025 && cp -r template day-{number}
```

## Workflow

1. Use `/aoc-new` to create day directory
2. Paste inputs:
   - Real input → `2025/day-{n}/input`
   - Example → `2025/day-{n}/example-one`
3. Update expected output in `part-one.rb`
4. Use `/aoc-solve` to implement solution
5. Use `/aoc-run` to test and run

## File Structure Quick Ref

```
2025/day-{number}/
  part-one.rb       # Implement parse() and run()
  example-one       # Paste example input here
  input             # Paste real input here
```

See `2025/README.md` for full details.
