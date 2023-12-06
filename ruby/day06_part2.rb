# frozen_string_literal: true

time, distance = File.read('input/day06.txt').split("\n").map { |l| l.scan(/\d+/).join.to_i }

p (1...time).filter_map { |hold| hold if (time - hold) * hold > distance }.count
