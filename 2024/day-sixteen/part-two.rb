require_relative '../puzzle.rb'

class PartTwo < Puzzle
  def initialize
    super(
      example_output: 64,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-two')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run(maze)
    start = find_reindeer_start
    finish = find_end_tile
    result = dijkstra(start, finish)

    puts print_path(result[:paths].flatten(1))

    result[:paths].flatten(1).uniq.size 
  end

  def dijkstra(start, finish)
    is_valid = -> (r,c) do
      inbounds?([r,c]) && @maze[r][c] != '#'
    end
    heuristic = ->(r,c) do
      er, ec = finish
      (er - r).abs + (ec - c).abs
    end

    queue = [
      [
        0, start, [start], :right
      ]
    ]
    result = {
      score: Float::INFINITY,
      paths: []
    }
    visited = Hash.new { |h, k| h[k] = Float::INFINITY } # Tracks the minimum cost to a state

    until queue.empty? 
      queue.sort_by! { |gx, pos, path, dir| gx}
      gx, (x, y), path, prev_dir = queue.shift

      if [x, y] == finish
        if gx < result[:score]
          result[:score] = gx
          result[:paths] = [path]
        elsif gx == result[:score]
          result[:paths] << path
        end
        next
      end

      state = [x, y, prev_dir]
      next if gx > visited[state]
      visited[state] = gx


      directions.each do |current_dir, (dx, dy)|
        nx, ny = x + dx, y + dy

        if is_valid.call(nx, ny) && !path.include?([nx, ny])
          next_state = [nx, ny, current_dir]
          move_cost = 1
          turn_cost = prev_dir && current_dir != prev_dir ? 1000 : 0
          new_gx = gx + move_cost + turn_cost
          new_fx = new_gx
          queue << [new_gx, [nx, ny], path + [[nx, ny]], current_dir]
        end
      end
    end
    result
  end

  def cell(position)
    @maze[position[0]][position[1]]
  end

  def turn?(direction, new_direction)
    case direction
    when :up
      new_direction == :right || new_direction == :left
    when :down
      new_direction == :right || new_direction == :left
    when :right
      new_direction == :up || new_direction == :down
    when :left
      new_direction == :up || new_direction == :down
    end
  end

  def inbounds?(position)
    r, c = position 
    r >= 0 && c >= 0 && r < @maze.size && c < @maze[0].size
  end

  def find_end_tile
    @maze.each_with_index do |row, r|
      row.each_with_index do |cell, c|
        return [r, c] if cell == 'E'
      end
    end
  end

  def find_reindeer_start
    @maze.each_with_index do |row, r|
      row.each_with_index do |cell, c|
        return [r, c] if cell == 'S'
      end
    end
  end

  def print_path(path)
    copy = @maze.map(&:dup)
    path.each do |r, c|
      copy[r][c] = 'O'
    end
    print_grid(copy)
  end

  def print_grid(grid)
    grid.map do |row|
      row.join('')
    end.join("\n")
  end

  def directions
    @directions ||= {
        up:[-1, 0],
        down:[1, 0],
        right: [0, 1],
        left:[0, -1],
      }
  end

  def direction_char(direction)
    case direction
    when :up
      '^'
    when :down
      'v'
    when :right
      '>'
    when :left
      '<'
    end
  end

  def parse(input)
    @maze = input.split("\n").map do |line|
      line.chars
    end
  end
end
