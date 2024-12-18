require_relative '../puzzle.rb'

class PartTwo < Puzzle
  def initialize
    super(
      example_output: [1, 6],
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-two')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end


  def run(incoming_bytes)
    hi, lo = incoming_bytes.size, 0
    while lo < hi
      middle = ((lo + hi) / 2).abs

      if has_path(incoming_bytes[..middle])
        lo =  middle + 1
      else 
        hi = middle
      end
    end
    incoming_bytes[lo]
  end

  def has_path(fallen)
    start_node = Node.new(0, 0)
    goal_node = Node.new(dimension, dimension)

    open = [start_node]
    closed = []

    until open.empty?
      current = open.min_by(&:f)

      if current == goal_node
        return true
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

    false
  end

  def dimension = @solve ? 70 : 6

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
