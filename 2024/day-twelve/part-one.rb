require_relative '../puzzle.rb'

class PartOne < Puzzle
  def initialize
    super(
      example_output: 1930,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run(grid)
    visited = Hash.new(false)
    directions = [
      [0, -1], #up
      [1, 0], #right
      [0, 1], #bottom
      [-1, 0], #let
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

        # Check all neighbors
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

    price = 0
    regions.each do |region|
      area = region.size

      perimeter = 0
      region.each do |(x, y)|
        directions.each do |(dx, dy)|
          if !inbound.call([x + dx, y +dy])
            perimeter += 1
            next
          end

          perimeter += 1 if grid[x][y] != grid[x + dx][y + dy]
        end
      end

      price += area * perimeter

      puts price
    end
    
    price
  end

  def parse(input)
    input.split("\n").map do |line|
      line.chars
    end
  end
end

