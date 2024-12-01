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
    nums, freq = parsed
   
    freq_map = {}
    freq.each do |f|
      if freq_map[f].nil?
        freq_map[f] = 1
      else
        freq_map[f] += 1
      end
    end

    nums.map do |n|
      if freq_map[n]
        n * freq_map[n]
      else
        0
      end
    end.sum
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
