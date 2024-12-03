
require_relative '../puzzle.rb'

class PartTwo < Puzzle
  def initialize
    super(
      example_output: 48,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-two')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run(instructions)
    control = true
    sum = 0
    instructions.each do |instruction|
      case instruction.first
      when :mul
        sum += instruction[1] * instruction[2] if control
      when :do
        control = true
      when :dont
        control = false
      end
    end
    sum
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
      when 'd'
        parse_control
      else
        consume
      end
    end
    @ast
  end

  def parse_control
    consume
    return if !current_token? 'o'
    consume

    if current_token == "n"
      consume
      return if !current_token? "'"
      consume
      return if !current_token? 't'
      consume
      return if !current_token? '('
      consume
      return if !current_token? ')'
      consume
      @ast << [:dont]
    else
      return if !current_token? '('
      consume
      return if !current_token? ')'
      consume
      @ast << [:do]
    end
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
    @ast << [:mul, first_operand.to_i, second_operand.to_i]
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
