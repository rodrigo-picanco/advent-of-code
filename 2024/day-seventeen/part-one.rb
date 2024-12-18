require_relative '../puzzle.rb'

class PartOne < Puzzle
  def initialize
    super(
      example_output: "4,6,3,5,6,3,5,2,1,0",
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run((regs, prog))
    instruction_pointer = 0
    out = []
    until prog[instruction_pointer].nil?
      opcode = prog[instruction_pointer]
      operand = prog[instruction_pointer + 1] 
      case opcode
      when 0
        numerator = regs["A"]
        denominator=(2**combo(operand, regs))
        regs["A"] = (numerator / denominator).abs
      when 1
        regs["B"] = regs["B"] ^ operand
      when 2
        regs["B"] = combo(operand, regs) % 8
      when 3
        if regs["A"] > 0
          instruction_pointer = operand
          next
        end
      when 4
        regs["B"] = regs["B"] ^ regs["C"]
      when 5
        out << combo(operand, regs) % 8
      when 6
        numerator = regs["A"]
        denominator=(2**combo(operand, regs))
        regs["B"] = (numerator / denominator).abs
      when 7
        numerator = regs["A"]
        denominator=(2**combo(operand, regs))
        regs["C"] = (numerator / denominator).abs
      end
      instruction_pointer += 2
    end

    puts out.inspect
    puts regs.inspect

    out.join(",")
  end

  def combo(operand, regs)
    case operand
    when 0
      0
    when 1
      1
    when 2
      2
    when 3
      3
    when 4
      regs["A"]
    when 5
      regs["B"]
    when 6
      regs["C"]
    when 7
      raise "Invalid combo operand 7"
    end
  end

  def parse(input)
    regs, prog = input.split("\n\n")
    regs = regs.split("\n").map do |reg|
      name, value = reg.split(":")
      name = name.split(" ").last
      value = value.to_i
      [name, value]
    end.to_h
    prog = prog.split(":").last.strip.split(",").map(&:to_i)
    [regs, prog]
  end
end
