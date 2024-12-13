require_relative '../puzzle.rb'

class PartOne < Puzzle
  def initialize
    super(
      example_output: 55312,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run(stones)
    75.times do
      stones = stones.map do |stone|
        if stone == 0
          1
        elsif stone.to_s.size.even? 
          stone.to_s.then do |stone|
            [stone[0...(stone.size/2).abs].to_i, stone[(stone.size/2).abs..stone.size].to_i]
          end
        else
          stone * 2024
        end
      end.flatten
    end

    stones.size
  end

  def parse(input)
    input.split(" ").map(&:to_i)
  end
end
