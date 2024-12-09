require_relative '../puzzle.rb'

class PartOne < Puzzle
  def initialize
    super(
      example_output: 143,
      example_path: File.expand_path(File.join(File.dirname(__FILE__), './example-one')),
      input_path:  File.expand_path(File.join(File.dirname(__FILE__), './input')),
    )
  end

  def run(input)
    ordering_hash, pages_to_produce = input
    pages_to_produce.filter_map do |page_list|
      valid = true
      seen = []
      page_list.each do |page, index|
        ordering = ordering_hash[page]
        ordering.each do |order|
          if seen.include?(order)
            valid = false
            break
          end
        end if ordering
        seen << page
      end
      if valid
        middle_index = page_list.length / 2
        page_list[middle_index]
      end
    end.sum
  end

  def parse(input)
    page_ordering, pages_to_produce = input.split("\n\n")
    ordering_hash = {}
    page_ordering = page_ordering.split("\n").map do |line|
      rule_number, have_be_after = line.split('|')
      rule_number = rule_number.to_i
      if ordering_hash[rule_number]
        ordering_hash[rule_number] << have_be_after.to_i
      else
        ordering_hash[rule_number] = [have_be_after.to_i]
      end
    end
    pages_to_produce = pages_to_produce.split("\n").map do |line|
      line.split(',').map(&:to_i)
    end
    [ordering_hash, pages_to_produce]
  end
end
