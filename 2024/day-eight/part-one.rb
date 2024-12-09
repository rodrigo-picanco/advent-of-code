require_relative '../puzzle.rb'

class PartOne < Puzzle
  def initialize
    super(
      example_output: 14,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run(grid)
    grid.each_with_index.filter_map do |_, x|
      grid.each_with_index.filter_map do |_, y|
        grid[x][y].then do |cell|
          next if  cell == '.'
          { frequency: cell, position: [x, y] }
        end
      end
    end
    .flatten
    .group_by { |cell| cell[:frequency] }
    .transform_values do |cells|
      cells.combination(2).filter_map do |a, b|
        xa, ya = a[:position]
        xb, yb = b[:position]
        vector = [xb - xa, yb - ya]
        anti_one = [xa - vector[0], ya - vector[1]]
        anti_two = [xb + vector[0], yb + vector[1]]
        [anti_one, anti_two]
      end
    end
    .values
    .flatten(2)
    .filter { |position| inbound?(grid, position) }
    .uniq
    .count

  end

  def inbound?(grid, position)
    x, y = position
    x >= 0 && x < grid.size && y >= 0 && y < grid.first.size
  end

  def parse(input)
    input.split("\n").map do |line|
      line.chars
    end
  end
end
