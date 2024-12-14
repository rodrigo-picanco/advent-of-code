require_relative '../puzzle.rb'

class PartTwo < Puzzle
  def initialize
    super(
      example_output: :bypass,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-two')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )

    @cols, @lines = [101, 103] 
  end

  def print_grid(robots)
    grid = ""
    @lines.times do |x|
      line = ""
      @cols.times do |y|
       if robots.include? [x, y]
         line << "X"
        else
          line << "."
        end
      end
      line << "\n"
      grid << line
    end
    grid
  end

  def run(robots)
    return :bypass if !@solve 

    elapse = -> (robots, seconds = 1) {
      robots.map do |((px, py), (vx, vy))|
        x =  (px + vx * seconds) % @cols 
        y =  (py + vy * seconds) % @lines
        x = x + @cols % @cols if x.negative? 
        y = y + @lines % @lines if y.negative? 
        [x, y]
      end
    }

    cx, cy = @cols / 2, @lines / 2
    quadrant = -> ((x, y)) {
      case
      when x < cx && y < cy
        :top_left
      when x >= cx && y < cy
        :bottom_left
      when x < cx && y >= cy
        :top_right
      when x >= cx && y >= cy
        :bottom_right
      end
    }

    seconds = (1..100_000).find do |i|
      Set.new(elapse.call(robots, i)).size == robots.size
    end

    puts print_grid(elapse.call(robots, seconds))

    seconds
  end

  def parse(input)
    input.each_line.map do |line|
      p, v = line.split
      parse_info = ->(info) { info.split('=').last.split(',').map(&:to_i) }
      [parse_info.call(p), parse_info.call(v)]
    end
  end
end
