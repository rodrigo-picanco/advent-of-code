require_relative '../puzzle.rb'

class PartOne < Puzzle
  def initialize
    super(
      example_output: 11,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run(parsed)
    parsed.first.sort.zip(parsed.last.sort).map do |first, second|
      (first - second).abs
    end
    .sum
  end

  def parse(input)
    input.split("\n").map do |line|
      line.strip.split(' ').map(&:to_i)
    end
    .transpose
  end
end
