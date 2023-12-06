# frozen_string_literal: true

almanac = File.read('input/day05.txt').split("\n\n")

seeds = almanac.shift.scan(/\d+/).map(&:to_i)
maps = almanac.map do |map|
  map.split("\n")
     .drop(1)
     .map { |l| l.scan(/\d+/).map(&:to_i) }
     .map do |line|
       range_length = line.pop
       line.map { |l| (l...(l + range_length)) }.reverse
     end
     .to_h
end

result = seeds.map do |seed|
  value = seed
  maps.each do |map|
    map.each do |k, v|
      if k.include?(value)
        value = v.min + (value - k.min)
        break
      end
    end
  end
  value
end

p result.min
