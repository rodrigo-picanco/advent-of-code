require_relative '../puzzle.rb'

class PartTwo < Puzzle
  def initialize
    super(
      example_output: 9,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-two')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  DIRECTIONS = {
    up_left: [-1, -1],
    up_right: [-1, 1],
    down_left: [1, -1],
    down_right: [1, 1]
  }


  def run(grid)
    result = 0 
    bounded = ->(x, y) { x >= 0 && x < grid.size && y >=0 && y < grid.first.size }
    check_direction = ->(direction:, letter:, x:, y:) {
      dx, dy = DIRECTIONS[direction]
      nx, ny = x + dx, y + dy
      bounded.call(nx, ny) && grid[nx][ny] == letter
    }

    grid.each_with_index do |_, x|
      grid.each_with_index do |_, y|
        next unless grid[x][y] == 'A'
       result += 1 if (check_direction.call(direction: :up_left, x:, y:, letter: 'M') &&
          check_direction.call(direction: :up_right, x:, y:, letter: 'M') &&
          check_direction.call(direction: :down_left, x:, y:, letter: 'S') &&
          check_direction.call(direction: :down_right, x:, y:, letter: 'S')) || 
        (check_direction.call(direction: :up_left, x:, y:, letter: 'S') &&
          check_direction.call(direction: :up_right, x:, y:, letter: 'S') &&
          check_direction.call(direction: :down_left, x:, y:, letter: 'M') &&
          check_direction.call(direction: :down_right, x:, y:, letter: 'M') ) ||
        (check_direction.call(direction: :up_left, x:, y:, letter: 'M') &&
          check_direction.call(direction: :up_right, x:, y:, letter: 'S') &&
          check_direction.call(direction: :down_left, x:, y:, letter: 'M') &&
          check_direction.call(direction: :down_right, x:, y:, letter: 'S')) ||
        (check_direction.call(direction: :up_left, x:, y:, letter: 'S') &&
          check_direction.call(direction: :up_right, x:, y:, letter: 'M') &&
          check_direction.call(direction: :down_left, x:, y:, letter: 'S') &&
          check_direction.call(direction: :down_right, x:, y:, letter: 'M'))
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
