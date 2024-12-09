require_relative '../puzzle.rb'

class PartTwo < Puzzle
  def initialize
    super(
      example_output: 11387,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-two')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run(input)
    ops = ["*", "+", "||"]

    input.filter_map do |expected, factors|
      expected if ops.repeated_permutation(factors.count - 1).find do |ops|
        expected == evaluate(factors.zip(ops).flatten.compact)
      end
    end.sum
  end

  def evaluate(expression)
    while expression.size > 1
      left, op, right = expression.shift(3)

      if op == "||"
        expression.unshift("#{left}#{right}".to_i)
        next
      end

      expression.unshift(left.send(op, right))
    end
    expression.first 
  end

  def parse(input)
    input.split("\n").map do |line|
      expected, factors = line.split(':')
      expected = expected.to_i
      factors = factors.split(' ').map(&:to_i)

      [expected, factors]
    end
  end
end
