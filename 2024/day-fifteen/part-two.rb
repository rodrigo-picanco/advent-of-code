require_relative '../puzzle.rb'

class PartTwo < Puzzle
  def initialize
    super(
      example_output: 10092,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-two')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
    @directions = {
      '^' => [-1, 0],
      'v' => [1, 0],
      '<' => [0, -1],
      '>' => [0, 1],
    }
  end

  def run((grid, moves))
    robot = grid.each_with_index do |row, r|
      row.each_with_index do |cell, c|
        robot = [r, c] if cell == '@'
      end
    end

    moves.each do |move|
      rr, rc = robot
      dr, dc = @directions[move]
      nr, nc = [rr + dr, rc + dc]

      if grid[nr][nc] == '.'
        robot = [nr, nc]
        grid[rr][rc] = '.'
        grid[nr][nc] = '@'
      elsif grid[nr][nc] == '['
      end
    end
  end

  def parse(input)
    grid, moves = input.split("\n\n")
    parse_grid = -> (grid) {
      grid = grid.split("\n").map do |line|
        line.chars.map do |char|
          case char
            when '#'
              ['#', '#']
            when 'O'
              ['[', ']']
            when '@'
              ['@', '.']
          else
            ['.', '.']
          end
        end.flatten
      end
      grid
    }
    parse_moves = -> (moves) {
      moves.split("\n").join.chars
    }
    [parse_grid.call(grid), parse_moves.call(moves)]
  end
end
