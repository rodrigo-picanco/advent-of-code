require_relative '../puzzle.rb'

class PartTwo < Puzzle
  def initialize
    super(
      example_output: 31,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-two')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run(parsed)
    freq = parsed.last.tally
    parsed.first.map do |n|
        n * freq[n]
    end.sum
  end

  def parse(input)
    input.split("\n").map do |line|
      line.strip.split(' ').map(&:to_i)
    end
    .transpose
  end
end
