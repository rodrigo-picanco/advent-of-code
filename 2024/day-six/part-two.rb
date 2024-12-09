require_relative '../puzzle.rb'
require 'parallel'

class PartTwo < Puzzle
  def initialize
    super(
      example_output: 6,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-two')),
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

  def walk(grid:, direction:, position:, blocked:)
    dx, dy = DIRECTIONS[direction]
    x, y = position
    nx, ny = x + dx, y + dy

   return [[x, y], :outbound] if outbound?(grid, [nx, ny]) 

    if grid[nx][ny] == OBSTACLE || blocked == [nx, ny]
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
      return walk(grid:, direction:, position:, blocked:)
    end

    [[nx, ny], direction]
  end

  def outbound?(grid, position)
    x, y = position
    x < 0 || x >= grid.size || y < 0 || y >= grid.first.size
  end

  def process(grid:, visited:, direction:, position:, loops:, blocked:)
    tortoise_position, tortoise_direction = walk(grid:, direction:, position:, blocked:)
    hare_position, hare_direction = walk(grid:, direction: tortoise_direction, position: tortoise_position, blocked:)
    return if hare_direction == :outbound || hare_direction == :outbound
    loop do
      if hare_position == tortoise_position && hare_direction == tortoise_direction
        loops << blocked
        break
      end
      tortoise_position, tortoise_direction = walk(grid:, direction: tortoise_direction, position: tortoise_position, blocked:)
      break if tortoise_direction == :outbound
      hare_position, hare_direction = walk(grid:, direction: hare_direction, position: hare_position, blocked:)
      break if hare_direction == :outbound
      hare_position, hare_direction = walk(grid:, direction: hare_direction, position: hare_position, blocked:)
      break if hare_direction == :outbound
    end
  end

  def run(grid)
    loops = []
    grid_size = grid.size * grid[0].size
    Parallel.each(0...grid_size, in_threads: 64, progress: "Calculating loops", finish: -> (_, i, _) { puts "#{i} of #{grid_size} completed. #{loops.count} loops found so far"   }) do |index|
      x = index / grid[0].size
      y = index % grid[0].size
      next if grid[x][y] == OBSTACLE
      position = find_value_in_grid(grid, '^')
      next if position == [x, y]
      direction = :up
      visited = [[position, direction]]
      process(grid:, visited:, direction:, position:, blocked: [x, y], loops:)
    end
    loops.count
  end

  def parse(input)
    input.split("\n").map do |line|
      line.split('')
    end
  end
end
