require_relative '../puzzle.rb'

class PartOne < Puzzle
  def initialize
    super(
      example_output: 2,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run(reports)
    reports.filter_map { |report| safe? report }.count 
  end

  def safe?(report)
    (report.dup.sort == report || report.dup.sort.reverse == report) &&
    report.each_cons(2).find{ |a, b| !(a - b).abs.between?(1, 3) }.nil?
  end

  def parse(input)
    input.lines.map(&:split).map do |line|
      line.map(&:to_i)
    end
  end
end
