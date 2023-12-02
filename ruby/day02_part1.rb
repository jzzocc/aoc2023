# frozen_string_literal: true

COLORS = {
  /(\d+) red/ => 12,
  /(\d+) green/ => 13,
  /(\d+) blue/ => 14
}.freeze
GAME_RE = /Game (\d+):/.freeze

games = File.read('input/day02.txt').split("\n")

result = games.map do |game|
  reveals = game.split(';')
  possible = reveals.all? { |r| COLORS.all? { |re, max| (r.match(re)&.captures&.first.to_i || 0) <= max } }
  possible ? game.match(GAME_RE).captures.first.to_i : 0
end
.reduce(:+)

p result
