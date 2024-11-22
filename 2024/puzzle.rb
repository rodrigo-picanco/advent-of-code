class PuzzleSolutionError < StandardError
end

class Puzzle
  def initialize(example_path:, input_path:, example_output:)
    @example_output = example_output
    @example_input = File.read(example_path)
    @puzzle_input = File.read(input_path)
  end

  def assert
    result = run parse @example_input 
    if @example_output == result
      "You did it! Example result is #{result}"
    else
      raise PuzzleSolutionError, "Ooops! Not yet. Expected output: #{@example_output} but got #{result}"
    end
  end

  def solve
    run parse @puzzle_input
  end

  def parse(input)
    raise NotImplementedError
  end

  def run(input)
    raise NotImplementedError
  end
end


