def convert_temp(temp, input_scale: "celsius", output_scale: "celsius")
  if input_scale == output_scale
    return temp
  end

  map = {
    "celsius" => {
      "kelvin" => Proc.new do |x| x + 273.15 end,
      "fahrenheit" => Proc.new do |x| x * 1.8 + 32.0 end
    },
    "fahrenheit" => {
      "celsius" => Proc.new do |x| (x - 32.0) / 1.8 end,
      "kelvin" => Proc.new do |x| (x + 459.67) * 5.0 / 9.0 end
    },
    "kelvin" => {
      "fahrenheit" => Proc.new do |x| x * 1.8 - 459.67 end,
      "celsius" => Proc.new do |x| x - 273.15 end
    }
  }

  map[input_scale][output_scale].call(temp)
end


# puts(convert_temp(0, input_scale: 'celsius', output_scale: 'kelvin'))
# puts(convert_temp(0, input_scale: 'celsius', output_scale: 'fahrenheit'))
# puts(convert_temp(0, input_scale: 'kelvin', output_scale: 'celsius'))
# puts(convert_temp(0, input_scale: 'kelvin', output_scale: 'fahrenheit'))
# puts(convert_temp(0, input_scale: 'fahrenheit', output_scale: 'celsius'))
# puts(convert_temp(0, input_scale: 'fahrenheit', output_scale: 'kelvin'))
