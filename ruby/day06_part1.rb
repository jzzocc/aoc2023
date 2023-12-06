# frozen_string_literal: true

p File.read('input/day06.txt')
      .split("\n")
      .map { |l| l.scan(/\d+/).map(&:to_i) }
      .transpose
      .map { |t, d| (1...t).filter_map { |h| h if (t - h) * h > d }.count }
      .reduce(:*)
