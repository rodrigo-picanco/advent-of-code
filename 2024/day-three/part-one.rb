require_relative '../puzzle.rb'

class PartOne < Puzzle
  def initialize
    super(
      example_output: 161,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run(instructions)
    instructions.map { |a, b| a * b }.sum
  end

  def parse(input)
    Parser.new(input).parse
  end
end

class Parser
  def initialize(input)
    @input = input
    @position = 0
    @ast = []
  end

  def parse
    while current_token != nil
      case current_token
      when 'm'
        parse_mul
      else
        consume
      end
    end
    @ast
  end

  def parse_mul
    consume
    return if !current_token? 'u'
    consume 
    return if !current_token? 'l'
    consume
    return if !current_token? '('
    consume
    first_operand = ""
    while is_digit? current_token
      first_operand << current_token
      consume
    end
    return if !current_token? ','
    consume
    second_operand = ""
    while is_digit? current_token
      second_operand << current_token
      consume
    end
    return if !current_token? ')'
    consume
    @ast << [first_operand.to_i, second_operand.to_i]
  end

  def parse_int
    consume
    while current_token? '0'..'9'
      consume
    end
  end

  def is_digit?(char)
    ("0".."9").include? char
  end

  def current_token?(char)
    current_token == char
  end

  def current_token
    @current_token = @input[@position]
  end

  def next_token
    @position += 1
    current_token
  end

  def consume
    @position += 1
  end
end
