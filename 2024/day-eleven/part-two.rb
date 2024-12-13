require_relative '../puzzle.rb'
require 'parallel'

class PartTwo < Puzzle
  def initialize
    super(
      example_output: 224869647102559,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-two')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run(stones)
    stones.each_with_index.map do |stone, i|
      count(stone, 75)
    end.sum
  end

  $memo = {}

  def count(stone, steps)
    hash = "#{stone}-#{steps}"
    memo = ->(value) { $memo[hash] = value }

    return $memo[hash] if $memo[hash]

    return 1 if steps == 0
    return memo.call(count(1, steps -1)) if stone == 0

    string = stone.to_s
    length = string.size
    if length.even? 
      half = (length / 2).abs
      first_half = string[0...half].to_i
      second_half = string[half..length].to_i
      return memo.call(count(first_half, steps - 1) + count(second_half, steps - 1))
    end

    memo.call(count(stone * 2024, steps -1))
  end

  def parse(input)
    input.split(" ").map(&:to_i)
  end
end
