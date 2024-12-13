require 'rational'
require_relative '../puzzle.rb'

class PartOne < Puzzle
  def initialize
    super(
      example_output: 480,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run(parsed)
    parsed.filter_map do |a, b, prize|
      equalize(*a, *b, *prize)
    end
    .sum
  end

  def equalize(ax, ay, bx, by, px, py)
    determinant = ax * by - ay * bx
    return nil if determinant == 0 # Parallel or coincident lines
  
    x = Rational(px * by - py * bx, determinant)
    y = Rational(px - ax * x, bx) if bx != 0
    y ||= Rational(py - ay * x, by) # Fallback for bx == 0

    if x.denominator == 1 && y.denominator == 1 && x <= 100 && y <= 100
      return (3 * x + y).to_i
    end
    
    nil
  end

  def parse(input)
    input
      .split("\n")
      .filter{ |line| !line.empty? }
      .each_slice(3).map do |a, b, prize|
        a = a.strip.split(':').last.split(',').map { |x| x.split('+').last.to_i }
        b = b.strip.split(':').last.split(',').map { |x| x.split('+').last.to_i }
        prize = prize.strip.split(':').last.split(',').map { |x| x.split('=').last.to_i }
        [a, b, prize]
    end
  end
end
