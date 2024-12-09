require_relative '../puzzle.rb'

class PartOne < Puzzle
  def initialize
    super(
      example_output: 18,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  DIRECTIONS = [
    [0,-1],
    [0,1],
    [-1,0],
    [1,0],
    [-1,-1],
    [1,-1],
    [-1,1],
    [1,1]
  ]

  WORD = 'XMAS'

  def run(grid)
    result = 0 
    bounded = ->(x, y) { x >= 0 && x < grid.size && y >=0 && y < grid.first.size }

    grid.each_with_index do |_, x|
      grid.each_with_index do |_, y|
        next unless grid[x][y] == WORD.chars.first

        DIRECTIONS.each do |dx, dy|
          (0...WORD.size).each do |i|
            nx, ny = x + i * dx, y + i * dy

            if !bounded.call(nx, ny) || grid[nx][ny] != WORD[i]
              break
            end

            if i == WORD.size - 1
              result += 1
            end

          end
        end
      end 
    end

    result
  end

  def parse(input)
    input.split("\n").map do |line|
      line.split('')
    end
  end
end
