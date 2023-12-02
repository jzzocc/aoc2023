# frozen_string_literal: true

COLORS = %w[red green blue].freeze

result = File.read('input/day02.txt').split("\n").map do |game|
  reveals = game.split(';')
  COLORS.map { |c| reveals.map { |r| /(\d+) #{c}/.match(r)&.captures&.first.to_i || 0 }.max }
        .reduce(:*)
end
.reduce(:+)

p result
