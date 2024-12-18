require_relative '../puzzle.rb'
require 'concurrent-ruby'

class PartTwo < Puzzle
  def initialize
    super(
      example_output: 117440,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-two')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run((regs, prog))

    pool = Concurrent::FixedThreadPool.new(1024)  # Adjust thread count for your CPU
    result = Concurrent::AtomicReference.new(nil)


    (1..Float::INFINITY).each_slice(1_000_000) do |range|
      puts "Looking in range #{range.first} to #{range.last} at #{Time.now}"
      range.each do |ovr|
        pool.post do
          if result.value.nil? && check(ovr, regs, prog)
            result.set(ovr)
            pool.shutdown
          end
        end
      end
      break if result.value
    end

    pool.wait_for_termination

    result.value
  end

  def check(ovr, regs, prog)
    interpret(ovr, rgs, prog) == prog
  end

  def interpret(regs, prog, ovr)
    regs["A"] = ovr
    instruction_pointer = 0
    out = []

    until prog[instruction_pointer].nil?
      opcode = prog[instruction_pointer]
      operand = prog[instruction_pointer + 1] 
      case opcode
      when 0
        shift = combo(operand, regs)
        regs["A"] = (regs["A"] >> shift).abs
      when 1
        regs["B"] ^= operand
      when 2
        regs["B"] = combo(operand, regs) & 7  
      when 3
        if regs["A"] > 0
          instruction_pointer = operand
          next
        end
      when 4
        regs["B"] ^= regs["C"]
      when 5
        out << (combo(operand, regs) & 7)
      when 6
        shift = combo(operand, regs)
        regs["B"] = (regs["A"] >> shift).abs
      when 7
    shift = combo(operand, regs)
    regs["C"] = (regs["A"] >> shift).abs
      end
      instruction_pointer += 2
    end

    out
  end

  def combo(operand, regs)
    [0, 1, 2, 3, regs["A"], regs["B"], regs["C"]][operand]
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
