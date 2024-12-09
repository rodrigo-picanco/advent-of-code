require_relative '../puzzle.rb'

class PartOne < Puzzle
  def initialize
    super(
      example_output: 1928,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run(blocks)
    spaces = blocks.select { |block| block == "." }.length
    files = blocks.select { |block| block.is_a?(Integer) }
    files_length = files.length
    spaces.times do |i|
      blocks[blocks.index(".")] = files.pop
    end
    blocks = blocks[0...files_length]
    blocks.each_with_index.map do |block, index|
      block * index
    end.sum
  end


  def parse(input)
    blocks = []
    input.split("").map(&:to_i).each_with_index do |block, index|
      if index == 0 
        block.times { blocks.push(index) }
      elsif index.even?
        block.times { blocks.push(index / 2) }
      else
        block.times { blocks.push(".") }
      end
    end
    blocks
  end
end
