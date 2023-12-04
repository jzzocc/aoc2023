# frozen_string_literal: true

cards = File.read('input/day04.txt').split("\n")
card_copies = Hash.new { |hash, key| hash[key] = 0 }

cards.each.with_index do |card, i|
  numbers = card.split(':').last
  winning_numbers, my_numbers = numbers.split('|').map { |s| s.scan(/\d+/).map(&:to_i) }
  intersection = winning_numbers.intersection(my_numbers).count
  card_copies[i] += 1
  (card_copies[i]).times { ((i + 1)..(i + intersection)).each { |c| card_copies[c] += 1 } }
end

(0...card_copies.count).each { |i| puts "#{i}: #{card_copies[i]}" }

p card_copies.values.reduce(:+)
