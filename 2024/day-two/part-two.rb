require_relative '../puzzle.rb'

class PartTwo < Puzzle
  def initialize
    super(
      example_output: 4,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run(reports)
    reports.filter_map do |report| 
      safe = safe? report 
      if !safe 
        report.each_with_index do |_, i|
          if safe? report[0...i] + report[(i+1)..]
            safe = true
            break
          end
        end
      end
      safe
    end.count 
  end

  def safe?(report)
    return false if !incrementing_or_decrementing? report 
    safe = true
    report[0..-2].zip(report[1..]).each do |a, b|
      if !safe_range?(a, b)
        safe = false
        break
      end
    end
    safe
  end

  def safe_range?(a, b)
    diff = (a - b).abs
    return diff >= 1 && diff <= 3
  end

  def incrementing_or_decrementing?(report)
    report.dup.sort == report || report.dup.sort.reverse == report
  end

  def parse(input)
    input.lines.map(&:split).map do |line|
      line.map(&:to_i)
    end
  end
end
