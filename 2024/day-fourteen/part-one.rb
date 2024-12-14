require_relative '../puzzle.rb'

class PartOne < Puzzle
  include Grid

  def initialize
    super(
      example_output: 12,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  SECONDS = 100

  def run(robots)
    cols, lines = @solve ? [101, 103] : [11, 7]
    cx, cy = cols / 2, lines / 2

    quadrant = -> ((x, y)) {
      case
      when x < cx && y < cy
        :top_left
      when x >= cx && y < cy
        :bottom_left
      when x < cx && y >= cy
        :top_right
      when x >= cx && y >= cy
        :bottom_right
      end
    }
    
    robots.filter_map do |(px, py), (vx, vy)|
      x =  (px + vx * SECONDS) % cols 
      y =  (py + vy * SECONDS) % lines
      x = x + cols % cols if x.negative? 
      y = y + lines % lines if y.negative? 
      [x, y]
    end
    .filter { |(x, y)| x != cx && y != cy }
    .map { |pos| quadrant.call(pos) }
    .tally
    .values
    .reduce(:*)
  end

  def parse(input)
    input.each_line.map do |line|
      p, v = line.split
      parse_info = ->(info) { info.split('=').last.split(',').map(&:to_i) }
      [parse_info.call(p), parse_info.call(v)]
    end
  end
end
