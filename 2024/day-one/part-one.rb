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
    first_col, second_col = parsed
    first_sort = first_col.sort
    second_sort = second_col.sort

    diff = []
    first_sort.zip(second_sort).map do |first, second|
      diff << (first - second).abs
    end

    diff.sum
  end

  def parse(input)
    first_col, second_col = [], []
    input.split("\n").each do |line|
      values = line.strip.split(' ')
      first_col << values[0].to_i
      second_col << values[1].to_i
    end
    [first_col, second_col]
  end
end
