require_relative '../puzzle.rb'

class PartOne < Puzzle
  def initialize
    super(
      example_output: 10092,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run((grid, moves))
    directions = {
      '^' => [-1, 0],
      'v' => [1, 0],
      '<' => [0, -1],
      '>' => [0, 1],
    }

    positions = {
      robot: [],
      wall: [],
      box: [],
    }

    @cols = grid.size
    @rows = grid[0].size

    @rows.times do |r|
      @cols.times do |c|
        case grid[r][c]
        when '@'
          positions[:robot] << [r, c]
        when '#'
          positions[:wall] << [r, c]
        when  'O'
          positions[:box] << [r, c]
        end
      end
    end


    inbound = -> ((r, c)) {
      r >= 0 && r < grid.size && c >= 0 && c < grid[0].size
    }

    collision = -> ((r, c)) {
      positions[:box].include?([r, c]) || positions[:wall].include?([r, c]) || !inbound.call([r, c])
    }

    has_box = -> ((r, c)) {
      positions[:box].include?([r, c])
    }

    move_box = -> (initial_position, next_position) {
      positions[:box].delete(initial_position)
      positions[:box] << next_position
    }

    can_move = -> (next_position) {
      inbound.call(next_position) && !positions[:wall].include?(next_position)
    }

    move_to = -> (type, initial_position, next_position) {
      case type
        when :robot
          positions[:robot].delete(initial_position)
          positions[:robot] << next_position
        when :box
          move_box.call(initial_position, next_position)
      end
    }

    moves.each do |move|
      robot_position = positions[:robot].last
      r, c = robot_position
      dr, dc = directions[move]

      next_position = [r + dr, c + dc]

      next if !can_move.call(next_position)

      affected_boxes = []
      i = 1
      while has_box.call([r + i * dr, c + i * dc])
        affected_boxes << [r + i * dr, c + i * dc]
        i += 1
      end

      to_move = affected_boxes.reverse.map do |box|
        [:box, *box]
      end
      to_move << [:robot, *robot_position]

      to_move.each do |type, r, c|
        next_position = [r + dr, c + dc]
        position = [r, c]
        break if collision.call(next_position)
        move_to.call(type, position, next_position)
      end
    end

    positions[:box].map do |(r, c)|
      r * 100 + c
    end.sum
  end

  def gridify(positions)
    grid = []
    @rows.times do |r|
      @cols.times do |c|
        grid[r] ||= []
        if positions[:box].include?([r, c])
          grid[r][c] = 'O'
        elsif positions[:wall].include?([r, c])
          grid[r][c] = '#'
        elsif positions[:robot].include?([r, c])
          grid[r][c] = '@'
        else
          grid[r][c] = '.'
        end
      end
    end
    grid
  end

  def print_grid(positions)
    puts gridify(positions).map(&:join).join("\n")
  end


  def parse(input)
    grid, moves = input.split("\n\n")
    grid = grid.split("\n").map do |line|
      line.chars
    end
    moves = moves.split("\n").join.chars
    [grid, moves]
  end
end
