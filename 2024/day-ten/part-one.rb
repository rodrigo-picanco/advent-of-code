require_relative '../puzzle.rb'

class PartOne < Puzzle
  def initialize
    super(
      example_output: 36,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def print_grid(grid, special)
    grid.each do |row|
      row.each do |cell|
        if speciall.call(cell)
          print "\e[32m#{cell}\e[0m"
        else
          print cell
        end
      end
      print "\n"
    end
  end

  def run(grid)
    rows, cols = grid.length, grid.first.length
    directions = [[1, 0], [0, 1], [-1, 0], [0, -1]]
    inbound = -> (row, col) { row >= 0 && row < rows && col >= 0 && col < cols }

    trail_heads = []
    rows.times do |row|
      cols.times do |col|
        trail_heads << [row, col] if grid[row][col] == 0
      end
    end

    total = 0
    trail_heads.each do |trail_head|
      dp = Array.new(rows) { Array.new(cols, 0) }
      dp[trail_head[0]][trail_head[1]] = 1

      (1..9).each do |value|
        rows.times do |row|
          cols.times do |col|
            next unless grid[row][col] == value

            directions.each do |direction|
              new_row, new_col = row + direction.first, col + direction.last

              if inbound.call(new_row, new_col) && grid[new_row][new_col] == value - 1
                dp[row][col] += dp[new_row][new_col]
              end
            end
          end
        end
      end

      score = 0
      rows.times do |row|
        cols.times do |col|
          score += 1 if dp[row][col] > 0 && grid[row][col] == 9
        end
      end

      total += score
    end

    total
  end

  def parse(input)
    input.split("\n").map do |line|
      line.chars.map(&:to_i)
    end
  end
end
