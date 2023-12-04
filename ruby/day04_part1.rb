# frozen_string_literal: true

cards = File.read('input/day04.txt').split("\n")

result = cards.map do |card|
  numbers = card.split(':').last
  winning_numbers, my_numbers = numbers.split('|').map { |s| s.scan(/\d+/).map(&:to_i) }
  intersection = winning_numbers.intersection(my_numbers).count
  intersection.positive? ? 2.pow(intersection - 1) : 0
end
.reduce(:+)

p result
