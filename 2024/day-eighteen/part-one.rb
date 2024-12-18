require_relative '../puzzle.rb'

class PartOne < Puzzle
  def initialize
    super(
      example_output: 22,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end


  def run(incoming_bytes)
    fallen = incoming_bytes[...bytes]
    path = a_star([0,0], [dimension, dimension], fallen)
    puts grid(
      fallen, 
      path
    ).map(&:join).join("\n")
    path.size - 1
  end

  def a_star(start, goal, fallen)
    start_node = Node.new(start.first, start.last)
    goal_node = Node.new(goal.first, goal.last)

    open = [start_node]
    closed = []

    until open.empty?
      current = open.min_by(&:f)

      if current == goal_node
        return path(current)
      end

      open.delete(current)
      closed << current

      hood(current, fallen)
      .each do |neighbour|
        next if closed.include?(neighbour)
        tg = current.g + 1

        if open.include? neighbour
          if tg < neighbour.g
            neighbour.g = tg
            neighbour.parent = current
          end
        else
          neighbour.g = tg
          neighbour.h = heuristic(neighbour, goal_node)
          neighbour.parent = current
          open << neighbour
        end
      end
    end

    []
  end

  def dimension = @solve ? 70 : 6
  def bytes = @solve ? 1024 : 12


  def heuristic(node, goal)
    Math.sqrt((node.x - goal.x)**2 + (node.y - goal.y)**2)
  end

  def hood(node, fallen)
    [
      [0, 1],
      [1, 0],
      [0, -1],
      [-1, 0]
    ].filter_map do |dx, dy|
      x, y = node.x + dx, node.y + dy
      Node.new(x, y) if !fallen.include?([x, y]) && inbound(x, y)
    end
  end

  def inbound(x, y)
    x >= 0 && x <= dimension && y >= 0 && y <= dimension
  end

  def grid(fallen, path = []) 
    grid = Array.new(dimension)
    (0..dimension).each do |x|
      grid[x] ||= []
      (0..dimension).each do |y|
        if fallen.include? [x, y] 
          grid[x][y] = "#"
        elsif path.include? [x, y]
          grid[x][y] = 'O'
        else
          grid[x][y] = "."
        end
      end
    end
    grid
  end

  def path(node)
    path = []
    while node
      path << [node.x, node.y]
      node = node.parent
    end
    path.reverse
  end

  def parse(input)
    input.split("\n").map { |line| line.split(",").map(&:to_i).reverse }
  end
end

class Node
  attr_accessor :x, :y, :g, :h, :parent

  def initialize(x, y, g = 0, h = 0, parent = nil)
    @x = x
    @y = y
    @g = g
    @h = h
    @parent = parent
  end

  def f
    g + h
  end

  def ==(other)
    x == other.x && other.y == y
  end
end
