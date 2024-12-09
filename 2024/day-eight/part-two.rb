require_relative '../puzzle.rb'

class PartTwo < Puzzle
  def initialize
    super(
      example_output: 34,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-two')),
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
        antinodes = [a[:position], b[:position]]

        distance = 1
        loop do
            anti = [xa - vector[0] * distance, ya - vector[1] * distance]
            break unless inbound?(grid, anti)
            antinodes << anti
            distance += 1
        end

        distance = 1
        loop do
            anti = [xb + vector[0] * distance, yb + vector[1] * distance]
            break unless inbound?(grid, anti)
            antinodes << anti
            distance += 1
        end

        antinodes
      end
    end
    .values
    .flatten(2)
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
