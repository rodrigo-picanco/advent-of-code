require_relative '../puzzle.rb'

class PartOne < Puzzle
  def initialize
    super(
      example_output: 41,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  OBSTACLE = '#'
  DIRECTIONS = {
    up: [-1, 0],
    down: [1, 0],
    left: [0, -1],
    right: [0, 1]
  }

  def print_grid(grid)
    grid.each do |row|
      puts row.join('')
    end
  end

  def find_value_in_grid(grid, value )
    grid.each_with_index do |row, x|
      row.each_with_index do |cell, y|
        return [x, y] if cell == value
      end
    end
  end

  def walk(grid:, direction:, position:)
    dx, dy = DIRECTIONS[direction]
    x, y = position
    nx, ny = x + dx, y + dy

   return [[x, y], :outbound] if outbound?(grid, [nx, ny]) 

    if grid[nx][ny] == OBSTACLE
      case direction
      when :up
        direction = :right
      when :right
        direction = :down
      when :down
        direction = :left
      when :left
        direction = :up
      end
      return walk(grid:, direction:, position:)
    end

    [[nx, ny], direction]
  end

  def outbound?(grid, position)
    x, y = position
    x < 0 || x >= grid.size || y < 0 || y >= grid.first.size
  end

  def run(grid)
    position = find_value_in_grid(grid, '^')
    direction = :up
    visited = [position]

    loop do
      position, direction = walk(grid:, direction:, position:)
      break if direction == :outbound 
      visited << position
    end

    visited.uniq.size

  end

  def parse(input)
    input.split("\n").map do |line|
      line.split('')
    end
  end
end
