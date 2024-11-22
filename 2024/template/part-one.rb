require_relative '../puzzle.rb'

class PartOne < Puzzle
  def initialize
    super(
      example_output: 0,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run(parsed)
  end

  def parse(input)
    input.split("\n")
  end
end
