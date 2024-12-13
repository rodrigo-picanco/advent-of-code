require_relative '../puzzle.rb'

class PartTwo < Puzzle
  def initialize
    super(
      example_output: 1206,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-two')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

def run(grid)
    visited = Hash.new(false)
    directions = [
      [0, -1], # down
      [1, 0],  # left
      [0, 1],  # up
      [-1, 0], # right
    ]
    inbound = -> ((x, y)) {
      x >= 0 && x < grid.size && y >= 0 && y < grid.first.size
    }
    neighbours = ->(a, b) {
      (ax, ay) = a
      (bx, by) = b
      (ax - bx).abs <= 1 && (ay - by).abs <= 1
    }
    explore_region = ->(start) {
      stack = [start]
      region = []
      start_plant = grid[start[0]][start[1]]
      while !stack.empty?
        position = stack.pop
        x, y = position
        next if visited[position]
        visited[position] = true
        region << position
        directions.each do |(dx, dy)|
          neighbor = [x + dx, y + dy]
          if inbound.call(neighbor) &&
             !visited[neighbor] &&
             grid[neighbor[0]][neighbor[1]] == start_plant
            stack.push(neighbor)
          end
        end
      end
      region
    }

    plants = Hash.new { |hash, key| hash[key] = [] }

    regions = []
    grid.each_with_index do |_, x|
      grid.each_with_index do |_, y|
        position = [x, y]
        next if visited[position]
        region = explore_region.call(position)
        regions << region unless region.empty?
      end
    end

    regions.map do |plots|
      area = plots.size
      edges = Set.new
      plots.each do |pos|
        x, y = pos
        directions.each do |(dx, dy)|
          neighbour = [x + dx, y + dy]
          edge = [pos, neighbour]
          if !inbound.call(neighbour) || !plots.include?(neighbour)
            edges.add(edge)
          end
        end
      end

      sides = Set.new

      edges.each do |(pos, neighbour)|
        keep = true
        for dx, dy in [[1, 0], [0, 1]]
          px, py = pos
          nx, ny = neighbour

          other_pos = [px + dx, py + dy]
          other_neighbour = nx + dx, ny + dy

          if edges.include?([other_pos, other_neighbour])
            keep = false
            break
          end
        end

        sides.add([pos, neighbour]) if keep
      end

      area * sides.size
    end.sum
  end

  def parse(input)
    input.split("\n").map do |line|
      line.chars
    end
  end
end
