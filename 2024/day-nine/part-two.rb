require_relative '../puzzle.rb'

class PartTwo < Puzzle
  def initialize
    super(
      example_output: 2858,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-two')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run(items)
    i = items.length - 1
    while i >= 0
      block = items[i]
      if block[:type] == 'file'
        file = block
        j = 0
        while j <= i
          block = items[j]
          if block[:type] == 'space'
            space = block
            if space[:size] >= file[:size]
              items[j] = file
              items[i] = { type: 'space', size: file[:size] }
              size_diff = space[:size] - file[:size]
              if size_diff > 0
                items.insert(j + 1, { type: 'space', size: size_diff })
              end
              break
            end
          end
          j += 1
        end
      end
      i -= 1
    end

    blocks = []
    items.each do |item, index|
      if item[:type] == 'file'
        item[:size].times { blocks.push(item[:index]) }
      else
        item[:size].times { blocks.push(".") }
      end
    end

    blocks.each_with_index.filter_map do |block, index|
      next if block == "."
      block * index
    end.sum

  end

  def print_blocks(blocks)
    blocks.each do |block|
      if block[:type] == 'file'
        block[:size].times { print block[:index] }
      else
        block[:size].times { print "." }
      end
    end
    print "\n"
  end


  def parse(input)
    input.strip.chars.each_with_index.map do |block, index|
      if index.even?
        {
          type: 'file',
          size: block.to_i,
          index: index / 2,
        }
      else
        {
          type: 'space',
          size: block.to_i,
        }
      end
    end
  end
end
